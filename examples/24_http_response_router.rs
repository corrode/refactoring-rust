//! Decide what to do with an HTTP response based on its status code.

pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

pub enum Action {
    Ok(String),
    Retry,
    Fail(String),
}

pub fn handle_response(resp: Response) -> Action {
    if resp.status >= 100 && resp.status < 200 {
        println!("informational: {}", resp.status);
        return Action::Retry;
    } else if resp.status >= 200 && resp.status < 300 {
        let body = match String::from_utf8(resp.body) {
            Ok(s) => s,
            Err(_) => return Action::Fail("invalid utf-8".to_string()),
        };
        if resp.status == 204 {
            return Action::Ok(String::new());
        }
        println!("success: {}", resp.status);
        return Action::Ok(body);
    } else if resp.status >= 300 && resp.status < 400 {
        println!("redirect: {}", resp.status);
        return Action::Fail(format!("unhandled redirect {}", resp.status));
    } else if resp.status >= 400 && resp.status < 500 {
        if resp.status == 408 || resp.status == 429 {
            println!("client error, retrying: {}", resp.status);
            return Action::Retry;
        }
        println!("client error: {}", resp.status);
        return Action::Fail(format!("client error {}", resp.status));
    } else if resp.status >= 500 && resp.status < 600 {
        println!("server error, retrying: {}", resp.status);
        return Action::Retry;
    } else {
        println!("unknown status: {}", resp.status);
        return Action::Fail(format!("unknown status {}", resp.status));
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
