---
layout: default
zoom: 0.9
---

# 14 · `room_occupancy`

```rust
pub fn adults_total(rooms: &[RoomOccupancy]) -> i32 {
    let mut total = 0;
    for room in rooms { total += room.adults; }
    total
}

pub fn children_total(rooms: &[RoomOccupancy]) -> i32 {
    let mut total = 0;
    for room in rooms { total += room.children.len() as i32; }
    total
}

pub fn max_adults_in_a_room(rooms: &[RoomOccupancy]) -> i32 {
    let mut max = 0;
    for room in rooms { if room.adults > max { max = room.adults; } }
    max
}

pub fn child_ages(rooms: &[RoomOccupancy]) -> Vec<i32> {
    let mut ages = Vec::new();
    for room in rooms { for age in &room.children { ages.push(*age); } }
    ages
}
```

<div class="absolute top-20 right-12 text-sm opacity-60">
<code>cargo test --example 14_room_occupancy</code>
</div>

<!--
Each loop is a textbook iterator adapter:
  - sum:     map + sum
  - count:   map + sum
  - max:     map + max (note: returns Option, behaviour for empty differs!)
  - flatten: flat_map / flatten + cloned/copied + collect
Worth pausing on `max_adults`: original returns 0 on empty input;
`Option::unwrap_or(0)` preserves that.
-->

---

# 14 · Review

- Every loop has the same shape: read a field, accumulate. What's that pattern in iterator land?
- For the max, what does `Iterator::max` return - and how is that different from the current behaviour on an empty slice?
- For `child_ages`, the nested loop is exactly what one adapter is named after.
- Do you need any `mut` locals at all?


---

# 14 · Possible solution

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

<div class="mt-6 text-base opacity-80">

- Each helper collapses to one chain - the shape is now visible at a glance.
- `max` returns `Option`, so we restore the original "0 if empty" with `unwrap_or(0)`.
- `flat_map` + `copied` flattens the nested `&Vec<i32>` into owned values.

</div>
