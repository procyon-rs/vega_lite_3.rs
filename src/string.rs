// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::schema::*;
// use std::str::FromStr;
// use std::convert::TryFrom;
use serde_json;

// impl FromStr for Vegalite {
//     type Err = serde_json::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         serde_json::from_str(s)
//     }
// }

impl Vegalite {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// impl TryFrom<&Vegalite> for String {
//     type Error = serde_json::Error;
//     fn try_from(v: &Vegalite) -> Result<Self, Self::Error> {
//         v.to_string()
//     }
// }

// for every enum with String(String)
macro_rules! from_into_string{
    ( $( $x:ident ),* ) => {
            $(
                impl<S> From<S> for $x
                where
                    S: Into<String>,
                {
                    fn from(v: S) -> Self {
                        $x::String(v.into())
                    }
                }
            )*
    };
}

from_into_string!(
    Title,
    SelectionOperandElement,
    PurpleSelectionOperand,
    LogicalOperandPredicateElement,
    PurpleLogicalOperandPredicate,
    EqualUnion,
    Day,
    Month,
    Lt,
    SelectionInitIntervalElement,
    Value,
    Field,
    ScaleRange,
    RangeRange,
    Scheme,
    TooltipUnion,
    Style,
    BindValue,
    InitSelectionInitMapping,
    Translate,
    InlineDatasetValue,
    UrlDataInlineDataset
);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_string_to_string() {
//         let json1 = r#"
//             {
//                 "$schema": "https://vega.github.io/schema/vega-lite/v3.json",
//                 "description": "Google's stock price over time.",
//                 "data": {"url": "data/stocks.csv"},
//                 "transform": [{"filter": "datum.symbol==='GOOG'"}],
//                 "mark": "line",
//                 "encoding": {
//                     "x": {"field": "date", "type": "temporal"},
//                     "y": {"field": "price", "type": "quantitative"}
//                 }
//             }
//         "#;
//         let vega1 = Vegalite::from_str(json1).unwrap();
//         //dbg!(vega1);
//         //let json2 = vega1.to_string().unwrap();
//         //let vega2 = Vegalite::from_str(json2).unwrap();
//         //assert_eq!(json1, json2);
//         //assert_eq!(vega1, vega2);
//     }
// }
