use crate::RemovableValue;
use crate::UrlData;
use crate::UrlDataBuilder;
use crate::UrlDataInlineDataset;
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
