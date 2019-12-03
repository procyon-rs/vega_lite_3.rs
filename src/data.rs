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

#[cfg(feature = "ndarray")]
use ndarray::ArrayBase;

#[cfg(feature = "ndarray")]
impl<A, D, S> From<ArrayBase<S, D>> for UrlData
where
    A: Serialize,
    D: ndarray::Dimension,
    S: ndarray::Data<Elem = A>,
{
    fn from(v: ArrayBase<S, D>) -> Self {
        iter_to_data(v.genrows().into_iter())
    }
}

#[cfg(feature = "ndarray")]
impl<A, D, S> From<ArrayBase<S, D>> for RemovableValue<UrlData>
where
    A: Serialize,
    D: ndarray::Dimension,
    S: ndarray::Data<Elem = A>,
{
    fn from(v: ArrayBase<S, D>) -> Self {
        RemovableValue::Specified(v.into())
    }
}

#[cfg(feature = "rulinalg")]
use rulinalg::matrix::{BaseMatrix, Matrix};
#[cfg(feature = "rulinalg")]
impl<T> From<Matrix<T>> for UrlData
where
    T: Serialize,
{
    fn from(v: Matrix<T>) -> Self {
        iter_to_data(v.row_iter().map(|row| row.raw_slice()))
    }
}
#[cfg(feature = "rulinalg")]
impl<T> From<Matrix<T>> for RemovableValue<UrlData>
where
    T: Serialize,
{
    fn from(v: Matrix<T>) -> Self {
        RemovableValue::Specified(v.into())
    }
}
#[cfg(feature = "nalgebra")]
use nalgebra::base::storage::Storage;
#[cfg(feature = "nalgebra")]
use nalgebra::{Dim, Matrix as naMatrix, Scalar};

#[cfg(feature = "nalgebra")]
impl<N, R, C, S> From<naMatrix<N, R, C, S>> for UrlData
where
    N: Scalar + PartialOrd + Serialize,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
{
    fn from(v: naMatrix<N, R, C, S>) -> Self {
        let strides = v.strides();
        iter_to_data(v.row_iter().map(|row| {
            row.data
                .as_slice()
                .iter()
                .cloned()
                .enumerate()
                .filter(|(i, _)| i % strides.1 == 0)
                .map(|(_, v)| v)
                .collect::<Vec<_>>()
        }))
    }
}

#[cfg(feature = "nalgebra")]
impl<N, R, C, S> From<naMatrix<N, R, C, S>> for RemovableValue<UrlData>
where
    N: Scalar + PartialOrd + Serialize,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
{
    fn from(v: naMatrix<N, R, C, S>) -> Self {
        RemovableValue::Specified(v.into())
    }
}

#[cfg(feature = "csv")]
use csv::Reader;

#[cfg(feature = "csv")]
impl<R> From<Reader<R>> for UrlData
where
    R: std::io::Read,
{
    fn from(mut v: Reader<R>) -> Self {
        UrlDataBuilder::default()
            .values(UrlDataInlineDataset::UnionArray(
                v.records()
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

#[cfg(feature = "csv")]
impl<R> From<Reader<R>> for RemovableValue<UrlData>
where
    R: std::io::Read,
{
    fn from(v: Reader<R>) -> Self {
        RemovableValue::Specified(v.into())
    }
}
