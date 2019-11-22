use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Line Chart with Confidence Interval Band")
        .autosize(AutosizeType::Fit)
        //.height(200)
        //.width(300)
        .data(
            UrlDataBuilder::default()
                .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/cars.json")
                .build()?,
        )
        .mark(Mark::Line)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("Year")
                    .def_type(StandardType::Temporal)
                    .time_unit(TimeUnit::Year)
                    .build()?)
                .build()?,
        )
        .layer(vec![
            LayerSpecBuilder::default()
                .mark(Mark::Line)
                .encoding(
                    LayerEncodingBuilder::default()
                        .y(YClassBuilder::default()
                            .field("Miles_per_Gallon")
                            .def_type(StandardType::Quantitative)
                            .aggregate(AggregateOp::Mean)
                            .build()?)
                        .build()?,
                )
                .build()?,
            LayerSpecBuilder::default()
                .mark(
                    MarkDefClassBuilder::default()
                        .def_type(Mark::Errorband)
                        .extent(ExtentExtent::Ci)
                        .build()?,
                )
                .encoding(
                    LayerEncodingBuilder::default()
                        .y(YClassBuilder::default()
                            .title("Mean of Miles per Gallon (95% CIs)")
                            .field("Miles_per_Gallon")
                            .def_type(StandardType::Quantitative)
                            .build()?)
                        .build()?,
                )
                .build()?,
        ])
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
