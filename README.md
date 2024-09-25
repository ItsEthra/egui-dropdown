[![Crates.io](https://img.shields.io/crates/v/egui-dropdown)](https://crates.io/crates/egui-dropdown)
[![docs.rs](https://img.shields.io/docsrs/egui-dropdown)](https://docs.rs/egui-dropdown)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ItsEthra/egui-dropdown/blob/master/LICENSE)

# egui-dropdown

Dropdown list for egui.

![](media/showcase1.png)

# Installation

```toml
[dependencies]
egui-dropdown = "0.10"
```

or

```sh
cargo add egui-dropdown
```

# Usage

```rust
// Working example can be found in `examples/dropdown.rs`

ui.add(DropDownBox::from_iter(
    &self.items,
    "test_dropbox",
    &mut self.buf,
    |ui, text| ui.selectable_label(false, text)
));
```

# Naming

Although it's called `DropDownBox`, technically speaking it should be called `ComboBox`.
But this is what egui uses for its version of the widget so yeah.
