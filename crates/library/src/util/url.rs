#![allow(unused)]

use super::super::url::UrlQuery;

use {
    std::{borrow::*, collections::*},
    url::*,
};

/// URL query and fragment.
pub fn url_query_and_fragment(url: &Url) -> (Option<UrlQuery>, Option<String>) {
    (url_query(url), url_fragment(url))
}

/// URL query.
pub fn url_query(url: &Url) -> Option<UrlQuery> {
    url_query_from(url.query_pairs())
}

/// URL query.
pub fn url_query_from<'own, IteratorT>(pairs: IteratorT) -> Option<UrlQuery>
where
    IteratorT: Iterator<Item = (Cow<'own, str>, Cow<'own, str>)>,
{
    let mut query = UrlQuery::new();
    for (k, v) in pairs {
        query.insert(k.into(), v.into());
    }
    if query.is_empty() { None } else { Some(query) }
}

/// URL query string.
pub fn url_query_string(query: &Option<UrlQuery>) -> String {
    if let Some(query) = query {
        if !query.is_empty() {
            let pairs: Vec<String> = query.into_iter().map(|(key, value)| key.to_owned() + "=" + value).collect();
            return "?".to_string() + &pairs.join("&");
        }
    }

    String::new()
}

/// URL fragment.
pub fn url_fragment(url: &Url) -> Option<String> {
    url.fragment().filter(|f| !f.is_empty()).map(|fragment| fragment.into())
}

/// URL fragment string.
pub fn url_fragment_string(fragment: &Option<String>) -> String {
    fragment.as_ref().map_or(String::new(), |fragment| String::from("#") + fragment)
}
