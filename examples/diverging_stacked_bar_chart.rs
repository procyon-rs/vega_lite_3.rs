use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .description("A population pyramid for the US in 2000.")
        .data(DataBuilder::default().url(
            "https://raw.githubusercontent.com/vega/vega-datasets/master/data/population.json"
        ).build()?)
        .height(200)
        .width(300)
        .transform(vec![
            TransformBuilder::default().filter("datum.year == 2000").build()?,
            TransformBuilder::default().calculate("datum.sex == 2 ? 'Female' : 'Male'").transform_as("gender").build()?,
            TransformBuilder::default().calculate("datum.sex == 2 ? -datum.people : datum.people").transform_as("signed_people").build()?,
        ])
        .mark(Mark::Bar)
        .encoding(EncodingBuilder::default()
            .x(XClassBuilder::default()
                .aggregate(Aggregate::Enum(AggregateOp::Sum))
                .field("signed_people")
                .def_type(StandardType::Quantitative)
                .axis(AxisBuilder::default().title("population").format("s").build()?)
                .build()?)
            .y(YClassBuilder::default()
                .field("age")
                .def_type(StandardType::Ordinal)
                .sort(Sort::Enum(SortOrder::Descending))
                .axis(AxisBuilder::default().build()?) // TODO: set axis to null, see issue #5
                .build()?)
            .color(ValueDefWithConditionMarkPropFieldDefStringNullBuilder::default()
                .field("gender")
                .value_def_with_condition_mark_prop_field_def_string_null_type(StandardType::Nominal)
                .scale(ScaleBuilder::default().range(ScaleRange::UnionArray(vec![RangeRange::String("#e377c2".to_string()), RangeRange::String("#1f77b4".to_string())])).build()?)
                .legend(LegendBuilder::default().orient(LegendOrient::Top).build()?)
                        // TODO: set title of legend to null, see issue #5
                .build()?)
            .build()?)
        .config(ConfigBuilder::default()
            .view(ViewConfigBuilder::default().stroke("".to_string()).build()?) // TODO: set stroke to null, see issue #5

            .axis(AxisConfigBuilder::default().grid(false).build()?)
            .build()?)
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
