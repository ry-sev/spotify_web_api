use super::error::BodyError;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::borrow::Cow;
use url::Url;

/// A trait representing a parameter value.
pub trait ParamValue<'a> {
    #[allow(clippy::wrong_self_convention)]
    /// The parameter value as a string.
    fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
    fn as_value(&self) -> Cow<'static, str> {
        if *self { "true".into() } else { "false".into() }
    }
}

impl<'a> ParamValue<'a> for &'a str {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl ParamValue<'static> for String {
    fn as_value(&self) -> Cow<'static, str> {
        self.clone().into()
    }
}

impl<'a> ParamValue<'a> for &'a String {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
    fn as_value(&self) -> Self {
        self.clone()
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}

impl ParamValue<'static> for u8 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for u32 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for u64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for i64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for DateTime<Utc> {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .into()
    }
}

impl<'a> ParamValue<'a> for &'a crate::model::AlbumType {
    fn as_value(&self) -> Cow<'a, str> {
        self.to_string().into()
    }
}

impl<'a> ParamValue<'a> for &'a crate::model::TimeRange {
    fn as_value(&self) -> Cow<'a, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for crate::model::FollowedArtistsType {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for crate::model::FollowType {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl<'a> ParamValue<'a> for &'a crate::model::IncludeExternalType {
    fn as_value(&self) -> Cow<'a, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for crate::model::RepeatState {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for crate::model::PlaylistItem {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

/// A structure for form parameters.
#[derive(Debug, Default, Clone)]
pub struct FormParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> FormParams<'a> {
    /// Push a single parameter.
    pub fn push<'b, K, V>(&mut self, key: K, value: &V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Encode the parameters into a request body.
    pub fn into_body(self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let body = serde_urlencoded::to_string(self.params)?;
        Ok(Some((
            "application/x-www-form-urlencoded",
            body.into_bytes(),
        )))
    }
}

/// A structure for JSON parameters.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct JsonParams {}

impl JsonParams {
    /// Clean a JSON value for submission.
    ///
    /// Removes `null` and empty array values from top-level objects.
    pub fn clean(mut val: Value) -> Value {
        if let Some(obj) = val.as_object_mut() {
            obj.retain(|_, v| {
                !v.is_null()
                    && v.as_array().is_none_or(|a| !a.is_empty())
                    && v.as_object().is_none_or(|o| !o.is_empty())
            });
        }

        val
    }

    /// Encode the parameters into a request body.
    pub fn into_body(input: &Value) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let body = serde_json::to_string(input)?;
        Ok(Some(("application/json", body.into_bytes())))
    }
}

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push<'b, K, V>(&mut self, key: K, value: &V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn bool_str() {
        let items = &[(true, "true"), (false, "false")];

        for (i, s) in items {
            assert_eq!((*i).as_value(), *s);
        }
    }

    #[test]
    fn test_str_as_value() {
        let items = &["foo", "bar"];

        for i in items {
            assert_eq!(i.as_value(), *i);
        }
    }

    #[test]
    fn test_string_as_value() {
        let items = &["foo", "bar"];

        for i in items {
            let s = String::from(*i);
            assert_eq!(s.as_value(), s);
        }
    }

    #[test]
    fn json_params_clean() {
        let dirty = json!({
            "null": null,
            "int": 1,
            "str": "str",
            "array": [null],
            "empty_array": [],
            "object": {
                "nested_null": null,
                "nested_empty_array": [],
                "nested_empty_object": {},
            },
            "empty_object": {},
        });

        let clean = json!({
            "int": 1,
            "str": "str",
            "array": [null],
            "object": {
                "nested_null": null,
                "nested_empty_array": [],
                "nested_empty_object": {},
            },
        });

        assert_eq!(JsonParams::clean(dirty), clean);
    }
}
