use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Weather in Seattle")
        .data(
            DataBuilder::default()
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
                    .aggregate(Aggregate::Enum(AggregateOp::Count))
                    .build()?)
                .color(ValueDefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .field("weather")
                    .scale(ScaleBuilder::default()
                        .domain(DomainUnion::UnionArray(vec![
                            SelectionInitArrayElement::String("sun".to_string()),
                            SelectionInitArrayElement::String("fog".to_string()),
                            SelectionInitArrayElement::String("drizzle".to_string()),
                            SelectionInitArrayElement::String("rain".to_string()),
                            SelectionInitArrayElement::String("snow".to_string()),
                        ]))
                        .range(ScaleRange::UnionArray(vec![
                            RangeRange::String("#e7ba52".to_string()),
                            RangeRange::String("#c7c7c7".to_string()),
                            RangeRange::String("#aec7e8".to_string()),
                            RangeRange::String("#1f77b4".to_string()),
                            RangeRange::String("#9467bd".to_string()),
                        ]))
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
