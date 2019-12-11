use crate::data::base_data::*;
use crate::RemovableValue;
use crate::UrlData;
use rulinalg::matrix::{BaseMatrix, Matrix};
use serde::Serialize;
impl<T> From<Matrix<T>> for UrlData
where
    T: Serialize,
{
    fn from(v: Matrix<T>) -> Self {
        iter_to_data(v.row_iter().map(|row| row.raw_slice()))
    }
}
impl<T> From<Matrix<T>> for RemovableValue<UrlData>
where
    T: Serialize,
{
    fn from(v: Matrix<T>) -> Self {
        RemovableValue::Specified(v.into())
    }
}
