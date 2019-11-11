use csv;
use failure;
use serde::{Deserialize, Serialize};
use showata::Showable;
use std::path::Path;
use vega_lite_3::*;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub x: f64,
    pub y: f64,
    pub cluster: usize,
}

macro_rules! build {
    ($s:expr ) => {
        $s.build().map_err(|s| failure::format_err!("{}", s))?
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(Path::new("examples/res/data/clustered_data.csv"))?;
    let values = rdr
        .deserialize()
        .into_iter()
        .collect::<Result<Vec<Item>, csv::Error>>()?;
    let chart = build!(VegaliteBuilder::default()
        .title("Clusters")
        .description("Dots colored by their cluster.")
        .data(&values)
        .mark(Mark::Point)
        .encoding(build!(EncodingBuilder::default()
            .x(build!(XClassBuilder::default()
                .field("x")
                .def_type(StandardType::Quantitative)))
            .y(build!(YClassBuilder::default()
                .field("y")
                .def_type(StandardType::Quantitative)))
            .color(build!(
                ValueDefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .field(Field::String("cluster".to_string()))
            )))));
    chart.show()?;
    let content = chart.to_string()?;
    eprint!("{}", content);
    Ok(())
}
