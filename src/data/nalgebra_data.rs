use crate::data::base_data::*;
use crate::RemovableValue;
use crate::UrlData;
use nalgebra::base::storage::Storage;
use nalgebra::{Dim, Matrix as naMatrix, Scalar};
use serde::Serialize;
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
