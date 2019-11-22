# Vega-lite V3 for Rust

[![license](https://img.shields.io/crates/l/vega_lite_3.svg)](https://spdx.org/licenses/Apache-2.0.html)
[![version](https://img.shields.io/crates/v/vega_lite_3.svg)](https://crates.io/crates/vega_lite_3)
[![Release Doc](https://docs.rs/vega_lite_3/badge.svg)](https://docs.rs/vega_lite_3)

[![Actions Status](https://github.com/procyon-rs/vega_lite_3.rs/workflows/ci-flow/badge.svg)](https://github.com/procyon-rs/vega_lite_3.rs/actions)
[![codecov](https://codecov.io/gh/procyon-rs/vega_lite_3.rs/branch/master/graph/badge.svg)](https://codecov.io/gh/procyon-rs/vega_lite_3.rs)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=procyon-rs/vega_lite_3.rs)](https://dependabot.com)

## About

A Rust API for vega-lite v3 to build chart with a rusty API.

It's possible to generate a complete vega-lite json and display the result in your Browser or in Jupyter. This feature is made possible thanks to [showata](https://crates.io/crates/showata).
It's also possible to use an existing vega-lite json and plug your data source seamlessly. This way you can leverage existing vizualisation and adapt it to your design.

## Examples

In order to have a complete mapping of the vega-lite v3 specification the code for the schema was automaticlly generated.
To help describe all the possible features a [gallery of example is provided on github](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/)

[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/cloropleth_unemployment.png" height="150px">](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/cloropleth_unemployment.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/diverging_stacked_bar_chart.png" height="150px">](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/diverging_stacked_bar_chart.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/scatterplot.png" height="150px">](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/scatterplot.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stacked_bar_chart.png" height="150px">](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/stacked_bar_chart.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png" height="150px">](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/stock_graph.rs)

### Simple chart using ndarray generated data

```rust
let values: Array2<f64> = Array::random((100, 2), StandardNormal);

let chart = VegaliteBuilder::default()
    .title("Random points")
    .data(values)
    .mark(Mark::Point)
    .encoding(
        EncodingBuilder::default()
            .x(XClassBuilder::default()
                .field("data.0")
                .def_type(StandardType::Quantitative)
                .build()?)
            .y(YClassBuilder::default()
                .field("data.1")
                .def_type(StandardType::Quantitative)
                .build()?)
            .build()?,
    )
    .build()?;
chart.show()?;
```

### Simple chart using existing json definition with new data

```rust
// Use existing vega-lite json specification
let spec = r##"{
    "$schema": "https://vega.github.io/schema/vega-lite/v3.4.0.json",
    "encoding": {
        "x": {
            "field": "data.0",
            "type": "quantitative"
        },
        "y": {
            "field": "data.1",
            "type": "quantitative"
        }
    },
    "mark": "point",
    "title": "Random points"
}"##;

// Use you own data to populate the chart
let values: Array2<f64> = Array::random((100, 2), StandardNormal);
let mut chart: Vegalite = serde_json::from_str(spec)?;
chart.data = values.into();

// display the chart using `showata`
chart.show()?;
```

## Features

| name      | enabled by default | functionnality                                     |
| --------- | ------------------ | -------------------------------------------------- |
| show_vega | yes                | can display charts in the browser or in a notebook |
| csv       | yes                | can load data from a csv                           |
| ndarray   | yes                | can load data from a ndarray                       |

## Links

- [Wiki - AGuideToRustGraphicsLibraries2019](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019)
- [A Dramatic Tour through Python’s Data Visualization Landscape (including ggplot and Altair) – Regress to Impress](https://dsaber.com/2016/10/02/a-dramatic-tour-through-pythons-data-visualization-landscape-including-ggplot-and-altair/)
- [Specifying Data in Altair — Altair 3.0.0 documentation](https://altair-viz.github.io/user_guide/data.html#long-form-vs-wide-form-data)
- [Visualization — list of Rust libraries/crates // Lib.rs](https://lib.rs/visualization)
- [Quicktype](https://quicktype.io/) (got issue with the alternative https://transform.now.sh/json-to-rust-serde) was used to bootstrap `vegalite.rs` from the [vega-lite's json schema](https://vega.github.io/schema/vega-lite/v3.json)
