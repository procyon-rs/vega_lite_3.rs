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
use crate::RemovableValue;
use crate::UrlData;
use crate::UrlDataBuilder;
use crate::UrlDataInlineDataset;
use serde::Serialize;

/// Helper method turning an iterator over a `Serialize`-able type into a data that can't be used in a graph.
pub fn iter_to_data<T>(v: impl Iterator<Item = T>) -> UrlData
where
    T: Serialize,
{
    UrlDataBuilder::default()
        .values(iter_to_data_inline_dataset(v))
        .build()
        .unwrap()
}

fn iter_to_data_inline_dataset<T>(v: impl Iterator<Item = T>) -> UrlDataInlineDataset
where
    T: Serialize,
{
    let values = v
        .map(serde_json::to_value)
        .collect::<Result<Vec<_>, _>>()
        .expect("TODO manage error in iter_to_dataInlineDataSet");
    UrlDataInlineDataset::UnionArray(values)
}

impl<T> From<&[T]> for UrlData
where
    T: Serialize,
{
    fn from(v: &[T]) -> Self {
        iter_to_data(v.iter())
    }
}
impl<T> From<&[T]> for RemovableValue<UrlData>
where
    T: Serialize,
{
    fn from(v: &[T]) -> Self {
        RemovableValue::Specified(v.into())
    }
}

impl<T> From<&Vec<T>> for UrlData
where
    T: Serialize,
{
    fn from(v: &Vec<T>) -> Self {
        iter_to_data(v.iter())
    }
}

impl<T> From<&Vec<T>> for RemovableValue<UrlData>
where
    T: Serialize,
{
    fn from(v: &Vec<T>) -> Self {
        RemovableValue::Specified(v.into())
    }
}
