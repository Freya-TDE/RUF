# ruf – Rust native UI Framework

**English:** a desktop UI framework for Rust based on **GTK 4** – the same code on Linux, Windows and macOS.
RUF is the system layer: the GTK 4 backend behind a trait, a platform-independent core, `Widget`,
`Component`, `ComponentRegistry` and the property model. Ready-made components are in
[`rncl`](https://crates.io/crates/rncl). RUF is the foundation of the RAD tool
[Freya TDE](https://github.com/Freya-TDE/Freya-TDE).

**Deutsch:** ein Desktop-UI-Framework für Rust auf Basis von **GTK 4** – derselbe Code auf Linux, Windows
und macOS. RUF ist die System-Ebene: GTK-4-Backend hinter einem Trait, plattformunabhängiger Kern, `Widget`,
`Component`, `ComponentRegistry` und das Eigenschaftsmodell. Fertige Komponenten liefert
[`rncl`](https://crates.io/crates/rncl). RUF ist das Fundament des RAD-Tools
[Freya TDE](https://github.com/Freya-TDE/Freya-TDE).

```toml
[dependencies]
ruf = "0.2"
rncl = "0.2"
```

Requirements / Voraussetzungen: GTK 4 (≥ 4.12) and GtkSourceView 5 with their development files
(e.g. `libgtk-4-dev libgtksourceview-5-dev` on Debian/Ubuntu, `gtk4 gtksourceview5` on Arch and Homebrew,
MSYS2 UCRT64 on Windows).

- Documentation / Dokumentation: <https://docs.rs/ruf>, [manual (EN)](https://github.com/Freya-TDE/RUF/blob/main/docs/en/README.md), [Handbuch (DE)](https://github.com/Freya-TDE/RUF/blob/main/docs/de/README.md)
- Repository: <https://github.com/Freya-TDE/RUF>

Licensed under either of Apache License 2.0 or MIT, at your option. / Nach Wahl lizenziert unter Apache 2.0
oder MIT. © Dragon Software
