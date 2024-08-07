use serde::{Serialize, Serializer};

pub fn serialize_option<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + Default,
{
    match value {
        Some(v) => serializer.serialize_some(v),
        None => serializer.serialize_some(&T::default()),
    }
}
