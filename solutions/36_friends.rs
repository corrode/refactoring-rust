// ANCHOR: listing
//! One owner stores people; friendships store IDs rather than owning people.
//!
//! IDs remain valid as this append-only directory grows. Use an ID only with
//! the `People` that issued it: bounds checks do not distinguish directories.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersonId(usize);

#[derive(Debug, Default)]
pub struct People {
    people: Vec<Person>,
}

#[derive(Debug)]
pub struct Person {
    name: String,
    friends: Vec<PersonId>,
}

impl Person {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn friends(&self) -> &[PersonId] {
        &self.friends
    }
}

impl People {
    pub fn add(&mut self, name: &str) -> PersonId {
        let id = PersonId(self.people.len());
        self.people.push(Person {
            name: name.to_owned(),
            friends: Vec::new(),
        });
        id
    }

    pub fn get(&self, id: PersonId) -> Option<&Person> {
        self.people.get(id.0)
    }

    pub fn add_friend(&mut self, person: PersonId, friend: PersonId) -> Option<()> {
        self.people.get(friend.0)?;
        let entry = self.people.get_mut(person.0)?;
        if person != friend {
            entry.friends.push(friend);
        }
        Some(())
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_person_has_name_and_no_friends() {
        let mut people = People::default();
        for name in ["", "Alice"] {
            let id = people.add(name);
            let person = people.get(id).unwrap();
            assert_eq!(person.name(), name);
            assert!(person.friends().is_empty());
        }
    }

    #[test]
    fn own_id_is_a_noop() {
        let mut people = People::default();
        let alice = people.add("Alice");
        assert_eq!(people.add_friend(alice, alice), Some(()));
        assert!(people.get(alice).unwrap().friends().is_empty());

        let bob = people.add("Bob");
        assert_eq!(people.add_friend(alice, bob), Some(()));
        assert_eq!(people.add_friend(alice, alice), Some(()));
        assert_eq!(people.get(alice).unwrap().friends(), &[bob]);
    }

    #[test]
    fn equal_names_have_distinct_ids() {
        let mut people = People::default();
        let alice = people.add("Alice");
        let other_alice = people.add("Alice");
        assert_ne!(alice, other_alice);
        assert_eq!(people.add_friend(alice, other_alice), Some(()));
        assert_eq!(people.get(alice).unwrap().friends(), &[other_alice]);
        assert!(people.get(other_alice).unwrap().friends().is_empty());
    }

    #[test]
    fn friendships_are_directed_ordered_and_allow_duplicates() {
        let mut people = People::default();
        let alice = people.add("Alice");
        let bob = people.add("Bob");
        let carol = people.add("Carol");
        for friend in [bob, carol, bob] {
            assert_eq!(people.add_friend(alice, friend), Some(()));
        }
        assert_eq!(people.get(alice).unwrap().friends(), &[bob, carol, bob]);
        assert!(people.get(bob).unwrap().friends().is_empty());
        assert!(people.get(carol).unwrap().friends().is_empty());
    }

    #[test]
    fn mutual_friendships() {
        let mut people = People::default();
        let alice = people.add("Alice");
        let bob = people.add("Bob");
        assert_eq!(people.add_friend(alice, bob), Some(()));
        assert_eq!(people.add_friend(bob, alice), Some(()));
        assert_eq!(people.get(alice).unwrap().friends(), &[bob]);
        assert_eq!(people.get(bob).unwrap().friends(), &[alice]);
    }

    #[test]
    fn ids_survive_vector_growth() {
        let mut people = People::default();
        let alice = people.add("Alice");
        let bob = people.add("Bob");
        assert_eq!(people.add_friend(alice, bob), Some(()));
        let capacity = people.people.capacity();
        for _ in 0..=capacity {
            people.add("Extra");
        }
        assert!(people.people.capacity() > capacity);
        assert_eq!(people.get(alice).unwrap().name(), "Alice");
        assert_eq!(people.get(bob).unwrap().name(), "Bob");
        assert_eq!(people.get(alice).unwrap().friends(), &[bob]);
        assert_eq!(people.add_friend(bob, alice), Some(()));
        assert_eq!(people.get(bob).unwrap().friends(), &[alice]);
    }

    #[test]
    fn invalid_ids_are_rejected_before_mutation() {
        let mut people = People::default();
        assert!(people.get(PersonId(0)).is_none());
        assert_eq!(people.add_friend(PersonId(0), PersonId(0)), None);
        assert!(people.people.is_empty());

        let alice = people.add("Alice");
        let bob = people.add("Bob");
        assert_eq!(people.add_friend(alice, bob), Some(()));
        // Only this module can fabricate IDs; callers must use People::add.
        for invalid in [PersonId(2), PersonId(usize::MAX)] {
            assert!(people.get(invalid).is_none());
            for (person, friend) in [(invalid, bob), (alice, invalid), (invalid, invalid)] {
                assert_eq!(people.add_friend(person, friend), None);
                assert_eq!(people.people.len(), 2);
                assert_eq!(people.get(alice).unwrap().friends(), &[bob]);
                assert!(people.get(bob).unwrap().friends().is_empty());
            }
        }
    }

    #[test]
    fn ids_do_not_identify_their_directory() {
        let mut first = People::default();
        let alice = first.add("Alice");
        let bob = first.add("Bob");
        let mut second = People::default();
        let carol = second.add("Carol");
        let dave = second.add("Dave");

        // A foreign ID whose index is in range is not detected.
        assert_eq!(alice, carol);
        assert_eq!(second.get(alice).unwrap().name(), "Carol");
        assert_eq!(second.add_friend(alice, bob), Some(()));
        assert_eq!(second.get(carol).unwrap().friends(), &[dave]);
        assert!(first.get(alice).unwrap().friends().is_empty());
    }
}
// ANCHOR_END: tests

// Teaching source: Jack O'Connor, https://jacko.io/object_soup.html
// This is a teaching example, not a production refactor.
