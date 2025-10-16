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
    #[track_caller]
    fn into_url_problem(self, scheme: &str) -> Result<OkT, Problem> {
        match self.into_problem() {
            Ok(ok) => Ok(ok),
            Err(problem) => Err(problem.via(UrlError).with(SchemeAttachment::new(scheme))),
        }
    }
}

/// URL not found [Problem].
pub fn unreachable_url<ToStringT>(url: ToStringT, scheme: &str) -> Problem
where
    ToStringT: ToString,
{
    UnreachableError::as_problem("URL").with(UrlAttachment::new(url)).via(UrlError).with(SchemeAttachment::new(scheme))
}
