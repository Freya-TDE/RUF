//! Gemeinsame Eigenschafts-Helfer für Komponenten mit gleichartigen Eigenschaften.
//!
//! Jeder Helfer liefert Deskriptoren, liest per `get` und schreibt per `set`.
//! `set` gibt `None` zurück, wenn der Name nicht zu ihm gehört – dann ist der
//! nächste Helfer bzw. [`ruf::base::set`] an der Reihe.

/// Eigenschaft `text`.
pub(crate) mod text {
    use ruf::{PropertyDescriptor, PropertyType, PropertyValue, Widget};

    pub const TEXT: &str = "text";

    pub fn descriptor(description: &'static str) -> PropertyDescriptor {
        PropertyDescriptor::new(TEXT, PropertyType::Text, description)
    }

    pub fn get(widget: &Widget, name: &str) -> Option<PropertyValue> {
        (name == TEXT).then(|| widget.text().into())
    }
}

/// Eigenschaften `items` und `current_index` (Combobox, Liste).
pub(crate) mod items {
    use ruf::{PropertyDescriptor, PropertyError, PropertyType, PropertyValue, Widget, base};

    pub const ITEMS: &str = "items";
    pub const CURRENT_INDEX: &str = "current_index";

    pub fn descriptors() -> [PropertyDescriptor; 2] {
        [
            PropertyDescriptor::new(ITEMS, PropertyType::TextList, "Einträge"),
            PropertyDescriptor::new(
                CURRENT_INDEX,
                PropertyType::Int,
                "Ausgewählter Eintrag (-1 = keiner)",
            ),
        ]
    }

    pub fn get(widget: &Widget, name: &str) -> Option<PropertyValue> {
        match name {
            ITEMS => Some(widget.items().into()),
            CURRENT_INDEX => Some(widget.current_index().into()),
            _ => None,
        }
    }

    pub fn set(
        widget: &Widget,
        name: &str,
        value: &PropertyValue,
    ) -> Option<Result<(), PropertyError>> {
        let result = match name {
            ITEMS => base::expect_text_list(name, value).map(|v| widget.set_items(v)),
            CURRENT_INDEX => base::expect_int(name, value).map(|v| widget.set_current_index(v)),
            _ => return None,
        };
        Some(result)
    }
}

/// Eigenschaften `value`, `minimum`, `maximum` (Spinbox, Schieberegler, Fortschritt).
pub(crate) mod range {
    use ruf::{PropertyDescriptor, PropertyError, PropertyType, PropertyValue, Widget, base};

    pub const VALUE: &str = "value";
    pub const MINIMUM: &str = "minimum";
    pub const MAXIMUM: &str = "maximum";

    pub fn descriptors() -> [PropertyDescriptor; 3] {
        [
            PropertyDescriptor::new(VALUE, PropertyType::Int, "Aktueller Wert"),
            PropertyDescriptor::new(MINIMUM, PropertyType::Int, "Kleinster Wert"),
            PropertyDescriptor::new(MAXIMUM, PropertyType::Int, "Größter Wert"),
        ]
    }

    pub fn get(widget: &Widget, name: &str) -> Option<PropertyValue> {
        match name {
            VALUE => Some(widget.value().into()),
            MINIMUM => Some(widget.minimum().into()),
            MAXIMUM => Some(widget.maximum().into()),
            _ => None,
        }
    }

    pub fn set(
        widget: &Widget,
        name: &str,
        value: &PropertyValue,
    ) -> Option<Result<(), PropertyError>> {
        let result = match name {
            VALUE => base::expect_int(name, value).map(|v| widget.set_value(v)),
            MINIMUM => base::expect_int(name, value).map(|v| widget.set_range(v, widget.maximum())),
            MAXIMUM => base::expect_int(name, value).map(|v| widget.set_range(widget.minimum(), v)),
            _ => return None,
        };
        Some(result)
    }
}

/// Eigenschaft `vertical` (Schieberegler, Fortschritt).
pub(crate) mod orientation {
    use ruf::{PropertyDescriptor, PropertyError, PropertyType, PropertyValue, Widget, base};

    pub const VERTICAL: &str = "vertical";

    pub fn descriptor() -> PropertyDescriptor {
        PropertyDescriptor::new(VERTICAL, PropertyType::Bool, "Senkrechte Ausrichtung")
    }

    pub fn get(widget: &Widget, name: &str) -> Option<PropertyValue> {
        (name == VERTICAL).then(|| widget.is_vertical().into())
    }

    pub fn set(
        widget: &Widget,
        name: &str,
        value: &PropertyValue,
    ) -> Option<Result<(), PropertyError>> {
        (name == VERTICAL).then(|| base::expect_bool(name, value).map(|v| widget.set_vertical(v)))
    }
}

/// Einpassung, Gamma und Deckkraft von Bildern (ImageBox, ImageScrollBox).
/// Gamma und Deckkraft sind ganze Zahlen: Gamma in Hundertsteln (100 = 1,0;
/// 10…500), Deckkraft in Prozent (0…100).
pub(crate) mod picture {
    use ruf::{ImageFit, PropertyDescriptor, PropertyError, PropertyType, PropertyValue, base};

    pub const FIT: &str = "fit";
    pub const GAMMA: &str = "gamma";
    pub const OPACITY: &str = "opacity";
    pub const FITS: [&str; 4] = ["Contain", "Cover", "Stretch", "Center"];

    /// Gamma und Deckkraft eines Bild-Widgets.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Tone {
        pub gamma: f32,
        pub opacity: f32,
    }

    impl Tone {
        /// `image` mit diesem Ton; ohne Gamma und Deckkraft unverändert (ein
        /// schon getöntes Bild aus `set_image` behält seinen Ton).
        pub fn apply(&self, image: &ruf::Image) -> ruf::Image {
            if *self == Self::default() {
                image.clone()
            } else {
                image.with_tone(self.gamma, self.opacity)
            }
        }
    }

    impl Default for Tone {
        fn default() -> Self {
            Self {
                gamma: 1.0,
                opacity: 1.0,
            }
        }
    }

    pub fn descriptors() -> [PropertyDescriptor; 3] {
        [
            PropertyDescriptor::new(
                FIT,
                PropertyType::Text,
                "Einpassung: Contain, Cover, Stretch oder Center",
            ),
            PropertyDescriptor::new(
                GAMMA,
                PropertyType::Int,
                "Gamma in Hundertsteln (100 = unverändert, darüber heller)",
            ),
            PropertyDescriptor::new(OPACITY, PropertyType::Int, "Deckkraft in Prozent"),
        ]
    }

    pub fn fit_name(fit: ImageFit) -> &'static str {
        match fit {
            ImageFit::Contain => "Contain",
            ImageFit::Cover => "Cover",
            ImageFit::Stretch => "Stretch",
            ImageFit::Center => "Center",
        }
    }

    pub fn parse_fit(name: &str, value: &PropertyValue) -> Result<ImageFit, PropertyError> {
        match base::expect_text(name, value)? {
            "Contain" => Ok(ImageFit::Contain),
            "Cover" => Ok(ImageFit::Cover),
            "Stretch" => Ok(ImageFit::Stretch),
            "Center" => Ok(ImageFit::Center),
            other => Err(PropertyError::invalid_value(name, other, &FITS)),
        }
    }

    pub fn get(fit: ImageFit, tone: Tone, name: &str) -> Option<PropertyValue> {
        match name {
            FIT => Some(fit_name(fit).to_owned().into()),
            GAMMA => Some(((tone.gamma * 100.0).round() as i32).into()),
            OPACITY => Some(((tone.opacity * 100.0).round() as i32).into()),
            _ => None,
        }
    }

    /// Was `name` ändert; `None`, wenn der Name nicht hierher gehört.
    pub enum Change {
        Fit(ImageFit),
        Gamma(f32),
        Opacity(f32),
    }

    pub fn parse(name: &str, value: &PropertyValue) -> Option<Result<Change, PropertyError>> {
        Some(match name {
            FIT => parse_fit(name, value).map(Change::Fit),
            GAMMA => base::expect_int(name, value)
                .map(|v| Change::Gamma(v.clamp(10, 500) as f32 / 100.0)),
            OPACITY => base::expect_int(name, value)
                .map(|v| Change::Opacity(v.clamp(0, 100) as f32 / 100.0)),
            _ => return None,
        })
    }
}
