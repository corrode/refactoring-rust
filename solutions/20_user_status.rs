//! Decide whether a user can access a given path.
//!
//! Solution highlights:
//! - Two booleans collapse into three real states; the impossible
//!   "unauthenticated admin" no longer exists.
//! - The decision is one flat `match`, read top to bottom.
//! - Adding a new role is "add a variant, handle it" -- the compiler tells
//!   you where.

pub enum UserStatus {
    Anonymous,
    Authenticated,
    Admin,
}

pub fn can_access(path: &str, status: UserStatus) -> bool {
    match (path, status) {
        (p, _) if p.starts_with("/public") => true,
        (p, UserStatus::Admin) if p.starts_with("/admin") => true,
        (p, _) if p.starts_with("/admin") => false,
        (_, UserStatus::Anonymous) => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_is_open() {
        assert!(can_access("/public/x", UserStatus::Anonymous));
    }

    #[test]
    fn admin_path_requires_admin() {
        assert!(!can_access("/admin/users", UserStatus::Authenticated));
        assert!(can_access("/admin/users", UserStatus::Admin));
    }

    #[test]
    fn private_path_requires_login() {
        assert!(!can_access("/dashboard", UserStatus::Anonymous));
        assert!(can_access("/dashboard", UserStatus::Authenticated));
    }
}
