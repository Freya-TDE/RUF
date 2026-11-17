use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use ruf::{
    AsWidget, Component, ComponentInfo, Gallery, GalleryLayout, Image, ImageFit,
    PropertyDescriptor, PropertyError, PropertyType, PropertyValue, Size, Widget, WidgetKind, base,
};

use crate::props::picture::{self, Change, Tone};

const IMAGES: &str = "images";
const LAYOUT: &str = "layout";
const THUMB_WIDTH: &str = "thumb_width";
const THUMB_HEIGHT: &str = "thumb_height";
const SPACING: &str = "spacing";
const CURRENT_INDEX: &str = "current_index";
const LAYOUTS: [&str; 3] = ["Horizontal", "Vertical", "Grid"];

#[derive(Debug, Default)]
struct State {
    files: Vec<String>,
    /// Die Bilder ohne Gamma und Deckkraft.
    sources: Vec<Image>,
    tone: Tone,
}

/// Bildergalerie (`GtkFlowBox` in einem Scrollbereich) als Fortsetzung der
/// [`RnclImageBox`](crate::RnclImageBox): eine Zeile, eine Spalte oder ein
/// Raster aus Vorschaubildern; ein Klick wählt ein Bild.
///
/// *A picture gallery (row, column or grid); a click selects a picture.*
///
/// ```no_run
/// use rncl::prelude::*;
///
/// let app = Application::new();
/// let window = RnclWindow::new("Galerie");
/// let gallery = RnclImageScrollBox::new(&window);
/// gallery.set_geometry(10, 10, 320, 200);
/// gallery.set_thumb_size(64, 64);
/// gallery.set_files(&["a.png", "b.png", "c.png"]);
/// let g = gallery.clone();
/// gallery.on_current_index_changed(move |index| {
///     println!("{index}: {:?}", g.current_file());
/// });
/// window.show();
/// std::process::exit(app.exec());
/// ```
#[derive(Debug, Clone)]
pub struct RnclImageScrollBox {
    widget: Widget,
    state: Rc<RefCell<State>>,
}

impl RnclImageScrollBox {
    pub const TYPE_NAME: &'static str = "RnclImageScrollBox";

    /// Erzeugt eine leere Galerie (Standardgröße 240×128, Raster, Vorschau
    /// 96×96, Abstand 8).
    pub fn new<P: AsWidget + ?Sized>(parent: &P) -> Self {
        Self::with_parent(Some(parent.as_widget()))
    }

    /// Wie `new`, aber mit optionalem Elternteil (`None` = eigenständiges Fenster).
    pub fn with_parent(parent: Option<&Widget>) -> Self {
        let widget = Widget::create(WidgetKind::ImageGallery, parent);
        widget.set_size(240, 128);
        widget.set_gallery(Gallery::default());
        widget.set_image_fit(ImageFit::Cover);
        Self {
            widget,
            state: Rc::default(),
        }
    }

    /// Lädt die Bilder aus Dateien. Eine Datei, die sich nicht lesen lässt,
    /// bleibt als leerer Platz stehen (die Indizes passen weiter zu den
    /// Pfaden); geliefert werden diese Pfade.
    pub fn set_files<P: AsRef<Path>>(&self, paths: &[P]) -> Vec<String> {
        let mut failed = Vec::new();
        let mut files = Vec::new();
        let mut sources = Vec::new();
        for path in paths {
            let path = path.as_ref();
            let name = path.display().to_string();
            match std::fs::read(path) {
                Ok(bytes) => sources.push(Image::from_bytes(bytes)),
                Err(_) => {
                    failed.push(name.clone());
                    sources.push(Image::from_bytes(Vec::new()));
                }
            }
            files.push(name);
        }
        {
            let mut state = self.state.borrow_mut();
            state.files = files;
            state.sources = sources;
        }
        self.apply();
        failed
    }

    /// Pfade aus [`set_files`](Self::set_files).
    pub fn files(&self) -> Vec<String> {
        self.state.borrow().files.clone()
    }

    /// Setzt die Bilder direkt (ohne Pfade).
    pub fn set_images(&self, images: &[Image]) {
        {
            let mut state = self.state.borrow_mut();
            state.files.clear();
            state.sources = images.to_vec();
        }
        self.apply();
    }

    pub fn count(&self) -> i32 {
        self.state.borrow().sources.len() as i32
    }

    pub fn set_layout(&self, layout: GalleryLayout) {
        let gallery = Gallery {
            layout,
            ..self.widget.gallery()
        };
        self.widget.set_gallery(gallery);
    }

    pub fn layout(&self) -> GalleryLayout {
        self.widget.gallery().layout
    }

    /// Größe jedes Vorschaubilds in Pixeln.
    pub fn set_thumb_size(&self, width: i32, height: i32) {
        let gallery = Gallery {
            thumb: Size::new(width.max(1), height.max(1)),
            ..self.widget.gallery()
        };
        self.widget.set_gallery(gallery);
    }

    pub fn thumb_size(&self) -> Size {
        self.widget.gallery().thumb
    }

    /// Abstand zwischen den Bildern in Pixeln.
    pub fn set_spacing(&self, spacing: i32) {
        let gallery = Gallery {
            spacing: spacing.max(0),
            ..self.widget.gallery()
        };
        self.widget.set_gallery(gallery);
    }

    pub fn spacing(&self) -> i32 {
        self.widget.gallery().spacing
    }

    pub fn set_fit(&self, fit: ImageFit) {
        self.widget.set_image_fit(fit);
    }

    pub fn fit(&self) -> ImageFit {
        self.widget.image_fit()
    }

    /// Gamma aller Bilder: 1 = unverändert, darüber heller (0,1…5).
    pub fn set_gamma(&self, gamma: f32) {
        self.state.borrow_mut().tone.gamma = gamma.clamp(0.1, 5.0);
        self.apply();
    }

    pub fn gamma(&self) -> f32 {
        self.state.borrow().tone.gamma
    }

    /// Deckkraft aller Bilder: 0 = unsichtbar, 1 = deckend.
    pub fn set_opacity(&self, opacity: f32) {
        self.state.borrow_mut().tone.opacity = opacity.clamp(0.0, 1.0);
        self.apply();
    }

    pub fn opacity(&self) -> f32 {
        self.state.borrow().tone.opacity
    }

    /// Wählt ein Bild aus; `-1` hebt die Auswahl auf.
    pub fn set_current_index(&self, index: i32) {
        self.widget.set_current_index(index);
    }

    pub fn current_index(&self) -> i32 {
        self.widget.current_index()
    }

    /// Pfad des gewählten Bilds (`None` ohne Auswahl oder ohne Pfade).
    pub fn current_file(&self) -> Option<String> {
        let index = usize::try_from(self.current_index()).ok()?;
        self.state.borrow().files.get(index).cloned()
    }

    /// Wird mit dem neu gewählten Bild aufgerufen (`-1` = keine Auswahl).
    pub fn on_current_index_changed(&self, f: impl FnMut(i32) + 'static) {
        self.widget.on_current_index_changed(f);
    }

    fn change(&self, change: Change) {
        match change {
            Change::Fit(fit) => self.set_fit(fit),
            Change::Gamma(gamma) => self.set_gamma(gamma),
            Change::Opacity(opacity) => self.set_opacity(opacity),
        }
    }

    fn apply(&self) {
        let state = self.state.borrow();
        let images: Vec<Image> = state.sources.iter().map(|i| state.tone.apply(i)).collect();
        self.widget.set_images(&images);
    }

    pub fn info() -> ComponentInfo {
        ComponentInfo::new(Self::TYPE_NAME, |parent| {
            Box::new(Self::with_parent(parent))
        })
        .category("Anzeige")
        .description("Bildergalerie: Zeile, Spalte oder Raster, ein Bild auswählbar")
    }
}

fn layout_name(layout: GalleryLayout) -> &'static str {
    match layout {
        GalleryLayout::Horizontal => "Horizontal",
        GalleryLayout::Vertical => "Vertical",
        GalleryLayout::Grid => "Grid",
    }
}

impl Component for RnclImageScrollBox {
    fn widget(&self) -> &Widget {
        &self.widget
    }

    fn type_name(&self) -> &'static str {
        Self::TYPE_NAME
    }

    fn properties(&self) -> Vec<PropertyDescriptor> {
        let mut props = base::descriptors();
        props.extend([
            PropertyDescriptor::new(IMAGES, PropertyType::TextList, "Bilddateien"),
            PropertyDescriptor::new(
                LAYOUT,
                PropertyType::Text,
                "Anordnung: Horizontal, Vertical oder Grid",
            ),
            PropertyDescriptor::new(THUMB_WIDTH, PropertyType::Int, "Breite der Vorschaubilder"),
            PropertyDescriptor::new(THUMB_HEIGHT, PropertyType::Int, "Höhe der Vorschaubilder"),
            PropertyDescriptor::new(SPACING, PropertyType::Int, "Abstand zwischen den Bildern"),
            PropertyDescriptor::new(
                CURRENT_INDEX,
                PropertyType::Int,
                "Ausgewähltes Bild (-1 = keins)",
            ),
        ]);
        props.extend(picture::descriptors());
        props
    }

    fn property(&self, name: &str) -> Option<PropertyValue> {
        let tone = self.state.borrow().tone;
        match name {
            IMAGES => Some(self.files().into()),
            LAYOUT => Some(layout_name(self.layout()).to_owned().into()),
            THUMB_WIDTH => Some(self.thumb_size().width.into()),
            THUMB_HEIGHT => Some(self.thumb_size().height.into()),
            SPACING => Some(self.spacing().into()),
            CURRENT_INDEX => Some(self.current_index().into()),
            _ => picture::get(self.fit(), tone, name).or_else(|| base::get(&self.widget, name)),
        }
    }

    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), PropertyError> {
        match name {
            IMAGES => {
                let files = base::expect_text_list(name, &value)?.to_vec();
                // Fehlende Dateien bleiben leere Plätze.
                let _ = self.set_files(&files);
                return Ok(());
            }
            LAYOUT => {
                let layout = match base::expect_text(name, &value)? {
                    "Horizontal" => GalleryLayout::Horizontal,
                    "Vertical" => GalleryLayout::Vertical,
                    "Grid" => GalleryLayout::Grid,
                    other => return Err(PropertyError::invalid_value(name, other, &LAYOUTS)),
                };
                self.set_layout(layout);
                return Ok(());
            }
            THUMB_WIDTH => {
                let width = base::expect_int(name, &value)?;
                self.set_thumb_size(width, self.thumb_size().height);
                return Ok(());
            }
            THUMB_HEIGHT => {
                let height = base::expect_int(name, &value)?;
                self.set_thumb_size(self.thumb_size().width, height);
                return Ok(());
            }
            SPACING => {
                self.set_spacing(base::expect_int(name, &value)?);
                return Ok(());
            }
            CURRENT_INDEX => {
                self.set_current_index(base::expect_int(name, &value)?);
                return Ok(());
            }
            _ => {}
        }
        match picture::parse(name, &value) {
            Some(change) => change.map(|c| self.change(c)),
            None => base::set(&self.widget, name, value),
        }
    }
}
