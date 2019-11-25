use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use serde_json;
use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let values: Array2<f64> = Array::random((100, 2), Uniform::new(0., 1000.));
    let mut chart: Vegalite = serde_json::from_str(spec)?;
    chart.data = values.into();
    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprintln!("{}", chart.to_string()?);

    Ok(())
}
