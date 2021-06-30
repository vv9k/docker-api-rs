use std::{borrow::Borrow, string::ToString};
use url::form_urlencoded;

pub fn url_encoded_pair<K, V>(key: K, val: V) -> String
where
    K: AsRef<str> + 'static,
    V: ToString,
{
    form_urlencoded::Serializer::new(String::new())
        .append_pair(key.as_ref(), &val.to_string())
        .finish()
}

pub fn url_encoded_pairs<I, K, V>(iter: I) -> String
where
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    form_urlencoded::Serializer::new(String::new())
        .extend_pairs(iter)
        .finish()
}
