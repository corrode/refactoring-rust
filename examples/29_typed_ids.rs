// ANCHOR: listing
//! Review the IDs used to look up variables and declarations.
//!
//! You may change the public API. Keep successful lookups and the `None`
//! result for missing entries, and adapt callers and tests as needed.

pub type VarId = usize;
pub type DeclId = usize;

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
        self.variables.get(id).map(String::as_str)
    }

    pub fn declaration(&self, id: DeclId) -> Option<&str> {
        self.declarations.get(id).map(String::as_str)
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
        let variable: VarId = 1;
        let declaration: DeclId = 0;
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
                assert_eq!(engine.variable(id), None);
                assert_eq!(engine.declaration(id), None);
            }
        }
        let empty = Engine::new(vec![], vec![]);
        assert_eq!(empty.variable(0), None);
        assert_eq!(empty.declaration(0), None);
    }

    // Expected to fail until the ID categories have distinct types.
    #[test]
    fn variable_and_declaration_ids_are_distinct() {
        assert_ne!(
            std::any::TypeId::of::<VarId>(),
            std::any::TypeId::of::<DeclId>(),
            "variable and declaration IDs must be distinct types"
        );
    }
}
// ANCHOR_END: tests

// Provenance: workshop-created Engine fixture, inspired by Nushell PR #13832.
// https://github.com/nushell/nushell/pull/13832
// Nushell uses generic Id<T> with PhantomData, not this Engine implementation.
// Project: https://github.com/nushell/nushell
// Pinned source: https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L7-L33
// Upstream MIT notice: ../THIRD_PARTY_NOTICES.md (Nushell).
