# rncl – Rust native Component Library

**English:** ready-made GTK 4 components for [`ruf`](https://crates.io/crates/ruf): window, panel, button,
label, text field, check box, radio button, group box, text area, combo box, list, spin box, slider,
progress bar, tabs, scroll area, canvas, tree view and a code editor (GtkSourceView) – the same code on
Linux, Windows and macOS.

**Deutsch:** fertige GTK-4-Komponenten für [`ruf`](https://crates.io/crates/ruf): Fenster, Panel, Knopf,
Beschriftung, Textfeld, Checkbox, Optionsfeld, Gruppenrahmen, Textbereich, Auswahlliste, Liste, Zahlenfeld,
Schieberegler, Fortschrittsanzeige, Registerkarten, Scrollbereich, Zeichenfläche, Baumansicht und ein
Code-Editor (GtkSourceView) – derselbe Code auf Linux, Windows und macOS.

```toml
[dependencies]
ruf = "0.2"
rncl = "0.2"
```

```rust,no_run
use rncl::prelude::*;

fn main() {
    let app = Application::new();
    let window = RnclWindow::new("Hello RUF");
    window.set_size(320, 140);

    let button = RnclButton::new(&window, "Click me");
    button.set_position(20, 50);
    let b = button.clone();
    button.on_clicked(move || b.set_text("Thanks!"));

    window.show();
    std::process::exit(app.exec());
}
```

Requirements / Voraussetzungen: GTK 4 (≥ 4.12) and GtkSourceView 5 with their development files.

- Documentation / Dokumentation: <https://docs.rs/rncl>, [manual (EN)](https://github.com/Freya-TDE/RUF/blob/main/docs/en/README.md), [Handbuch (DE)](https://github.com/Freya-TDE/RUF/blob/main/docs/de/README.md)
- Repository: <https://github.com/Freya-TDE/RUF>

Licensed under either of Apache License 2.0 or MIT, at your option. / Nach Wahl lizenziert unter Apache 2.0
oder MIT. © Dragon Software
