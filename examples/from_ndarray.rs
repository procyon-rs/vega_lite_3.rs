use showata::Showable;
use vega_lite_3::*;

use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let content = chart.to_string()?;
    eprint!("{}", content);
    Ok(())
}
