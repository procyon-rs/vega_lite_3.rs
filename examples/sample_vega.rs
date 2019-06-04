use vega_lite_3::*;
//use showata::Showable;
use csv;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub symbol: String,
    pub date: String,
    pub price: f64,
}

macro_rules! build{
    ($s:expr ) => {
        $s.build().map_err(|s| failure::format_err!("{}", s))?
    };
}

fn main() -> Result<(), failure::Error> {
    // {
    //   "$schema": "https://vega.github.io/schema/vega-lite/v3.json",
    //   "description": "Google's stock price over time.",
    //   "data": {"url": "data/stocks.csv"},
    //   "transform": [{"filter": "datum.symbol==='GOOG'"}],
    //   "mark": "line",
    //   "encoding": {
    //     "x": {"field": "date", "type": "temporal"},
    //     "y": {"field": "price", "type": "quantitative"}
    //   }
    // }

    let mut rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv"))?;
    let values = rdr.deserialize().into_iter().collect::<Result<Vec<Item>, csv::Error>>()?;
    let chart = build!(VegaliteBuilder::default()
        .title("Stock price")
        // .width(400.0)
        // .height(200.0)
        // .padding(Some(Padding::Double(5.0)))
        .description("Google's stock price over time.")
        .data(&values)
        .transform(vec![
            build!(TransformBuilder::default().filter(
                "datum.symbol==='GOOG'"
            ))
        ])
        .mark(Mark::Line)
        .encoding(build!(EncodingBuilder::default()
            .x(build!(XClassBuilder::default().field("date").def_type(StandardType::Temporal)))
            .y(build!(YClassBuilder::default().field("price").def_type(StandardType::Quantitative)))
        ))
    );
    //chart.show()?;
    let content = serde_json::to_string(&chart)?;
    eprint!("{}", content);
    Ok(())
}
