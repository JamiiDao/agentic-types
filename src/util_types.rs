use std::collections::BTreeMap;

/// The `JsonStr` is a JSON &str that is not standardized
/// It is be domain specific
pub type JsonStr<'a> = &'a str;

pub type OpenIdExtension<'a> = BTreeMap<String, JsonStr<'a>>;
pub type JsonStrMemKV<'a> = BTreeMap<String, JsonStr<'a>>;
