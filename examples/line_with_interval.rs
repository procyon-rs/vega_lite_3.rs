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
        //.autosize(AutosizeType::Fit)
        .height(500)
        .width(500)
        .data(
            DataBuilder::default()
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
                .y(YClassBuilder::default()
                    .aggregate(Aggregate::Enum(AggregateOp::Mean))
                    .field("Miles_per_Gallon")
                    .def_type(StandardType::Quantitative)
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
