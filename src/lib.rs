mod schema;

pub use schema::*;
use serde::Serialize;

#[cfg(feature = "showata")]
mod show_vega;

pub fn iter_to_data<T>(v: impl Iterator<Item = T>) -> Data
where
    T: Serialize,
{
    DataBuilder::default()
        .values(iter_to_data_inline_dataset(v))
        .build()
        .unwrap()
}

fn iter_to_data_inline_dataset<T>(v: impl Iterator<Item = T>) -> DataInlineDataset
where
    T: Serialize,
{
    // let values = v.map(|it|{
    //     match serde_json.to_json(it) {
    //         v: bool => InlineDataset::Bool(v),
    //         v: Double(f64),
    // String(String),

    //         v => AnythingMap(HashMap<String, Option<serde_json::Value>>),
    //     }
    // })
    let values = v
        .map(|it| serde_json::to_value(it))
        .collect::<Result<Vec<_>, _>>()
        .expect("TODO manage error in iter_to_dataInlineDataSet");
    DataInlineDataset::UnionArray(values)
}

impl<T> From<&[T]> for Data
where
    T: Serialize,
{
    fn from(v: &[T]) -> Self {
        iter_to_data(v.iter())
    }
}

impl<T> From<&Vec<T>> for Data
where
    T: Serialize,
{
    fn from(v: &Vec<T>) -> Self {
        iter_to_data(v.iter())
    }
}

impl From<Mark> for AnyMark {
    fn from(v: Mark) -> Self {
        AnyMark::Enum(v)
    }
}

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
    SelectionInitArrayElement,
    Value,
    Field,
    ScaleRange,
    RangeRange,
    Scheme,
    Tooltip,
    Style,
    BindValue,
    InitSelectionInitMapping,
    Translate,
    InlineDatasetValue,
    DataInlineDataset
);
