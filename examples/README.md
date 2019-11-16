# Graph types

## [Cloropleth](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/cloropleth_unemployment.rs)

```
cargo run --example cloropleth_unemployment
```

Display data from a csv and a tsv as a cloropleth on a geo projection.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/cloropleth_unemployment.png">

## [Diverging Stacked Bar Chart](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/diverging_stacked_bar_chart.rs)

```
cargo run --example diverging_stacked_bar_chart
```

Display data from a json after doing some transformation on it as a stacked bar chart

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/diverging_stacked_bar_chart.png">

## [Scatterplot](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/scatterplot.rs)

```
cargo run --example scatterplot
```

Loads data from a `csv`, deserializing to a `struct`, and display a colored scatterplot.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/scatterplot.png">

## [Stacked Bar Graph](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/stacked_bar_chart.rs)

```
cargo run --example stacked_bar_chart
```

Loads data from an URL, displaying the aggregated count by type by month as a stacked bar graph.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stacked_bar_chart.png">

## [Stock Graph](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/stock_graph.rs)

```
cargo run --example stock_graph
```

Loads data from a `csv`, deserializing to a `struct`, and display the graph as a line.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

# Data Sources

## [From ndarray](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_ndarray.rs)

```
cargo run --example from_ndarray
```

Loads data directly from a random [`ndarray::Array2`](https://docs.rs/ndarray/latest/ndarray/type.Array2.html).

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/from_ndarray.png">

## [From csv](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_csv.rs)

```
cargo run --example from_csv
```

Loads data directly from a `csv` without deserializing to a `struct`.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

## [From url](https://github.com/procyon-rs/vega_lite_3.rs/blob/master/examples/from_url.rs)

```
cargo run --example from_url
```

Loads data directly from an URL.

<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">
