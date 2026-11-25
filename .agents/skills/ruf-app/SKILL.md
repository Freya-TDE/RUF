---
name: ruf-app
description: Build a desktop application or a form with RUF and RNCL components (windows, buttons, input fields, events), including a visual check via offscreen screenshot. Use when creating a user interface with ruf/rncl, writing an example or looking for a bug in a RUF app.
---

# Building an application with RUF/RNCL

## Dependencies

```toml
[dependencies]
ruf  = { version = "0.4" }
rncl = { version = "0.4" }
```

RUF uses GTK 4 on Linux, Windows and macOS (needs the `gtk4` development files;
Windows: MSYS2 UCRT64, macOS: Homebrew). No C++, no Qt, no features.
Shipping with the GTK runtime: `tools/bundle-*.sh`, see `docs/en/release.md`.

## Basic pattern

```rust
use rncl::prelude::*;

fn main() {
    let app = Application::new();                // 1. always first
    let window = RnclWindow::new("Title");       // 2. window
    window.set_size(400, 300);

    let label = RnclLabel::new(&window, "…");    // 3. children: position relative to the parent
    label.set_geometry(20, 20, 360, 24);

    let button = RnclButton::new(&window, "OK");
    button.set_position(20, 60);
    let l = label.clone();                       // 4. clone for the closure
    button.on_clicked(move || l.set_text("Clicked"));

    window.show();                               // 5. show
    std::process::exit(app.exec());              // 6. event loop
}
```

## Components (full reference: `docs/en/components.md`)

| Type | Constructor | Main API |
|---|---|---|
| `RnclWindow` | `new(title)` | `set_title`, `close`, `on_resized`, `on_close_requested`, `add_shortcut` |
| `RnclPanel` | `new(&parent)` | container, positions relative |
| `RnclGroupBox` | `new(&parent, title)` | container with frame; the title takes ~20 px at the top |
| `RnclTabView` | `new(&parent)` | `add_tab(title) -> RnclPanel`, `remove_tab`, `set_current_index`, `on_current_index_changed` |
| `RnclButton` | `new(&parent, text)` | `set_text`, `on_clicked`, `click` |
| `RnclLabel` | `new(&parent, text)` | `set_text` (simple HTML allowed), `set_word_wrap` |
| `RnclTextField` | `new(&parent)` | `set_text`, `set_placeholder`, `set_password_mode`, `set_read_only`, `on_text_changed`, `on_return_pressed` |
| `RnclTextArea` | `new(&parent)` | `set_text`, `append_line`, `set_read_only`, `on_text_changed`; code editor: `set_monospace`, `add_highlight_rule`, `set_marked_line`, `goto_line` |
| `RnclCheckBox` | `new(&parent, text)` | `set_checked`, `is_checked`, `on_toggled`, `click` |
| `RnclRadioButton` | `new(&parent, text)` | like CheckBox; exclusive per parent → groups via `RnclGroupBox` |
| `RnclComboBox` | `new(&parent)` | `set_items`, `current_index`, `current_text`, `set_editable`, `on_current_index_changed`, `on_text_changed` |
| `RnclListBox` | `new(&parent)` | `set_items`, `add_item`, `current_index`, `current_text() -> Option`, `on_current_index_changed` |
| `RnclSpinBox` | `new(&parent)` | `set_range`, `set_value`, `on_value_changed` (default 0…99) |
| `RnclSlider` | `new(&parent)` | `set_range`, `set_value`, `set_vertical`, `on_value_changed` (default 0…99) |
| `RnclProgressBar` | `new(&parent)` | `set_value`, `set_range`, `set_busy` (default 0…100) |
| `RnclScrollView` | `new(&parent)` | children in `content()`, `set_content_size(0, height)` |
| `RnclImageBox` | `new(&parent)` | `set_file(path) -> io::Result`, `set_image`, `set_fit(ImageFit)`, `set_gamma`, `set_opacity`, `on_clicked` (default 96×96) |
| `RnclImageScrollBox` | `new(&parent)` | `set_files(&[..]) -> Vec<unreadable>`, `set_images`, `set_layout(GalleryLayout)`, `set_thumb_size`, `set_spacing`, `current_index`, `current_file`, `on_current_index_changed` |

All of them (trait `Component`, comes with the prelude): `set_position`, `set_size`,
`set_geometry`, `show`, `hide`, `set_enabled`, `set_tool_tip`, `set_style_sheet`,
`set_name`, `property`/`set_property`.

## Dynamic user interfaces and background work

- **Layout on resize:** RUF positions absolutely. Call `window.on_resized(|w, h| …)`
  or `panel.widget().on_resized(…)` (e.g. for tab pages) and set the children's
  `set_geometry` there.
- **Children created at run time** are visible at once; remove them with
  `component.widget().delete_later()`.
- **Debouncing:** `ruf::Timer` (single shot), `start(ms)` again on every input.
- **Network/files/processes:** run in a `std::thread`, send the result back with
  `ruf::GuiSender::new(move |v: T| …)` – the handler runs in the GUI thread.
  Discard outdated answers with a generation counter.
- **Questions:** `ruf::dialog::{information, warning, error, confirm, input, password}` (modal –
  not in tests); intercept closing with `window.on_close_requested(|| bool)`.
- **Signals while filling:** setters fire signals. When filling programmatically, set a
  `Cell<bool>` flag and check it in the handlers.
- **Texts in several languages:** `ruf::i18n` (TOML per language, `ruf::tr!`), see
  `docs/en/components.md`.

Larger practical example: `../CT-Editor` (search with background threads, dynamic
checkbox lists, code editor, tabs with their own layout function).

## Rules that avoid bugs

- Create the `Application` before any component, otherwise it panics with a clear message.
- Only **one** `Application` per process; everything on the main thread.
- Keep the owning `RnclWindow` alive (a variable in `main`). If it is dropped, the window
  disappears with its children. Child variables may be dropped – the widgets stay (the
  parent owns them).
- Closures need `'static`: capture components with `.clone()`, share own state with
  `Rc<Cell<_>>`/`Rc<RefCell<_>>`.
- Signals also fire on programmatic changes (`set_text`, `set_checked`).
- **Platform differences:** GTK never makes controls smaller than their natural size (use
  `adjust_size()` for buttons/labels); GTK layout is asynchronous (`process_events()` waits
  for it); `set_style_sheet` only knows `color`, `background`, `font-weight`; highlight
  patterns without lookarounds (group 1 is colored).
- `Application::quit()` and `Application::set_color_scheme(..)` are associated functions and
  can therefore be called from any closure.

## Visual check without a screen (for AI agents)

```rust
window.show();
Application::process_events();
window.widget().save_screenshot("/tmp/ui.png");
```

```bash
xvfb-run -a cargo run --example my_example   # Linux without a screen
```

Then look at the PNG and check the layout (overlaps, cut-off texts).
Ready-made examples: `cargo run -p rncl --example designer -- --screenshot /tmp/form.png`
and `cargo run -p rncl --example gallery -- --screenshot /tmp/gallery.png` (all components).

## Tests

Tests with a user interface need the main thread: a test file with `harness = false` and its
own `main()`, `Application::new()`; Linux without a screen: `xvfb-run -a cargo test`.
Template: `crates/rncl/tests/components.rs`.
