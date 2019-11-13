use csv;
use std::path::Path;
use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a CSV reader
    let rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv"))?;

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(rdr)
        .transform(vec![TransformBuilder::default()
            .filter("datum[0]==='GOOG'")
            .build()?])
        .mark(Mark::Line)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("1")
                    .def_type(StandardType::Temporal)
                    .axis(AxisBuilder::default().title("date").build()?)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("2")
                    .def_type(StandardType::Quantitative)
                    .axis(AxisBuilder::default().title("price").build()?)
                    .build()?)
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
