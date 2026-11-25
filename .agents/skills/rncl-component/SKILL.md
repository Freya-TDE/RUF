---
name: rncl-component
description: Create a new RNCL component (UI element such as a button, slider or list) for the RUF framework, or extend an existing one with properties/signals. Use when a new widget, a composite component or a third-party component for RUF/RNCL/Freya TDE is to be built.
---

# Creating an RNCL component

An RNCL component is a pure Rust struct that owns a native `ruf::Widget` and implements
`ruf::Component`. Reference implementation: `crates/rncl/src/button.rs` – read it before
writing and copy the pattern.

## Decision: which kind of component?

1. **Composite** (from existing components, e.g. label + text field): no native code
   needed. Template: `crates/rncl/examples/custom_component.rs` (`LabeledField`).
   The outer `RnclPanel` is the component's `widget()`.
2. **A control RUF can already create** (`WidgetKind`): Rust only.
3. **A new native control** (e.g. a date picker): extend the backend first
   → skill `ruf-backend`, then continue here.

## Required parts (checklist)

- [ ] `#[derive(Debug, Clone)] pub struct RnclXxx { widget: Widget }`
      (`Clone` = non-owning reference, for closures).
- [ ] `pub const TYPE_NAME: &'static str = "RnclXxx";` – unique in the registry.
- [ ] One constant per own property: `pub const VALUE: &'static str = "value";` (snake_case).
- [ ] `pub fn new<P: AsWidget + ?Sized>(parent: &P, …) -> Self` → calls `with_parent`.
- [ ] `pub fn with_parent(parent: Option<&Widget>, …) -> Self` → `Widget::create(kind, parent)`;
      for text widgets `widget.adjust_size()` afterwards.
- [ ] Typed methods (`set_value`/`value`, `on_value_changed(impl FnMut(..) + 'static)`).
      Setters take `&self` (core and system hold the state).
- [ ] `pub fn info() -> ComponentInfo` with `.category(..)`, `.description(..)`, and
      `.container(true)` if needed.
- [ ] `impl Component`: `widget`, `type_name`, and for own properties
      `properties` (`base::descriptors()` + own), `property`, `set_property`
      – always delegate unknown names to `base::get` / `base::set`,
      check values with `base::expect_text/int/bool/text_list(name, &value)?`.
      Do not rewrite similar properties; use the helpers from `crates/rncl/src/props.rs`
      (`text`, `items`, `range`, `orientation`); their `set` returns `None` if the name does
      not match → chain with `.or_else(..).unwrap_or_else(|| base::set(..))`, model `slider.rs`.
- [ ] In `crates/rncl/src/lib.rs`: `mod`, `pub use`, `register_all`, `prelude`,
      table in the crate doc.
- [ ] Add a test to `crates/rncl/tests/components.rs` (write the function **and** add it to
      the `tests` list in `main`; `registry_create` checks the count).
- [ ] Docs: `docs/de/komponenten.md` and `docs/en/components.md`.

## Skeleton

```rust
use ruf::{AsWidget, Component, ComponentInfo, PropertyDescriptor, PropertyError,
          PropertyType, PropertyValue, Widget, WidgetKind, base};

#[derive(Debug, Clone)]
pub struct RnclXxx { widget: Widget }

impl RnclXxx {
    pub const TYPE_NAME: &'static str = "RnclXxx";
    pub const VALUE: &'static str = "value";

    pub fn new<P: AsWidget + ?Sized>(parent: &P) -> Self {
        Self::with_parent(Some(parent.as_widget()))
    }

    pub fn with_parent(parent: Option<&Widget>) -> Self {
        let widget = Widget::create(WidgetKind::Container, parent);
        Self { widget }
    }

    pub fn info() -> ComponentInfo {
        ComponentInfo::new(Self::TYPE_NAME, |p| Box::new(Self::with_parent(p)))
            .category("Standard")
            .description("…")
    }
}

impl Component for RnclXxx {
    fn widget(&self) -> &Widget { &self.widget }
    fn type_name(&self) -> &'static str { Self::TYPE_NAME }

    fn properties(&self) -> Vec<PropertyDescriptor> {
        let mut p = base::descriptors();
        p.push(PropertyDescriptor::new(Self::VALUE, PropertyType::Int, "…"));
        p
    }
    fn property(&self, name: &str) -> Option<PropertyValue> {
        match name {
            Self::VALUE => Some(/* … */ 0.into()),
            _ => base::get(&self.widget, name),
        }
    }
    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), PropertyError> {
        match name {
            Self::VALUE => { let _v = base::expect_int(name, &value)?; /* … */ }
            _ => return base::set(&self.widget, name, value),
        }
        Ok(())
    }
}
```

## Rules

- No `unsafe` in components; FFI belongs exclusively in `crates/ruf`.
- No threads: widgets are `!Send`; everything runs in the GUI thread.
- Property names are public API for the designer – do not rename them.
- Description texts short and, like the existing ones, in German (they appear in the designer).

## Checking

```bash
cargo clippy --workspace --all-targets
cargo test --workspace
# Visual check without a screen (look at the image afterwards):
xvfb-run -a cargo run -p rncl --example designer -- --screenshot /tmp/form.png
```

For a visual check of the new component: create it in an example, then
`window.show(); Application::process_events(); window.widget().save_screenshot(path)`.
