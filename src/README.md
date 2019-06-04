# How to make `vegalite3`

The format command: `rustfmt src/vegalite3/schema.rs`

The check command: `cargo check`

- generate `vegalite3/schema.rs` with https://app.quicktype.io/
  - copy [`vega-lite`'s json schema](https://vega.github.io/schema/vega-lite/v3.json) in source field
  - set:
    - `Name` to `Vegalite`
    - `Source type` to `JSON Schema`
    - `Language` to `Rust`
    - `Density` to `Dense`
    - `Field Visibility` to `Public`
    - `Derive Debug impl` checked
  - copy the result into `vegalite.rs`
- replace the file header by the License header
- replace (non-regex)`extern crate serde_json;` by `use serde::{Deserialize, Serialize};`
  - reformat
- set every `None` not serialized as `null` (to let `vega-lite` manage defaults)
  - replace all (regex) `pub (\w*): Option` by `#[serde(skip_serializing_if = "Option::is_none")] pub $1: Option`
  - reformat
- rename `Box` into `DefBox` to remove conflict with std Box
  - replace all (non-regex) `Option<Box>` by `Option<DefBox>`
  - replace all (non-regex)`enum Box {` by `enum DefBox {`
  - check
- make the types `clonables`
  - replace all (non-regex) `#[derive(Debug, Serialize, Deserialize)]` by `#[derive(Debug, Clone, Serialize, Deserialize)]`
- setup builder
  - add `use derive_builder::Builder;`
  - replace all (regex) `Deserialize\)\]\npub struct` by `Deserialize, Builder)]\n#[builder(setter(into, strip_option))]\npub struct`
  - replace all (regex) `pub (\w*): Option` by `#[builder(default)] pub $1: Option`
  - replace once (non-regex) `#[builder(default)] pub schema: Option` by `#[builder(default = "Some(\"https://vega.github.io/schema/vega-lite/v3.json\".to_string())")] pub schema: Option`
  - reformat with `rustfmt src/vegalite.rs`
  - check
- manual change in types:
  - remove `InlineDataset` 
    - comment all the definition of `enum InlineDataset`
    - replace all (non-regex) `<InlineDataset>`by `<serde_json::value::Value>`
  - simplify AnyMark
    - replace (non-regex) `BoxPlotDefClass` by `MarkDefClass`
    - replace (non-regex) `BoxPlotDefExtent` by `MarkDefExtent`
    - replace (non-regex) `Enum(BoxPlot` into `Enum(Mark` 
    - replace (non-regex) `enum BoxPlot ` into `enum Mark `
    - replace (non-regex) `<BoxPlot>`into `<Mark>`
    - reformat
  - simplify some boxed type
    - replace (non-regex) `pub spec: Box<Option<SpecClass>>,`by `pub spec: Option<Box<SpecClass>>,` in SpecClass
    - replace all `pub (\w*): Box<Option<(\S*)>>` by `#[serde(skip_serializing_if = "Option::is_none")] #[builder(default)] pub $1: Option<$2>`
    - replace (non-regex) `pub filter: Option<Box<PurpleLogicalOperandPredicate>>,` by `#[serde(skip_serializing_if = "Option::is_none")] #[builder(default)] pub filter: Option<PurpleLogicalOperandPredicate>,`
    - reformat
    - check


<!-- - remove every recursive type `Box` (useless for the moment)
  - replace all `Box<(\w*)>` by `$1`
  - replace all `Box<Option<(\w*)>>` by `Option<$1>`
  - reformat with `rustfmt src/vegalite.rs` -->
