use crate::data::base_data::*;
use crate::RemovableValue;
use crate::UrlData;
use ndarray::ArrayBase;
use serde::Serialize;
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
