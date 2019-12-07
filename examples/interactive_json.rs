use std::collections::HashMap;
use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut selector_1 = HashMap::new();
  selector_1.insert(
    //format!("brush"), // Only accept String
    "brush".to_string(),
    SelectionDefBuilder::default()
      .encodings(vec![SingleDefUnitChannel::X])
      .selection_def_type(SelectionDefType::Interval)
      .build()?,
  );
  let mut selector_2 = HashMap::new();
  selector_2.insert(
    "click".to_string(),
    SelectionDefBuilder::default()
      .encodings(vec![SingleDefUnitChannel::Color])
      .selection_def_type(SelectionDefType::Multi)
      .build()?,
  );

  let chart = VegaliteBuilder::default()
    .title("Seattle Weather, 2012-2015")
    .data(
      UrlDataBuilder::default()
        .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/seattle-weather.csv")
        .build()?,
    )
    .vconcat(vec![
      SpecBuilder::default()
        .selection(selector_1)
        .transform(vec![TransformBuilder::default()
          .filter(PurpleLogicalOperandPredicate::Predicate(
            PredicateBuilder::default()
              .selection(PurpleSelectionOperand::String("click".to_string()))
              .build()?,
          ))
          .build()?])
        .mark(Mark::Point)
        .width(600)
        .height(300)
        .encoding(
          EncodingBuilder::default()
            .color(
              DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                .condition(
                  ConditionalPredicateStringValueDefClassBuilder::default()
                    .selection(PurpleSelectionOperand::String("brush".to_string()))
                    .conditional_def_type(StandardType::Nominal)
                    .field("weather")
                    .title("Weather")
                    .scale(
                      ScaleBuilder::default()
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
                        .build()?,
                    )
                    .build()?,
                )
                .value("lightgray")
                .build()?,
            )
            .x(
              XClassBuilder::default()
                .field("date")
                .def_type(StandardType::Temporal)
                .time_unit(TimeUnit::Monthdate)
                .axis(AxisBuilder::default().title("date").format("%b").build()?)
                .build()?,
            )
            .y(
              YClassBuilder::default()
                .field("temp_max")
                .def_type(StandardType::Quantitative)
                .scale(
                  ScaleBuilder::default()
                    .domain(vec![
                      SelectionInitIntervalElement::Double(-5.0),
                      SelectionInitIntervalElement::Double(40.0),
                    ])
                    .build()?,
                )
                .axis(
                  AxisBuilder::default()
                    .title("Maximum Daily Temperature (C)")
                    .build()?,
                )
                .build()?,
            )
            .size(
              DefWithConditionMarkPropFieldDefNumberBuilder::default()
                .title("Precipitation")
                .field("precipitation")
                .def_with_condition_mark_prop_field_def_number_type(StandardType::Quantitative)
                .scale(
                  ScaleBuilder::default()
                    .domain(vec![
                      SelectionInitIntervalElement::Double(-1.0),
                      SelectionInitIntervalElement::Double(50.0),
                    ])
                    .build()?,
                )
                .build()?,
            )
            .build()?,
        )
        .build()?,
      SpecBuilder::default()
        .width(600)
        .mark(Mark::Bar)
        .selection(selector_2)
        .transform(vec![TransformBuilder::default()
          .filter(PurpleLogicalOperandPredicate::Predicate(
            PredicateBuilder::default()
              .selection(PurpleSelectionOperand::String("brush".to_string()))
              .build()?,
          ))
          .build()?])
        .encoding(
          EncodingBuilder::default()
            .color(
              DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                //     .value("lightgray")
                .condition(
                  ConditionalPredicateStringValueDefClassBuilder::default()
                    .selection(PurpleSelectionOperand::String("click".to_string()))
                    .conditional_def_type(StandardType::Nominal)
                    .field("weather")
                    .title("Weather")
                    .build()?,
                )
                .build()?,
            )
            .x(
              XClassBuilder::default()
                .aggregate(AggregateOp::Count)
                .def_type(StandardType::Quantitative)
                .build()?,
            )
            .y(
              YClassBuilder::default()
                .title("Weather")
                .field("weather")
                .def_type(StandardType::Nominal)
                .build()?,
            )
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
