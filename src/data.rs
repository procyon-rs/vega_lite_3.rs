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
use crate::Data;
use crate::DataBuilder;
use crate::DataInlineDataset;
use serde::Serialize;

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

#[cfg(feature = "ndarray")]
use ndarray::ArrayBase;

#[cfg(feature = "ndarray")]
impl<A, D, S> From<ArrayBase<S, D>> for Data
where
    A: Serialize,
    D: ndarray::Dimension,
    S: ndarray::Data<Elem = A>,
{
    fn from(v: ArrayBase<S, D>) -> Self {
        iter_to_data(v.genrows().into_iter())
    }
}

#[cfg(feature = "csv")]
use csv::Reader;

#[cfg(feature = "csv")]
impl<R> From<Reader<R>> for Data
where
    R: std::io::Read,
{
    fn from(mut v: Reader<R>) -> Self {
        DataBuilder::default()
            .values(DataInlineDataset::UnionArray(
                v.records()
                    .into_iter()
                    .map(|it: Result<csv::StringRecord, _>| {
                        serde_json::Value::Array(
                            it.expect("TODO manage error in csv")
                                .iter()
                                .map(|f: &str| serde_json::Value::String(f.to_string()))
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>(),
            ))
            .build()
            .unwrap()
    }
}
