// ANCHOR: listing
//! Record directed friendships between people.
//!
//! You may change the public API. Keep directed friendships, insertion order,
//! and duplicate additions. Self-friendship should do nothing, and distinct
//! people with the same name should be able to be friends.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Person {
    name: String,
    friends: Vec<Rc<RefCell<Person>>>,
}

impl Person {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            friends: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, other: &Rc<RefCell<Person>>) {
        if other.borrow().name != self.name {
            self.friends.push(Rc::clone(other));
        }
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_person_has_name_and_no_friends() {
        for name in ["", "Alice"] {
            let person = Person::new(name);
            assert_eq!(person.name, name);
            assert!(person.friends.is_empty());
        }
    }

    // Expected to fail: the name check borrows an already mutably borrowed person.
    #[test]
    fn self_friendship_is_a_noop() {
        let alice = Rc::new(RefCell::new(Person::new("Alice")));
        alice.borrow_mut().add_friend(&alice);
        assert!(alice.borrow().friends.is_empty());
    }

    // Expected to fail: a name is not an identity.
    #[test]
    fn people_with_same_name_are_distinct() {
        let alice = Rc::new(RefCell::new(Person::new("Alice")));
        let other_alice = Rc::new(RefCell::new(Person::new("Alice")));
        alice.borrow_mut().add_friend(&other_alice);
        assert_eq!(
            alice.borrow().friends.len(),
            1,
            "distinct people with the same name must be able to be friends"
        );
        assert!(Rc::ptr_eq(&alice.borrow().friends[0], &other_alice));
        assert!(other_alice.borrow().friends.is_empty());
    }

    #[test]
    fn friendships_are_directed_ordered_and_allow_duplicates() {
        let alice = Rc::new(RefCell::new(Person::new("Alice")));
        let bob = Rc::new(RefCell::new(Person::new("Bob")));
        let carol = Rc::new(RefCell::new(Person::new("Carol")));
        for friend in [&bob, &carol, &bob] {
            alice.borrow_mut().add_friend(friend);
        }
        let alice = alice.borrow();
        assert_eq!(alice.friends.len(), 3);
        for (actual, expected) in alice.friends.iter().zip([&bob, &carol, &bob]) {
            assert!(Rc::ptr_eq(actual, expected));
        }
        assert!(bob.borrow().friends.is_empty());
        assert!(carol.borrow().friends.is_empty());
    }

    #[test]
    fn mutual_friendships() {
        let alice = Rc::new(RefCell::new(Person::new("Alice")));
        let bob = Rc::new(RefCell::new(Person::new("Bob")));
        alice.borrow_mut().add_friend(&bob);
        bob.borrow_mut().add_friend(&alice);
        assert_eq!(alice.borrow().friends.len(), 1);
        assert_eq!(bob.borrow().friends.len(), 1);
        assert!(Rc::ptr_eq(&alice.borrow().friends[0], &bob));
        assert!(Rc::ptr_eq(&bob.borrow().friends[0], &alice));

        // Break the strong cycle so this test does not leak either person.
        alice.borrow_mut().friends.clear();
        bob.borrow_mut().friends.clear();
    }
}
// ANCHOR_END: tests

// Teaching source: Jack O'Connor, https://jacko.io/object_soup.html
// This is a teaching example, not a production refactor.
