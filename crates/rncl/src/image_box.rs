use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use ruf::{
    AsWidget, Component, ComponentInfo, Image, ImageFit, PropertyDescriptor, PropertyError,
    PropertyType, PropertyValue, Widget, WidgetKind, base,
};

use crate::props::picture::{self, Change, Tone};

const FILE: &str = "file";

#[derive(Debug, Default)]
struct State {
    file: String,
    /// Das Bild ohne Gamma und Deckkraft.
    source: Option<Image>,
    tone: Tone,
}

/// Ein Bild (`GtkPicture`): aus einer Datei oder aus Bytes, mit Einpassung,
/// Gamma und Deckkraft; meldet Klicks.
///
/// *A picture with fit, gamma and opacity; reports clicks.*
///
/// ```no_run
/// use rncl::prelude::*;
///
/// let app = Application::new();
/// let window = RnclWindow::new("Bild");
/// let photo = RnclImageBox::new(&window);
/// photo.set_geometry(10, 10, 200, 150);
/// photo.set_fit(ImageFit::Cover);
/// photo.set_opacity(0.8);
/// photo.set_file("photo.png").ok();
/// photo.on_clicked(|| println!("Bild angeklickt"));
/// window.show();
/// std::process::exit(app.exec());
/// ```
#[derive(Debug, Clone)]
pub struct RnclImageBox {
    widget: Widget,
    state: Rc<RefCell<State>>,
}

impl RnclImageBox {
    pub const TYPE_NAME: &'static str = "RnclImageBox";

    /// Erzeugt ein leeres Bild (Standardgröße 96×96).
    pub fn new<P: AsWidget + ?Sized>(parent: &P) -> Self {
        Self::with_parent(Some(parent.as_widget()))
    }

    /// Wie `new`, aber mit optionalem Elternteil (`None` = eigenständiges Fenster).
    pub fn with_parent(parent: Option<&Widget>) -> Self {
        let widget = Widget::create(WidgetKind::Image, parent);
        widget.set_size(96, 96);
        Self {
            widget,
            state: Rc::default(),
        }
    }

    /// Lädt das Bild aus einer Datei (PNG, JPEG, GIF, WebP …, je nach den
    /// Bildladern des Systems). Bei einem Fehler bleibt das Bild leer.
    pub fn set_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let path = path.as_ref();
        let result = std::fs::read(path).map(Image::from_bytes);
        {
            let mut state = self.state.borrow_mut();
            state.file = path.display().to_string();
            state.source = result.as_ref().ok().cloned();
        }
        self.apply();
        result.map(drop)
    }

    /// Pfad aus [`set_file`](Self::set_file) (leer bei [`set_image`](Self::set_image)).
    pub fn file(&self) -> String {
        self.state.borrow().file.clone()
    }

    /// Setzt das Bild direkt (z. B. `include_bytes!`); `None` leert es.
    pub fn set_image(&self, image: Option<&Image>) {
        {
            let mut state = self.state.borrow_mut();
            state.file.clear();
            state.source = image.cloned();
        }
        self.apply();
    }

    /// Das Bild ohne Gamma und Deckkraft.
    pub fn image(&self) -> Option<Image> {
        self.state.borrow().source.clone()
    }

    pub fn set_fit(&self, fit: ImageFit) {
        self.widget.set_image_fit(fit);
    }

    pub fn fit(&self) -> ImageFit {
        self.widget.image_fit()
    }

    /// Gamma: 1 = unverändert, darüber heller, darunter dunkler (0,1…5).
    pub fn set_gamma(&self, gamma: f32) {
        self.state.borrow_mut().tone.gamma = gamma.clamp(0.1, 5.0);
        self.apply();
    }

    pub fn gamma(&self) -> f32 {
        self.state.borrow().tone.gamma
    }

    /// Deckkraft: 0 = unsichtbar, 1 = deckend.
    pub fn set_opacity(&self, opacity: f32) {
        self.state.borrow_mut().tone.opacity = opacity.clamp(0.0, 1.0);
        self.apply();
    }

    pub fn opacity(&self) -> f32 {
        self.state.borrow().tone.opacity
    }

    /// Wird bei einem Klick auf das Bild aufgerufen.
    pub fn on_clicked(&self, f: impl FnMut() + 'static) {
        self.widget.on_clicked(f);
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
        let image = state.source.as_ref().map(|i| state.tone.apply(i));
        self.widget.set_image(image.as_ref());
    }

    pub fn info() -> ComponentInfo {
        ComponentInfo::new(Self::TYPE_NAME, |parent| {
            Box::new(Self::with_parent(parent))
        })
        .category("Anzeige")
        .description("Bild mit Einpassung, Gamma und Deckkraft")
    }
}

impl Component for RnclImageBox {
    fn widget(&self) -> &Widget {
        &self.widget
    }

    fn type_name(&self) -> &'static str {
        Self::TYPE_NAME
    }

    fn properties(&self) -> Vec<PropertyDescriptor> {
        let mut props = base::descriptors();
        props.push(PropertyDescriptor::new(
            FILE,
            PropertyType::Text,
            "Bilddatei (leer = kein Bild)",
        ));
        props.extend(picture::descriptors());
        props
    }

    fn property(&self, name: &str) -> Option<PropertyValue> {
        if name == FILE {
            return Some(self.file().into());
        }
        let tone = self.state.borrow().tone;
        picture::get(self.fit(), tone, name).or_else(|| base::get(&self.widget, name))
    }

    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), PropertyError> {
        if name == FILE {
            let path = base::expect_text(name, &value)?;
            if path.is_empty() {
                self.set_image(None);
            } else {
                // Eine fehlende Datei lässt das Bild leer; der Pfad bleibt stehen.
                let _ = self.set_file(path);
            }
            return Ok(());
        }
        match picture::parse(name, &value) {
            Some(change) => change.map(|c| self.change(c)),
            None => base::set(&self.widget, name, value),
        }
    }
}
