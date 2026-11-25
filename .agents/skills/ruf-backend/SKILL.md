---
name: ruf-backend
description: Extend the system layer of RUF – new controls (WidgetKind), new widget capabilities or toolkit functions in the GTK 4 backend (runs on Linux, Windows and macOS). Use for changes to crates/ruf/src/backend/*, crates/ruf/src/core.rs or for build/run-time errors that affect only one operating system (e.g. missing GTK DLLs on Windows).
---

# Extending the RUF backend

RUF has one backend: GTK 4 (`gtk4-rs`) on all systems. Win32 and Cocoa were removed on
23.10.2026 (Git history). The trait `Backend` stays as a layer.

| File | Content |
|---|---|
| `crates/ruf/src/backend/mod.rs` | trait `Backend` – the backend's contract |
| `crates/ruf/src/backend/gtk.rs` | GTK 4, container `RufFixed` |
| `crates/ruf/src/core.rs` | platform-independent logic: registry, `sync`, semantics |
| `crates/ruf/src/widget.rs` | public API, calls the core or `Platform::…` |

## Basic rule: what goes where?

- **Backend:** create, set, read, report user actions
  (`core::notify(id, Notify::Clicked | Changed | Activated | Resized(size))`,
  `core::close_requested`, `core::timer_fired`, `core::shortcut_fired`).
- **Core:** semantics – when signals fire, exclusivity, limits, selection semantics.
  Do not move this into the backend.
- During `core::setter(|| Platform::…)` the core ignores reports; afterwards it calls
  `sync(id)` and fires exactly the changed signals.

## Recipe: new widget capability

1. Add the method to the trait `Backend` (doc comment in German, like the rest of the file).
2. Implement it in the GTK backend.
3. Public method in `widget.rs`: store state in the core node if needed (`core::update`),
   then `core::setter(|| Platform::…)`; on state changes `core::sync`.
4. Test in `crates/rncl/tests/components.rs` (function + entry in the `tests` list).

## Recipe: new control

1. Add the **next free number** to `WidgetKind` in `core.rs`.
2. Adjust `Signal::supported_by`, and `is_container`/`has_range` if needed.
3. In the backend: a case in `create` (signal connections → `core::notify`) and in the
   affected getters/setters.
4. RNCL component → skill `rncl-component`.

## GTK notes

- Controls are never smaller than their natural size. Layout is asynchronous
  (frame clock); `RufFixed::size_allocate` reports sizes via idle.
- Do **not** use `glib::MainContext::invoke` for `post` (may run in a foreign thread) –
  use `glib::idle_add_once`.
- Shortcuts: `<Primary>` instead of `<Control>` – on macOS that is the Command key.
- No Linux-only APIs (X11/Wayland specific) without a guard; the code also runs on Windows
  and macOS. Platform differences at most via `cfg!(target_os = …)`.

## Checking

```bash
cargo clippy --workspace --all-targets
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 xvfb-run -a cargo test --workspace
```

In a sandbox that shields `/tmp/.X11-unix`, Xvfb does not start – run the tests outside
the sandbox. Windows/macOS: build natively (MSYS2 UCRT64 or Homebrew) and run
`cargo test --workspace`; ship with `tools/bundle-*.sh` (`docs/en/release.md`).
