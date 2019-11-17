use csv;
use serde::{Deserialize, Serialize};
use std::path::Path;
use vega_lite_3::*;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub x: f64,
    pub y: f64,
    pub cluster: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a CSV serialized to a `Vec<Item>`
    let mut rdr = csv::Reader::from_path(Path::new("examples/res/data/clustered_data.csv"))?;
    let values = rdr
        .deserialize()
        .into_iter()
        .collect::<Result<Vec<Item>, csv::Error>>()?;

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Clusters")
        .description("Dots colored by their cluster.")
        .data(&values)
        .mark(Mark::Point)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("x")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("y")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .color(
                    DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                        .field(Field::String("cluster".to_string()))
                        .build()?,
                )
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
