# RUF – Rust native UI Framework

**Deutsch:** Desktop-UI-Framework für Rust auf Basis von **GTK 4** – derselbe Code auf Linux, Windows und
macOS. Releases für Windows und macOS liefern die GTK-Laufzeit mit (`tools/`, `docs/de/release.md`). RUF ist
das Fundament des RAD-Tools [Freya TDE](https://github.com/Freya-TDE/Freya-TDE) und des
[CT-Editors](https://github.com/Freya-TDE/CT-E).

**English:** A desktop UI framework for Rust based on **GTK 4** – the same code on Linux, Windows and macOS.
Releases for Windows and macOS ship the GTK runtime (`tools/`, `docs/en/release.md`). RUF is the foundation
of the RAD tool [Freya TDE](https://github.com/Freya-TDE/Freya-TDE) and of the
[CT-Editor](https://github.com/Freya-TDE/CT-E).

> **Mit Claude Code gebaut / Built with Claude Code** – RUF wurde vollständig mit
> [Claude Code](https://claude.com/claude-code) (Modell Claude Opus 5.5) umgesetzt. / RUF was built entirely
> with [Claude Code](https://claude.com/claude-code) (model Claude Opus 5.5).

## Crates

| Crate | Deutsch | English |
| --- | --- | --- |
| [`ruf`](crates/ruf) | System-Ebene (GTK-4-Backend hinter dem Trait `Backend`), plattformunabhängiger Kern, `Widget`, `Component`, `ComponentRegistry`, Eigenschaftsmodell, Dialoge, Code-Editor-Funktionen | system layer (GTK 4 backend behind the `Backend` trait), platform-independent core, `Widget`, `Component`, `ComponentRegistry`, property model, dialogs, code editor functions |
| [`rncl`](crates/rncl) | Rust native Component Library: Fenster, Container, Registerkarten, Scrollbereich, Knöpfe, Text-, Listen-, Baum- und Zahleneingaben, Code-Editor, Zeichenfläche, Fortschrittsanzeige | Rust native Component Library: window, container, tabs, scroll area, buttons, text, list, tree and number inputs, code editor, canvas, progress bar |

Einbinden, solange RUF nicht auf crates.io steht / Until RUF is on crates.io:

```toml
[dependencies]
ruf  = { git = "https://github.com/Freya-TDE/RUF" }   # das Fundament / the foundation
rncl = { git = "https://github.com/Freya-TDE/RUF" }   # die Komponenten / the components
```

## Beispiel / Example

```rust
use rncl::prelude::*;

fn main() {
    let app = Application::new();
    let window = RnclWindow::new("Hallo RUF / Hello RUF");
    window.set_size(320, 140);

    let button = RnclButton::new(&window, "Klick mich / Click me");
    button.set_position(20, 50);
    let b = button.clone();
    button.on_clicked(move || b.set_text("Danke! / Thanks!"));

    window.show();
    std::process::exit(app.exec());
}
```

```bash
cargo test --workspace                 # Tests gegen das echte Toolkit / tests against the real toolkit
                                       # (Linux ohne Bildschirm / without a display: xvfb-run -a …)
cargo run -p rncl --example hello      # weitere / more: counter, form, gallery, custom_component, designer
```

## Was RUF kann / What RUF offers

| Deutsch | English |
|---|---|
| ein Backend für alle Systeme (GTK 4), Fenster nativ positionieren (Windows, macOS, X11) | one backend for all systems (GTK 4), native window positioning (Windows, macOS, X11) |
| Anker für mitwachsende Inhalte | anchors for content that grows along |
| Code-Editor mit Hervorhebung, Faltung, Fehler-, Git- und Haltepunkt-Marken | code editor with highlighting, folding, error, Git and breakpoint marks |
| Ziehen und Ablegen, auch zwischen Fenstern; Kontextmenüs auf jedem Widget | drag and drop, also between windows; context menus on any widget |
| Hell/Dunkel, Programmsymbol, Dateidialoge mit Startordner | light/dark, program icon, file dialogs with a start folder |
| Tests ohne Bildschirm (Xvfb), Dialoge automatisch beantworten | tests without a display (Xvfb), answering dialogs automatically |

## Dokumentation / Documentation

- 🇩🇪 [Deutsch](docs/de/README.md) – Erste Schritte, Architektur, Komponenten, eigene Komponenten, Designer-Anbindung, Release, Fehlerbehebung
- 🇬🇧 [English](docs/en/README.md) – getting started, architecture, components, custom components, designer integration, release, troubleshooting

## KI-Unterstützung / AI support

Skills im offenen Format Agent Skills unter [`.agents/skills/`](.agents/skills), kompakte Regeln in
[`AGENTS.md`](AGENTS.md). / Skills in the open Agent Skills format in [`.agents/skills/`](.agents/skills),
compact rules in [`AGENTS.md`](AGENTS.md).

## Lizenz / License

Nach Wahl lizenziert unter / Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

**Deutsch:** Programme, die mit RUF gebaut werden, dürfen unter beliebiger Lizenz stehen – auch proprietär.
Beiträge werden, sofern nicht ausdrücklich anders angegeben, unter denselben beiden Lizenzen eingebracht.

**English:** Software built with RUF may use any license, including proprietary ones. Unless explicitly
stated otherwise, contributions are dual-licensed as above.

© Dragon Software
