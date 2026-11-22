// ANCHOR: listing
//! Distinguish variable IDs from declaration IDs at the lookup boundary.
//!
//! These IDs identify a category, not a valid index or a particular engine.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeclId(pub usize);

pub struct Engine {
    variables: Vec<String>,
    declarations: Vec<String>,
}

impl Engine {
    pub fn new(variables: Vec<String>, declarations: Vec<String>) -> Self {
        Self {
            variables,
            declarations,
        }
    }

    pub fn variable(&self, id: VarId) -> Option<&str> {
        self.variables.get(id.0).map(String::as_str)
    }

    pub fn declaration(&self, id: DeclId) -> Option<&str> {
        self.declarations.get(id.0).map(String::as_str)
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_up_both_categories() {
        let engine = Engine::new(vec!["count".into(), "name".into()], vec!["print".into()]);
        let variable = VarId(1);
        let declaration = DeclId(0);
        assert_eq!(engine.variable(variable), Some("name"));
        assert_eq!(engine.declaration(declaration), Some("print"));
    }

    #[test]
    fn empty_and_out_of_range_lookups_return_none() {
        for engine in [
            Engine::new(vec![], vec![]),
            Engine::new(vec!["count".into()], vec!["print".into()]),
        ] {
            for id in [1, usize::MAX] {
                assert_eq!(engine.variable(VarId(id)), None);
                assert_eq!(engine.declaration(DeclId(id)), None);
            }
        }
        let empty = Engine::new(vec![], vec![]);
        assert_eq!(empty.variable(VarId(0)), None);
        assert_eq!(empty.declaration(DeclId(0)), None);
    }

    #[test]
    fn ids_have_distinct_types() {
        assert_ne!(
            std::any::TypeId::of::<VarId>(),
            std::any::TypeId::of::<DeclId>()
        );
        assert_eq!(VarId(0), VarId(0));
        assert_eq!(DeclId(0), DeclId(0));
    }

    #[test]
    fn ids_are_not_tied_to_an_engine() {
        let first = Engine::new(vec!["count".into()], vec!["print".into()]);
        let second = Engine::new(vec!["name".into()], vec!["save".into()]);
        let variable = VarId(0);
        let declaration = DeclId(0);
        assert_eq!(first.variable(variable), Some("count"));
        assert_eq!(second.variable(variable), Some("name"));
        assert_eq!(first.declaration(declaration), Some("print"));
        assert_eq!(second.declaration(declaration), Some("save"));
    }
}
// ANCHOR_END: tests

// Provenance: workshop-created Engine fixture, inspired by Nushell PR #13832.
// https://github.com/nushell/nushell/pull/13832
// Separate newtypes are a teaching simplification of Nushell's Id<T>/PhantomData.
// Project: https://github.com/nushell/nushell
// Pinned source: https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L7-L33
// Upstream MIT notice: ../THIRD_PARTY_NOTICES.md (Nushell).
