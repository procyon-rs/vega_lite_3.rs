use serde_json;
use vega_lite_3::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = r##"
{
  "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
  "description": "A population pyramid for the US in 2000, created using stack. See https://vega.github.io/vega-lite/examples/concat_population_pyramid.html for a variant of this created using concat.",
  "data": { "url": "https://raw.githubusercontent.com/vega/vega-datasets/master/data/population.json"},
  "transform": [
    {"filter": "datum.year == 2000"},
    {"calculate": "datum.sex == 2 ? 'Female' : 'Male'", "as": "gender"},
    {"calculate": "datum.sex == 2 ? -datum.people : datum.people", "as": "signed_people"}
  ],
  "width": 300,
  "height": 200,
  "mark": "bar",
  "encoding": {
    "y": {
      "field": "age", "type": "ordinal",
      "axis": null, "sort": "descending"
    },
    "x": {
      "aggregate": "sum", "field": "signed_people", "type": "quantitative",
      "axis": {"title": "population", "format": "s"}
    },
    "color": {
      "field": "gender", "type": "nominal",
      "scale": {"range": ["#e377c2", "#1f77b4"]},
      "legend": {"orient": "top", "title": null}
    }
  },
  "config": {
    "view": {"stroke": null},
    "axis": {"grid": false}
  }
}
"##;

    let chart: Vegalite = serde_json::from_str(spec)?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprintln!("{}", chart.to_string()?);

    eprintln!("{:#?}", chart);
    Ok(())
}
