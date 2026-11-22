//! A tiny in-memory key/value store, Redis-flavored.
//!
//! Three commands: `SET key value`, `LPUSH key value [value ...]`, and
//! `HSET key field value`. One command per data type.
//!
//! A real implementation would also have GET, DEL, EXISTS, INCR, TYPE,
//! LRANGE, LLEN, HGET, HKEYS, HDEL, EXPIRE, and roughly thirty more.
//! Each one would slot into the if/else cascade below the same way the
//! three already there did. Imagine this file at 10x the size.
//!
//! The lesson is the *shape* of `handle`, not the command list. The
//! three commands here are enough to expose every problem the bigger
//! version would scale up linearly.
//!
//! Bonus ideas, all unlocked by the same refactor:
//!   - GET / DEL / EXISTS / TYPE  - each becomes one match arm.
//!   - INCR  - parse-or-create an integer; WRONGTYPE on a list.
//!   - EXPIRE / TTL  - wrap the value in an `Entry { value, expires_at }`.
//!   - RESP wire format  - a single `Display for Reply` impl.

use std::collections::HashMap;

#[derive(Default)]
pub struct Db {
    pub strings: HashMap<String, String>,
    pub lists: HashMap<String, Vec<String>>,
    pub hashes: HashMap<String, HashMap<String, String>>,
}

pub fn handle(db: &mut Db, args: &[&str]) -> String {
    if args.is_empty() {
        return "-ERR empty command\r\n".to_string();
    }
    let cmd = args[0].to_uppercase();
    if cmd == "SET" {
        if args.len() < 3 {
            return "-ERR wrong number of arguments\r\n".to_string();
        }
        let key = args[1].to_string();
        let value = args[2].to_string();
        db.strings.insert(key, value);
        return "+OK\r\n".to_string();
    } else if cmd == "LPUSH" {
        assert!(args.len() >= 3, "LPUSH needs key and at least one value");
        let key = args[1].to_string();
        let list = db.lists.entry(key).or_insert_with(Vec::new);
        for v in &args[2..] {
            list.insert(0, v.to_string());
        }
        return format!(":{}\r\n", list.len());
    } else if cmd == "HSET" {
        let key = args[1].to_string();
        let field = args[2].to_string();
        let value = args[3].to_string();
        let hash = db.hashes.entry(key).or_insert_with(HashMap::new);
        let is_new = !hash.contains_key(&field);
        hash.insert(field, value);
        return format!(":{}\r\n", if is_new { 1 } else { 0 });
    } else {
        return format!("-ERR unknown command '{}'\r\n", args[0]);
    }
}

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
}
