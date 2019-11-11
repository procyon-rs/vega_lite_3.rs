use failure;
use showata::Showable;
use vega_lite_3::*;

macro_rules! build {
    ($s:expr ) => {
        $s.build().map_err(|s| failure::format_err!("{}", s))?
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chart = build!(VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(build!(DataBuilder::default().url(
            "https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/data/stocks.csv"
        )))
        .transform(vec![build!(
            TransformBuilder::default().filter("datum.symbol==='GOOG'")
        )])
        .mark(Mark::Line)
        .encoding(build!(EncodingBuilder::default()
            .x(build!(XClassBuilder::default()
                .field("date")
                .def_type(StandardType::Temporal)
                ))
            .y(build!(YClassBuilder::default()
                .field("price")
                .def_type(StandardType::Quantitative)
                )))));
    chart.show()?;
    let content = chart.to_string()?;
    eprint!("{}", content);
    Ok(())
}
