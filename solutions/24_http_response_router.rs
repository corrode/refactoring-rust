//! Decide what to do with an HTTP response based on its status code.
//!
//! Solution highlights:
//! - Split "what class is this status?" from "what do we do?" with a `StatusClass` enum.
//! - `classify` is the only place that knows the numeric ranges.
//! - `handle_response` becomes a flat `match` — one arm per policy decision.
//! - Adding a new class or special-case status now changes exactly one place.

pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

pub enum Action {
    Ok(String),
    Retry,
    Fail(String),
}

enum StatusClass {
    Informational,
    Success,
    Redirect,
    ClientError,
    ServerError,
    Unknown,
}

impl StatusClass {
    fn classify(status: u16) -> Self {
        match status {
            100..=199 => Self::Informational,
            200..=299 => Self::Success,
            300..=399 => Self::Redirect,
            400..=499 => Self::ClientError,
            500..=599 => Self::ServerError,
            _ => Self::Unknown,
        }
    }
}

pub fn handle_response(resp: Response) -> Action {
    use StatusClass::*;
    match StatusClass::classify(resp.status) {
        Success => match String::from_utf8(resp.body) {
            Ok(_) if resp.status == 204 => Action::Ok(String::new()),
            Ok(s) => Action::Ok(s),
            Err(_) => Action::Fail("invalid utf-8".into()),
        },
        ServerError => Action::Retry,
        ClientError if matches!(resp.status, 408 | 429) => Action::Retry,
        ClientError => Action::Fail(format!("client error {}", resp.status)),
        Informational => Action::Retry,
        Redirect => Action::Fail(format!("unhandled redirect {}", resp.status)),
        Unknown => Action::Fail(format!("unknown status {}", resp.status)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_returns_body() {
        let r = Response {
            status: 200,
            body: b"hi".to_vec(),
        };
        assert!(matches!(handle_response(r), Action::Ok(s) if s == "hi"));
    }

    #[test]
    fn server_error_retries() {
        let r = Response {
            status: 503,
            body: vec![],
        };
        assert!(matches!(handle_response(r), Action::Retry));
    }

    #[test]
    fn rate_limited_retries() {
        let r = Response {
            status: 429,
            body: vec![],
        };
        assert!(matches!(handle_response(r), Action::Retry));
    }

    #[test]
    fn not_found_fails() {
        let r = Response {
            status: 404,
            body: vec![],
        };
        assert!(matches!(handle_response(r), Action::Fail(_)));
    }
}
