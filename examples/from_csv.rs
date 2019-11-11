use csv;
use failure;
use showata::Showable;
use std::path::Path;
use vega_lite_3::*;

macro_rules! build {
    ($s:expr ) => {
        $s.build().map_err(|s| failure::format_err!("{}", s))?
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv"))?;
    let chart = build!(VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(rdr)
        .transform(vec![build!(
            TransformBuilder::default().filter("datum[0]==='GOOG'")
        )])
        .mark(Mark::Line)
        .encoding(build!(EncodingBuilder::default()
            .x(build!(XClassBuilder::default()
                .field("1")
                .def_type(StandardType::Temporal)
                .axis(build!(AxisBuilder::default().title("date")))))
            .y(build!(YClassBuilder::default()
                .field("2")
                .def_type(StandardType::Quantitative)
                .axis(build!(AxisBuilder::default().title("price"))))))));
    chart.show()?;
    let content = chart.to_string()?;
    eprint!("{}", content);
    Ok(())
}
