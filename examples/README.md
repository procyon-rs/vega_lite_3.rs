# Examples Gallery

## Graph types

### [Cloropleth](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/cloropleth_unemployment.rs)

```bash
cargo run --example cloropleth_unemployment
```

Display data from a csv and a tsv as a cloropleth on a geo projection.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/cloropleth_unemployment.png">

### [Diverging Stacked Bar Chart](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/diverging_stacked_bar_chart.rs)

```bash
cargo run --example diverging_stacked_bar_chart
```

Display data from a json after doing some transformation on it as a stacked bar chart

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/diverging_stacked_bar_chart.png">

### [Scatterplot](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/scatterplot.rs)

```bash
cargo run --example scatterplot
```

Loads data from a `csv`, deserializing to a `struct`, and display a colored scatterplot.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/scatterplot.png">

### [Stacked Bar Graph](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/stacked_bar_chart.rs)

```bash
cargo run --example stacked_bar_chart
```

Loads data from an URL, displaying the aggregated count by type by month as a stacked bar graph.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stacked_bar_chart.png">

### [Stock Graph](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/stock_graph.rs)

```bash
cargo run --example stock_graph
```

Loads data from a `csv`, deserializing to a `struct`, and display the graph as a line.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

### [Line with confidence Interval](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/line_with_interval.rs)

```bash
cargo run --example line_with_interval.rs
```

Loads data from an URL, displaying the aggregated mean by year with the variance on a two level layers as a line graph.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

## Data Sources

### [From ndarray](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_ndarray.rs)

```bash
cargo run --example from_ndarray
```

Loads data directly from a random [`ndarray::Array2`](https://docs.rs/ndarray/latest/ndarray/type.Array2.html).

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/from_ndarray.png">

### [From csv](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_csv.rs)

```bash
cargo run --example from_csv
```

Loads data directly from a `csv` without deserializing to a `struct`.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

### [From url](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_url.rs)

```bash
cargo run --example from_url
```

Loads data directly from an URL.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

### [From JSON spec](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_json_spec.rs)

```bash
cargo run --example from_json_spec
```

Load the chart directly from the JSON spec.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/diverging_stacked_bar_chart.png">

### [From JSON spec with new Data](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_mixed_json_rust.rs)

```bash
cargo run --example from_mixed_json_rust
```

Create a chart from existing json and add new data

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/mixed.png">

## Miscellaneous

### [Without using builders](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/without_builders.rs)

```bash
cargo run --example without_builders
```

Build a graph without using the builders. This is not the recommended way as it's more verbose and expose more the
internal structures and types, but it's possible when wanting to avoid `Result`s produced by the Builders.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">
