// This is the schema we are trying to mimic
// {
//   "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
//   "data": {"url": "data/cars.json"},
//   "encoding": {
//     "x": {
//       "field": "Year",
//       "type": "temporal",
//       "timeUnit": "year"
//     }
//   },
//   "layer": [
//     {
//       "mark": {"type": "errorband", "extent": "ci"},
//       "encoding": {
//         "y": {
//           "field": "Miles_per_Gallon",
//           "type": "quantitative",
//           "title": "Mean of Miles per Gallon (95% CIs)"
//         }
//       }
//     },
//     {
//       "mark": "line",
//       "encoding": {
//         "y": {
//           "aggregate": "mean",
//           "field": "Miles_per_Gallon",
//           "type": "quantitative"
//         }
//       }
//     }
//   ]
// }
use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Line Chart with Confidence Interval Band")
        // .autosize(Autosize::Enum(AutosizeType::Fit))
        .height(200)
        .width(300)
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
                            .aggregate(Aggregate::Enum(AggregateOp::Mean))
                            .build()?)
                        .build()?,
                )
                .build()?,
            LayerSpecBuilder::default()
                // .mark(Mark::Errorband)
                .mark(AnyMark::MarkDefClass(
                    MarkDefClassBuilder::default()
                        .def_type(Mark::Errorband)
                        .extent(BoxPlotDefExtent::Enum(ExtentExtent::Ci))
                        .build()?,
                ))
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
