use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(UrlDataBuilder::default().url(
            "https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/data/stocks.csv"
        ).build()?)
        .transform(vec![
            TransformBuilder::default().filter("datum.symbol==='GOOG'")
        .build()?])
        .mark(Mark::Line)
        .encoding(EncodingBuilder::default()
            .x(XClassBuilder::default()
                .field("date")
                .def_type(StandardType::Temporal)
                .build()?)
            .y(YClassBuilder::default()
                .field("price")
                .def_type(StandardType::Quantitative)
                .build()?).build()?).build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
