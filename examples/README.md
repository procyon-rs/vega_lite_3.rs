# Graph types

## [Scatterplot](https://github.com/davidB/vega_lite_3.rs/blob/master/examples/scatterplot.rs)

```
cargo run --example scatterplot --features show_vega
```

Loads data from a `csv`, deserializing to a `struct`, and display a colored scatterplot.

<img src="https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/screens/scatterplot.png">

## [Stock Graph](https://github.com/davidB/vega_lite_3.rs/blob/master/examples/stock_graph.rs)

```
cargo run --example stock_graph --features show_vega
```

Loads data from a `csv`, deserializing to a `struct`, and display the graph as a line.

<img src="https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">

# Data Sources

## [From ndarray](https://github.com/davidB/vega_lite_3.rs/blob/master/examples/from_ndarray.rs)

```
cargo run --example from_ndarray --features show_vega,ndarray
```

Loads data directly from a random [`ndarray::Array2`](https://docs.rs/ndarray/latest/ndarray/type.Array2.html).

<img src="https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/screens/from_ndarray.png">

## [From csv](https://github.com/davidB/vega_lite_3.rs/blob/master/examples/from_csv.rs)

```
cargo run --example from_csv --features show_vega,csv
```

Loads data directly from a `csv` without deserializing to a `struct`.

<img src="https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/screens/stock_graph.png">
