//! Refactor dispatch while preserving the starter's public fields and behavior.
//!
//! The independent maps still allow a key to hold multiple types. Arity behavior
//! is also unchanged: SET and HSET ignore extra arguments, short SET returns an
//! error, and short LPUSH or HSET panics.
//!
//! `35_mini_redis_alt.rs` preserves the API-changing redesign: one typed store,
//! parsed commands, centralized replies, cross-type rejection, and arity errors.
//! Those are separate contract changes, not part of this compatibility refactor.
//! Despite the alternative's original comment, real Redis SET replaces existing
//! values of other types; rejecting them is that alternative's own policy.

// ANCHOR: listing
use std::collections::HashMap;

#[derive(Default)]
pub struct Db {
    pub strings: HashMap<String, String>,
    pub lists: HashMap<String, Vec<String>>,
    pub hashes: HashMap<String, HashMap<String, String>>,
}

pub fn handle(db: &mut Db, args: &[&str]) -> String {
    let Some((command, rest)) = args.split_first() else {
        return "-ERR empty command\r\n".to_string();
    };

    match command.to_uppercase().as_str() {
        "SET" => {
            let [key, value, ..] = rest else {
                return "-ERR wrong number of arguments\r\n".to_string();
            };
            db.strings.insert((*key).to_string(), (*value).to_string());
            "+OK\r\n".to_string()
        }
        "LPUSH" => {
            assert!(args.len() >= 3, "LPUSH needs key and at least one value");
            let list = db.lists.entry(rest[0].to_string()).or_default();
            for value in &rest[1..] {
                list.insert(0, (*value).to_string());
            }
            format!(":{}\r\n", list.len())
        }
        "HSET" => {
            // Keep the starter's indexing (and panic) before any database write.
            let key = args[1].to_string();
            let field = args[2].to_string();
            let value = args[3].to_string();
            let hash = db.hashes.entry(key).or_default();
            let is_new = hash.insert(field, value).is_none();
            format!(":{}\r\n", u8::from(is_new))
        }
        _ => format!("-ERR unknown command '{}'\r\n", command),
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_writes_to_strings() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["SET", "x", "1"]), "+OK\r\n");
        assert_eq!(db.strings.get("x"), Some(&"1".to_string()));
    }

    #[test]
    fn lpush_writes_to_lists_in_reverse_order() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["LPUSH", "q", "a", "b"]), ":2\r\n");
        assert_eq!(
            db.lists.get("q").unwrap(),
            &vec!["b".to_string(), "a".to_string()]
        );
    }

    #[test]
    fn hset_writes_to_hashes() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["HSET", "u", "name", "ada"]), ":1\r\n");
        assert_eq!(
            db.hashes.get("u").unwrap().get("name"),
            Some(&"ada".to_string())
        );
    }

    #[test]
    fn hset_returns_zero_on_field_update() {
        let mut db = Db::default();
        handle(&mut db, &["HSET", "u", "name", "ada"]);
        assert_eq!(handle(&mut db, &["HSET", "u", "name", "grace"]), ":0\r\n");
    }

    #[test]
    fn unknown_command_returns_error() {
        let mut db = Db::default();
        let reply = handle(&mut db, &["FROBNICATE", "x"]);
        assert!(reply.starts_with("-ERR"));
    }

    #[test]
    fn set_and_lpush_on_same_key_both_succeed_silently() {
        // Documents the bug: each command writes to its own sidecar map,
        // so the same key can be a string AND a list at the same time.
        // After the refactor this test will need to change: one of the two
        // commands must return a WRONGTYPE error.
        let mut db = Db::default();
        handle(&mut db, &["LPUSH", "x", "a"]);
        handle(&mut db, &["SET", "x", "1"]);
        assert!(db.strings.contains_key("x"));
        assert!(db.lists.contains_key("x"));
    }

    #[test]
    fn original_public_api_supports_struct_literals_and_direct_mutation() {
        let handle: fn(&mut Db, &[&str]) -> String = super::handle;
        let mut db = Db {
            strings: HashMap::<String, String>::new(),
            lists: HashMap::<String, Vec<String>>::new(),
            hashes: HashMap::<String, HashMap<String, String>>::new(),
        };
        db.strings.insert("x".into(), "old".into());
        db.lists.insert("x".into(), vec!["old".into()]);
        db.hashes
            .insert("x".into(), HashMap::from([("field".into(), "old".into())]));
        assert_eq!(handle(&mut db, &["SET", "x", "new"]), "+OK\r\n");
        assert_eq!(handle(&mut db, &["LPUSH", "x", "new"]), ":2\r\n");
        assert_eq!(handle(&mut db, &["HSET", "x", "field", "new"]), ":0\r\n");
        let Db {
            strings,
            lists,
            hashes,
        } = db;
        assert_eq!(strings["x"], "new");
        assert_eq!(lists["x"], ["new", "old"]);
        assert_eq!(hashes["x"]["field"], "new");
    }

    #[test]
    fn exact_error_replies_and_command_case_are_preserved() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &[]), "-ERR empty command\r\n");
        for args in [&["SET"][..], &["set", "x"]] {
            assert_eq!(handle(&mut db, args), "-ERR wrong number of arguments\r\n");
        }
        assert_eq!(
            handle(&mut db, &["froB"]),
            "-ERR unknown command 'froB'\r\n"
        );
        assert_eq!(handle(&mut db, &[""]), "-ERR unknown command ''\r\n");
        assert_eq!(handle(&mut db, &["sEt", "", ""]), "+OK\r\n");
        assert_eq!(handle(&mut db, &["lPush", "q", ""]), ":1\r\n");
        assert_eq!(handle(&mut db, &["hSet", "h", "", ""]), ":1\r\n");
    }

    #[test]
    fn set_and_hset_ignore_extra_arguments() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["SET", "x", "1", "ignored"]), "+OK\r\n");
        assert_eq!(db.strings["x"], "1");
        assert_eq!(
            handle(&mut db, &["HSET", "h", "f", "v", "ignored"]),
            ":1\r\n"
        );
        assert_eq!(db.hashes["h"]["f"], "v");
    }

    #[test]
    fn short_lpush_and_hset_still_panic_without_mutating_the_database() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        for args in [
            &["LPUSH"][..],
            &["LPUSH", "x"],
            &["HSET"],
            &["HSET", "x"],
            &["HSET", "x", "field"],
        ] {
            let mut db = Db::default();
            db.strings.insert("existing".into(), "value".into());
            let result = catch_unwind(AssertUnwindSafe(|| handle(&mut db, args)));
            assert!(result.is_err(), "{args:?}");
            assert_eq!(
                db.strings,
                HashMap::from([("existing".into(), "value".into())])
            );
            assert!(db.lists.is_empty());
            assert!(db.hashes.is_empty());
        }
    }

    #[test]
    #[should_panic(expected = "LPUSH needs key and at least one value")]
    fn lpush_keeps_its_assertion_message() {
        handle(&mut Db::default(), &["LPUSH", "x"]);
    }
}
// ANCHOR_END: tests
