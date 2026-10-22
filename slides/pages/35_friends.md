---
layout: default
zoom: 0.85
---

# 35 · `friends` (advanced)

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
    friends: Vec<Rc<RefCell<Person>>>,
}

impl Person {
    fn new(name: &str) -> Self {
        Self { name: name.to_owned(), friends: Vec::new() }
    }

    fn add_friend(&mut self, other: &Rc<RefCell<Person>>) {
        if other.borrow().name != self.name {
            self.friends.push(Rc::clone(other));
        }
    }
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Adapted from Jack O'Connor's <a href="https://jacko.io/object_soup.html">Object Soup is Made of Indexes</a>, a teaching example, not a production refactor.
</div>

<!--
Source: examples/35_friends.rs. Run cargo test --example 35_friends.
Two tests intentionally fail. API redesign is allowed. Keep directed links,
insertion order, and duplicate additions. Removing people is out of scope.
-->

---

# 35 · Review

```rust
let alice = Rc::new(RefCell::new(Person::new("Alice")));
alice.borrow_mut().add_friend(&alice);
```

- Which borrow is still active when `other.borrow()` runs?
- Are two people named Alice the same person?
- Does `Rc::clone` copy a person or share one?
- What keeps Alice and Bob alive if they add each other?
- Does being someone's friend need to keep them alive?

<!--
This caller uses the starter definitions from the previous slide.
The mutable guard remains alive throughout the call. The second borrow panics.
Name comparison also silently rejects distinct allocations with the same name.
Mutual strong links retain both allocations after external handles are dropped.
The starter's mutual-friendship test explicitly clears links to avoid leaking.
-->

---
zoom: 0.9
---

# 35 · Possible solution: one owner

```rust
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
```

- `People` owns the records; links store identities.
- Two records can share a name without sharing an ID.
- Cycles in friendships are not cycles in ownership.
- An ID does not keep a person alive after the directory is dropped.

<!--
Full solution and tests: solutions/35_friends.rs.
People::add appends a person and returns an ID; People::get returns Option<&Person>.
Person::name and Person::friends expose borrowed views without mutable storage access.
-->

---
zoom: 0.9
---

# 35 · Change the caller, not just the pointer

```rust
impl People {
    pub fn add_friend(&mut self, person: PersonId, friend: PersonId) -> Option<()> {
        self.people.get(friend.0)?;
        let entry = self.people.get_mut(person.0)?;
        if person != friend {
            entry.friends.push(friend);
        }
        Some(())
    }
}
```

```rust
let mut people = People::default();
let alice = people.add("Alice");
let other_alice = people.add("Alice");
assert_eq!(people.add_friend(alice, alice), Some(()));
assert_eq!(people.add_friend(alice, other_alice), Some(()));
```

<!--
Fragments continue the solution definitions; add/get and the accessors are in
solutions/35_friends.rs. Validate both IDs before mutation or the self-link no-op.
Some(()) means a valid request, including a self-link that changes nothing.
None means an out-of-range index. Run cargo test --example 35_friends_solution.
-->

---

# 35 · What did we trade away?

- IDs survive vector growth, not arbitrary removal or reordering.
- IDs belong to their issuing directory by convention, not by type.
- Borrowed views cannot stay live across mutation of the directory.
- `Rc::ptr_eq` before borrowing is a smaller identity fix, but cycles remain.
- `Weak` links need explicit owners and do not eliminate borrow conflicts.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
<a href="https://jacko.io/object_soup.html">Teaching source</a> · <a href="https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq">Rc::ptr_eq</a> · <a href="https://doc.rust-lang.org/std/rc/struct.Weak.html">Weak</a>
</div>

<!--
The proposed model is append-only. Removing or swapping vector entries can make
old IDs refer to different people. Slot reuse needs a stale-handle policy.
A valid numeric ID from another directory can identify an unrelated person here.
This is explicitly tested, not claimed to be prevented by the private newtype.
The smaller pointer fix requires an operation receiving both handles, rather than
the original &mut self receiver. Use shared ownership if independent lifetimes matter.
-->
