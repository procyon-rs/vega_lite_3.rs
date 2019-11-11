use failure;
use showata::Showable;
use vega_lite_3::*;

use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;

macro_rules! build {
    ($s:expr ) => {
        $s.build().map_err(|s| failure::format_err!("{}", s))?
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Array2<f64> = Array::random((100, 2), StandardNormal);

    let chart = build!(VegaliteBuilder::default()
        .title("Random points")
        .data(values)
        .mark(Mark::Point)
        .encoding(build!(EncodingBuilder::default()
            .x(build!(XClassBuilder::default()
                .field("data.0")
                .def_type(StandardType::Quantitative)))
            .y(build!(YClassBuilder::default()
                .field("data.1")
                .def_type(StandardType::Quantitative))))));
    chart.show()?;
    let content = chart.to_string()?;
    eprint!("{}", content);
    Ok(())
}
