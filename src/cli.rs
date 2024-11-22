use arg::Args;

use core::time;
use core::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Method {
    Head,
    Get,
    Patch,
    Post,
    Put,
    Delete
}

impl Method {
    #[inline(always)]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Head => "HEAD",
            Self::Get => "GET",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
        }
    }
}

impl FromStr for Method {
    type Err = &'static str;

    #[inline]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.eq_ignore_ascii_case("head") {
            Ok(Self::Head)
        } else if text.eq_ignore_ascii_case("get") {
            Ok(Self::Get)
        } else if text.eq_ignore_ascii_case("patch") {
            Ok(Self::Patch)
        } else if text.eq_ignore_ascii_case("post") {
            Ok(Self::Post)
        } else if text.eq_ignore_ascii_case("put") {
            Ok(Self::Put)
        } else if text.eq_ignore_ascii_case("delete") {
            Ok(Self::Delete)
        } else {
            Err("Invalid HTTP method. Allowed: HEAD, GET, PATCH, POST, PUT, DELETE")
        }
    }
}

#[derive(Args, Debug)]
///Utility to download text of the kakuyomu novels
pub struct Cli {
    #[arg(long, default_value = "Method::Get")]
    ///HTTP method to use. Defaults to GET.
    pub method: Method,
    #[arg(long, default_value = "10")]
    ///Timeout(sec) for every request. Defaults to 10 seconds.
    pub timeout: u64,
    #[arg(required)]
    ///URL against which to make request.
    pub url: String,
}

impl Cli {
    pub fn build_http(&self) -> ureq::Agent {
        let mut builder = ureq::AgentBuilder::new();

        if self.timeout != 0 {
            builder = builder.timeout(time::Duration::from_secs(self.timeout));
        }

        builder.user_agent("aget").build()
    }
}

#[inline]
pub fn args() -> Cli {
    arg::parse_args()
}
