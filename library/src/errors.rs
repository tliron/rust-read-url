use problemo::{common::*, *};

tag_error!(UrlError, "URL");

string_attachment!(UrlAttachment);
string_attachment!(SchemeAttachment);
string_attachment!(FormatAttachment);

//
// UrlResult
//

/// URL result.
pub trait UrlResult<OkT> {
    /// Into a [Problem] via [UrlError] with [SchemeAttachment].
    fn into_url_problem(self, scheme: &str) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> UrlResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn into_url_problem(self, scheme: &str) -> Result<OkT, Problem> {
        self.into_problem().via(UrlError).map_with(|| SchemeAttachment::new(scheme))
    }
}

/// URL not found [Problem].
pub fn unreachable_url<ToStringT>(url: ToStringT, scheme: &str) -> Problem
where
    ToStringT: ToString,
{
    UnreachableError::new("URL")
        .into_problem()
        .with(UrlAttachment::new(url))
        .via(UrlError)
        .with(SchemeAttachment::new(scheme))
}
