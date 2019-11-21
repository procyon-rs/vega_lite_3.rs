use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Weather in Seattle")
        .data(
            UrlDataBuilder::default()
                .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/seattle-weather.csv")
                .build()?
        )
        .mark(Mark::Bar)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("date")
                    .time_unit(TimeUnit::Month)
                    .def_type(StandardType::Ordinal)
                    .title("Month of the year")
                    .build()?)
                .y(YClassBuilder::default()
                    .aggregate(AggregateOp::Count)
                    .build()?)
                .color(DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .field("weather")
                    .scale(ScaleBuilder::default()
                        .domain(vec![
                            SelectionInitIntervalElement::String("sun".to_string()),
                            SelectionInitIntervalElement::String("fog".to_string()),
                            SelectionInitIntervalElement::String("drizzle".to_string()),
                            SelectionInitIntervalElement::String("rain".to_string()),
                            SelectionInitIntervalElement::String("snow".to_string()),
                        ])
                        .range(vec![
                            RangeRange::String("#e7ba52".to_string()),
                            RangeRange::String("#c7c7c7".to_string()),
                            RangeRange::String("#aec7e8".to_string()),
                            RangeRange::String("#1f77b4".to_string()),
                            RangeRange::String("#9467bd".to_string()),
                        ])
                        .build()?)
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
