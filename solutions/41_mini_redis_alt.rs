//! A tiny in-memory key/value store, Redis-flavored.
//!
//! Solution highlights:
//! - One `HashMap<String, Value>` instead of three sidecar maps — a key has one type at a time,
//!   so `SET` on an existing list now returns a `WRONGTYPE` error like real Redis.
//! - `Command::parse` does arity checking once, up front; the dispatcher matches on the enum
//!   instead of an `if/else` ladder over uppercased strings.
//! - `Reply` owns the wire format; `Display` is the single place that knows about `\r\n`.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    String(String),
    List(Vec<String>),
    Hash(HashMap<String, String>),
}

impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Hash(_) => "hash",
        }
    }
}

#[derive(Default)]
pub struct Db {
    pub store: HashMap<String, Value>,
}

pub enum Command {
    Set {
        key: String,
        value: String,
    },
    LPush {
        key: String,
        values: Vec<String>,
    },
    HSet {
        key: String,
        field: String,
        value: String,
    },
}

#[derive(Debug)]
pub enum CommandError {
    Empty,
    Unknown(String),
    WrongArity(&'static str),
}

impl Command {
    pub fn parse(args: &[&str]) -> Result<Self, CommandError> {
        let (cmd, rest) = args.split_first().ok_or(CommandError::Empty)?;
        match cmd.to_uppercase().as_str() {
            "SET" => match rest {
                [key, value] => Ok(Command::Set {
                    key: (*key).to_string(),
                    value: (*value).to_string(),
                }),
                _ => Err(CommandError::WrongArity("SET")),
            },
            "LPUSH" => match rest {
                [key, values @ ..] if !values.is_empty() => Ok(Command::LPush {
                    key: (*key).to_string(),
                    values: values.iter().map(|s| (*s).to_string()).collect(),
                }),
                _ => Err(CommandError::WrongArity("LPUSH")),
            },
            "HSET" => match rest {
                [key, field, value] => Ok(Command::HSet {
                    key: (*key).to_string(),
                    field: (*field).to_string(),
                    value: (*value).to_string(),
                }),
                _ => Err(CommandError::WrongArity("HSET")),
            },
            other => Err(CommandError::Unknown(other.to_string())),
        }
    }
}

pub enum Reply {
    Ok,
    Integer(i64),
    Error(String),
}

impl fmt::Display for Reply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reply::Ok => write!(f, "+OK\r\n"),
            Reply::Integer(n) => write!(f, ":{}\r\n", n),
            Reply::Error(msg) => write!(f, "-ERR {}\r\n", msg),
        }
    }
}

pub fn handle(db: &mut Db, args: &[&str]) -> String {
    let cmd = match Command::parse(args) {
        Ok(cmd) => cmd,
        Err(CommandError::Empty) => return Reply::Error("empty command".into()).to_string(),
        Err(CommandError::Unknown(name)) => {
            return Reply::Error(format!("unknown command '{}'", name)).to_string();
        }
        Err(CommandError::WrongArity(name)) => {
            return Reply::Error(format!("wrong number of arguments for '{}'", name)).to_string();
        }
    };
    execute(db, cmd).to_string()
}

fn execute(db: &mut Db, cmd: Command) -> Reply {
    match cmd {
        Command::Set { key, value } => match db.store.get(&key) {
            Some(v) if !matches!(v, Value::String(_)) => wrong_type(v.type_name()),
            _ => {
                db.store.insert(key, Value::String(value));
                Reply::Ok
            }
        },
        Command::LPush { key, values } => {
            let entry = db
                .store
                .entry(key)
                .or_insert_with(|| Value::List(Vec::new()));
            match entry {
                Value::List(list) => {
                    for v in values {
                        list.insert(0, v);
                    }
                    Reply::Integer(list.len() as i64)
                }
                other => wrong_type(other.type_name()),
            }
        }
        Command::HSet { key, field, value } => {
            let entry = db
                .store
                .entry(key)
                .or_insert_with(|| Value::Hash(HashMap::new()));
            match entry {
                Value::Hash(hash) => {
                    let is_new = !hash.contains_key(&field);
                    hash.insert(field, value);
                    Reply::Integer(if is_new { 1 } else { 0 })
                }
                other => wrong_type(other.type_name()),
            }
        }
    }
}

fn wrong_type(actual: &str) -> Reply {
    Reply::Error(format!(
        "WRONGTYPE Operation against a key holding the wrong kind of value (got {})",
        actual
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_writes_to_strings() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["SET", "x", "1"]), "+OK\r\n");
        assert!(matches!(db.store.get("x"), Some(Value::String(s)) if s == "1"));
    }

    #[test]
    fn lpush_writes_to_lists_in_reverse_order() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["LPUSH", "q", "a", "b"]), ":2\r\n");
        match db.store.get("q") {
            Some(Value::List(list)) => {
                assert_eq!(list, &vec!["b".to_string(), "a".to_string()]);
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn hset_writes_to_hashes() {
        let mut db = Db::default();
        assert_eq!(handle(&mut db, &["HSET", "u", "name", "ada"]), ":1\r\n");
        match db.store.get("u") {
            Some(Value::Hash(h)) => assert_eq!(h.get("name"), Some(&"ada".to_string())),
            other => panic!("expected hash, got {:?}", other),
        }
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
    fn set_on_existing_list_returns_wrongtype() {
        // The previous version silently kept both writes in their own sidecar maps.
        // With a single, typed store the second command must refuse.
        let mut db = Db::default();
        handle(&mut db, &["LPUSH", "x", "a"]);
        let reply = handle(&mut db, &["SET", "x", "1"]);
        assert!(reply.starts_with("-ERR WRONGTYPE"));
        assert!(matches!(db.store.get("x"), Some(Value::List(_))));
    }
}
