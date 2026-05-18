//! Decide whether a user can access a given path.

pub fn can_access(path: &str, is_authenticated: bool, is_admin: bool) -> bool {
    if path.starts_with("/public") {
        return true;
    }
    if is_admin {
        if is_authenticated {
            return true;
        } else {
            return false;
        }
    }
    if path.starts_with("/admin") {
        return false;
    }
    if is_authenticated {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_is_open() {
        assert!(can_access("/public/x", false, false));
    }

    #[test]
    fn admin_path_requires_admin() {
        assert!(!can_access("/admin/users", true, false));
        assert!(can_access("/admin/users", true, true));
    }

    #[test]
    fn private_path_requires_login() {
        assert!(!can_access("/dashboard", false, false));
        assert!(can_access("/dashboard", true, false));
    }
}
