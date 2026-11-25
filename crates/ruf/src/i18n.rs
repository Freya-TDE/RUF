//! Mehrsprachigkeit (i18n) für Programme auf RUF.
//!
//! Texte liegen als TOML-Dateien je Sprache vor (`de.toml`, `en.toml` …) und
//! werden vom Programm eingebettet und angemeldet ([`register`]). Weitere
//! Sprachen lassen sich ohne Neukompilieren aus einem Ordner nachladen
//! ([`load_dir`]). Fehlt ein Text, gilt die Rückfallsprache [`FALLBACK`]
//! (Englisch), zuletzt der Schlüssel selbst – fehlende Übersetzungen fallen so
//! auf, ohne dass etwas abstürzt.
//!
//! Schlüssel sind verschachtelte Tabellen in Punktschreibweise
//! (`[menu] save = "…"` → `menu.save`), Platzhalter stehen in geschweiften
//! Klammern: `"Zeile {line}"`.
//!
//! *Multilingual texts for RUF programs: one TOML file per language, embedded
//! and registered by the program, English as fallback.*
//!
//! ```
//! use ruf::{i18n, tr};
//!
//! i18n::register(&[
//!     ("de", "[app]\nsave = \"Speichern\"\nline = \"Zeile {line}\""),
//!     ("en", "[app]\nsave = \"Save\"\nline = \"Line {line}\""),
//! ]);
//! i18n::set_language("de_DE.UTF-8");
//! assert_eq!(tr!("app.save"), "Speichern");
//! assert_eq!(tr!("app.line", line = 3), "Zeile 3");
//! i18n::set_language("fr");
//! assert_eq!(tr!("app.save"), "Save");
//! ```

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::sync::{OnceLock, RwLock};

use toml_edit::{DocumentMut, Item};

/// Sprache, auf die zurückgegriffen wird, wenn ein Text fehlt.
pub const FALLBACK: &str = "en";

struct Catalog {
    language: String,
    texts: HashMap<String, HashMap<String, String>>,
}

fn catalog() -> &'static RwLock<Catalog> {
    static CATALOG: OnceLock<RwLock<Catalog>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        RwLock::new(Catalog {
            language: FALLBACK.to_owned(),
            texts: HashMap::new(),
        })
    })
}

fn parse(source: &str) -> Result<HashMap<String, String>, String> {
    let doc = DocumentMut::from_str(source).map_err(|e| e.to_string())?;
    let mut out = HashMap::new();
    flatten("", doc.as_item(), &mut out);
    Ok(out)
}

fn flatten(prefix: &str, item: &Item, out: &mut HashMap<String, String>) {
    if let Some(table) = item.as_table_like() {
        for (key, child) in table.iter() {
            let path = if prefix.is_empty() {
                key.to_owned()
            } else {
                format!("{prefix}.{key}")
            };
            flatten(&path, child, out);
        }
    } else if let Some(text) = item.as_str() {
        out.insert(prefix.to_owned(), text.to_owned());
    }
}

/// Normalisiert Sprachangaben wie `de_DE.UTF-8` oder `de-AT` zu Kandidaten
/// (`de-de`, `de`) in absteigender Genauigkeit.
fn candidates(language: &str) -> Vec<String> {
    let base = language
        .split(['.', '@'])
        .next()
        .unwrap_or_default()
        .replace('_', "-")
        .to_lowercase();
    let mut out = vec![base.clone()];
    if let Some((primary, _)) = base.split_once('-') {
        out.push(primary.to_owned());
    }
    out
}

/// Meldet eingebettete Texte an: je Eintrag Sprache und TOML-Quelltext.
/// Mehrere Aufrufe ergänzen sich (gleiche Schlüssel: der spätere gilt).
///
/// # Panics
///
/// Bei fehlerhaftem TOML – eingebettete Texte gehören zum Programm, ein
/// Fehler darin ist ein Programmierfehler.
pub fn register(sources: &[(&str, &str)]) {
    let mut catalog = catalog().write().expect("i18n-Katalog vergiftet");
    for (lang, source) in sources {
        let texts = parse(source).unwrap_or_else(|e| panic!("Texte {lang}: {e}"));
        catalog
            .texts
            .entry(candidates(lang).remove(0))
            .or_default()
            .extend(texts);
    }
}

/// Stellt die Sprache ein und liefert die tatsächlich verwendete.
///
/// Unbekannte Sprachen fallen auf die Hauptsprache (`de-AT` → `de`) und
/// zuletzt auf [`FALLBACK`] zurück.
pub fn set_language(language: &str) -> String {
    let mut catalog = catalog().write().expect("i18n-Katalog vergiftet");
    let chosen = candidates(language)
        .into_iter()
        .find(|c| catalog.texts.contains_key(c))
        .unwrap_or_else(|| FALLBACK.to_owned());
    catalog.language = chosen.clone();
    chosen
}

/// Die aktuell eingestellte Sprache.
pub fn language() -> String {
    catalog()
        .read()
        .expect("i18n-Katalog vergiftet")
        .language
        .clone()
}

/// Die Sprache des Systems, z. B. `de-DE` oder `en_US.UTF-8` (`en`, wenn
/// nichts zu ermitteln ist): unter Linux aus `LC_ALL`, `LC_MESSAGES`, `LANG`;
/// unter Windows die Sprache des Benutzers; unter macOS die Einstellung des
/// Systems (vom Finder gestartete Programme haben kein `LANG`).
pub fn system_language() -> String {
    if let Some(lang) = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|v| std::env::var(v).ok())
        .filter(|s| !s.is_empty() && s != "C" && s != "POSIX")
    {
        return lang;
    }
    platform_language().unwrap_or_else(|| FALLBACK.to_owned())
}

#[cfg(windows)]
fn platform_language() -> Option<String> {
    use windows_sys::Win32::Globalization::GetUserDefaultLocaleName;
    let mut buffer = [0u16; 85];
    // SAFETY: Puffer mit der dokumentierten Höchstlänge (LOCALE_NAME_MAX_LENGTH).
    let len = unsafe { GetUserDefaultLocaleName(buffer.as_mut_ptr(), buffer.len() as i32) };
    (len > 1).then(|| String::from_utf16_lossy(&buffer[..len as usize - 1]))
}

#[cfg(target_os = "macos")]
fn platform_language() -> Option<String> {
    let out = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleLocale"])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    (out.status.success() && !text.is_empty()).then_some(text)
}

#[cfg(not(any(windows, target_os = "macos")))]
fn platform_language() -> Option<String> {
    None
}

/// Der Name der Sprache `language` in ihr selbst (`de` → „Deutsch“, Schlüssel
/// `language.name` ihrer Texte); ohne Eintrag das Kürzel.
pub fn name_of(language: &str) -> String {
    catalog()
        .read()
        .expect("i18n-Katalog vergiftet")
        .texts
        .get(language)
        .and_then(|t| t.get("language.name"))
        .cloned()
        .unwrap_or_else(|| language.to_owned())
}

/// Alle angemeldeten Sprachen (sortiert).
pub fn available() -> Vec<String> {
    let catalog = catalog().read().expect("i18n-Katalog vergiftet");
    let mut langs: Vec<String> = catalog.texts.keys().cloned().collect();
    langs.sort();
    langs
}

/// Lädt alle `*.toml`-Dateien aus `dir` als zusätzliche Sprachen. Vorhandene
/// Sprachen werden ergänzt bzw. Texte überschrieben. Liefert die geladenen
/// Sprachen; eine fehlerhafte Datei bricht mit Grund ab.
pub fn load_dir(dir: &Path) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut loaded = Vec::new();
    let mut catalog = catalog().write().expect("i18n-Katalog vergiftet");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let Some(lang) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let source =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let texts = parse(&source).map_err(|e| format!("{}: {e}", path.display()))?;
        let lang = candidates(lang).remove(0);
        catalog.texts.entry(lang.clone()).or_default().extend(texts);
        loaded.push(lang);
    }
    loaded.sort();
    Ok(loaded)
}

/// Übersetzt `key` in die aktuelle Sprache. Fehlt der Text, wird die
/// Rückfallsprache und zuletzt der Schlüssel selbst geliefert.
pub fn translate(key: &str) -> String {
    let catalog = catalog().read().expect("i18n-Katalog vergiftet");
    [catalog.language.as_str(), FALLBACK]
        .iter()
        .find_map(|lang| catalog.texts.get(*lang)?.get(key))
        .cloned()
        .unwrap_or_else(|| key.to_owned())
}

/// Wie [`translate`], ersetzt aber `{name}`-Platzhalter.
pub fn translate_with(key: &str, args: &[(&str, String)]) -> String {
    let mut text = translate(key);
    for (name, value) in args {
        text = text.replace(&format!("{{{name}}}"), value);
    }
    text
}

/// Ist `key` in der Rückfallsprache bekannt? (Prüfungen, Tests.)
pub fn has(key: &str) -> bool {
    catalog()
        .read()
        .expect("i18n-Katalog vergiftet")
        .texts
        .get(FALLBACK)
        .is_some_and(|t| t.contains_key(key))
}

/// Alle Schlüssel einer Sprache (sortiert; für Vollständigkeitstests).
pub fn keys(language: &str) -> Vec<String> {
    let catalog = catalog().read().expect("i18n-Katalog vergiftet");
    let mut keys: Vec<String> = catalog
        .texts
        .get(language)
        .map(|t| t.keys().cloned().collect())
        .unwrap_or_default();
    keys.sort();
    keys
}

/// Übersetzt einen Schlüssel: `tr!("menu.save")` oder mit Platzhaltern
/// `tr!("status.saved", path = p.display())`.
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::i18n::translate($key)
    };
    ($key:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::i18n::translate_with($key, &[$((stringify!($name), $value.to_string())),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidates_fallback_and_placeholders() {
        assert_eq!(candidates("de_DE.UTF-8"), ["de-de", "de"]);
        assert_eq!(candidates("de-AT"), ["de-at", "de"]);
        register(&[
            ("de", "[t]\na = \"A-de\"\nb = \"{n} Dateien\""),
            ("en", "[t]\na = \"A-en\"\nb = \"{n} files\"\nonly_en = \"x\""),
        ]);
        assert_eq!(set_language("de_AT.UTF-8"), "de");
        assert_eq!(translate("t.a"), "A-de");
        assert_eq!(translate_with("t.b", &[("n", "3".into())]), "3 Dateien");
        // Fehlt der Text, gilt Englisch, zuletzt der Schlüssel.
        assert_eq!(translate("t.only_en"), "x");
        assert_eq!(translate("t.unknown"), "t.unknown");
        assert!(has("t.a") && !has("t.unknown"));
        assert_eq!(set_language("fr_FR"), "en");
        assert_eq!(translate("t.a"), "A-en");
    }
}
