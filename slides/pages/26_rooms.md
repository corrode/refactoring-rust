---
layout: default
zoom: 0.9
---

# 26 · `rooms`

```rust
pub fn adults_total(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.adults).sum()
}

pub fn children_total(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.children.len() as i32).sum()
}

pub fn max_adults_in_a_room(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.adults).max().unwrap_or(0)
}

pub fn child_ages(rooms: &[RoomOccupancy]) -> Vec<i32> {
    rooms.iter().flat_map(|r| r.children.iter().copied()).collect()
}
```

<div class="absolute top-20 right-12 text-sm opacity-60">
<code>cargo test --example 26_rooms</code>
</div>

<!--
These are the iterator functions from exercise 16. RoomOccupancy has an adults:
i32 field and a children: Vec<i32> field. The loops are already gone; this time
we are reviewing the API. Domain-modeling exercises allow API changes.
The full starter and its tests are in examples/26_rooms.rs.
-->

---

# 26 · Review

- What do these four functions have in common?
- Would callers benefit from a type that owns the collection?
- Which operations should that type expose?
- Does owning the collection validate its contents?

<!--
Ask whether grouping these operations earns a new type. Free functions remain a
reasonable choice when callers need to query arbitrary borrowed slices.
An owned Rooms value makes its collection the receiver of every aggregate.
That is an API decision, not validation of counts, ages, or room membership.
-->

---
zoom: 0.85
---

# 26 · Possible solution: `Rooms`

```rust
pub struct Rooms(Vec<RoomOccupancy>);

impl Rooms {
    pub fn new(rooms: Vec<RoomOccupancy>) -> Self { Self(rooms) }

    pub fn adults_total(&self) -> i32 {
        self.0.iter().map(|r| r.adults).sum()
    }
    pub fn children_total(&self) -> i32 {
        self.0.iter().map(|r| r.children.len() as i32).sum()
    }
    pub fn max_adults_in_a_room(&self) -> i32 {
        self.0.iter().map(|r| r.adults).max().unwrap_or(0)
    }
    pub fn child_ages(&self) -> Vec<i32> {
        self.0.iter().flat_map(|r| r.children.iter().copied()).collect()
    }
}
```

- Construct `Rooms::new(vec)` once, then call `rooms.adults_total()`.
- The private collection gives these operations a shared receiver, not validation.

<!--
This is the main solution, with no compatibility wrappers around the free
functions. Callers transfer ownership of their Vec to Rooms::new.
The methods borrow self; constructing Rooms does not clone the collection.
Negative values remain accepted. max returns zero only for an empty collection,
not as a lower bound. child_ages still allocates its result and preserves order.
The full implementation, derives, and tests are in solutions/26_rooms.rs.
Run them with cargo test --example 26_rooms_solution.
-->
