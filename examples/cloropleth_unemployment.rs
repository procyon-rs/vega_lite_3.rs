use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Choropleth of Unemployment Rate per County")
        .data(
            DataBuilder::default()
                .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/us-10m.json")
                .format(
                    DataFormatBuilder::default()
                        .data_format_type(DataFormatType::Topojson)
                        .feature("counties")
                        .build()?,
                )
                .build()?,
        )
        .mark(Mark::Geoshape)
        .transform(vec![TransformBuilder::default()
            .lookup("id")
            .from(LookupDataBuilder::default()
                .data(DataBuilder::default()
                    .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/unemployment.tsv")
                    .build()?)
                .key("id")
                .fields(vec!["rate".to_string()])
                .build()?)
            .build()?])
        .projection(ProjectionBuilder::default().projection_type(ProjectionType::AlbersUsa).build()?)
        .encoding(
            EncodingBuilder::default()
                .color(
                    ValueDefWithConditionMarkPropFieldDefStringNullBuilder::default()
                        .field("rate")
                        .value_def_with_condition_mark_prop_field_def_string_null_type(StandardType::Quantitative)
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
