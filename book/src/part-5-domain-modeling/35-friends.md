# `friends`

**Advanced.** This exercise combines object identity, borrowing, and graph ownership.
`Person` records directed friendships.
Review the model while preserving insertion order and repeated additions to other people.
You may change the API, including how callers create people and record friendships.
Removing people is out of scope.
The example develops the friendship problem from Jack O'Connor's [Object Soup is Made of Indexes](https://jacko.io/object_soup.html), a teaching article rather than a production refactoring report.

Two tests intentionally fail; make them pass without weakening their assertions.

## Code to refactor

```rust,editable
{{#include ../../../examples/35_friends.rs:listing}}
```

<script type="text/plain" class="playground-tests" data-expected-failures="tests::self_friendship_is_a_noop tests::people_with_same_name_are_distinct">
{{#include ../../../examples/35_friends.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 35_friends
```

</div>

## Discussion

<details class="discussion-question">
<summary>Why does Alice trying to add herself panic?</summary>

With the starter's definitions in scope, this call reproduces the failure:

```rust,ignore,noplayground
let alice = Rc::new(RefCell::new(Person::new("Alice")));
alice.borrow_mut().add_friend(&alice);
```

`borrow_mut()` creates a guard that stays alive throughout the method call.
The method receives `&mut Person` through that guard.
Inside the method, `other.borrow()` tries to borrow the same `RefCell` again, this time immutably.
The two borrows conflict, so `RefCell` panics before the name comparison can reject the link.
This is a single-threaded program; no race is needed.

</details>

<details class="discussion-question">
<summary>Does comparing names identify the same person?</summary>

No.
Two separately allocated people can both be named Alice.
The starter silently refuses to link them, even though they are different people.
A display name is data about a person, not a unique identity.
The second failing test makes that distinction part of the required behaviour.

</details>

<details class="discussion-question">
<summary>Does <code>Rc::clone</code> copy the person?</summary>

No, it creates another owning handle to the same allocation and increments its reference count.
It does not copy the name or the friendship list.
Replacing it with a deep copy would change the model: updating the original person would not update the copied person in a friend's list.
The question is whether a friendship needs to own the person it identifies.

</details>

<details class="discussion-question">
<summary>What keeps two mutual friends alive?</summary>

After Alice adds Bob and Bob adds Alice, each friendship list contains a strong reference to the other person.
Dropping the callers' handles does not reduce either reference count to zero.
The people keep each other alive, so their allocations are not reclaimed unless something breaks the cycle.
The starter's mutual-friendship test clears the links after checking them so that its fixtures do not leak.
This is a memory leak, not memory unsafety.

</details>

<details class="discussion-question">
<summary>Would changing every friendship to <code>Weak</code> solve both problems?</summary>

It would stop those edges from keeping people alive, but it would not change the borrowing rules.
An upgraded weak handle can still point to the same person already borrowed by the caller.
You also need a place that owns strong references for as long as the people should exist.
For this exercise, a single collection can own the people while their friendships identify other entries in that collection.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Give the people one owner

```rust,editable
{{#include ../../../solutions/35_friends.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/35_friends.rs:tests}}
</script>

`People` owns the records in a vector.
Each friendship stores a `PersonId`, not a reference to another record.
Alice and Bob can refer to each other without either record owning the other.
Dropping `People` drops both records, even if callers still hold their IDs.

`add_friend` receives both identities before borrowing a record mutably.
It checks both indices, compares the IDs, and changes only the source person's list.
There is no nested `RefCell` borrow when both IDs identify the same entry.
A valid self-link request returns `Some(())` without adding an edge; an out-of-range ID returns `None` before any mutation.
`Some(())` means the request was handled, not that a new edge was necessarily added.

Distinct records receive distinct IDs within this directory, even when their names match.
This fixes the name-based rejection separately from the self-borrow panic.
Directed links, insertion order, and repeated additions to other people remain unchanged.

### Migrate callers and tests

Create one `People::default()` and replace each `Rc::new(RefCell::new(Person::new(name)))` with `people.add(name)`.
Callers retain the returned IDs and pass them to `people.add_friend(alice, bob)`.
To inspect Alice, use `people.get(alice)` and the borrowed `name()` and `friends()` accessors.
The friendship list now contains IDs, so looking up a friend's name requires the directory too.

The solution has its own attached tests because these are different caller-facing types.
The assertions still require the same directed, ordered links, including duplicates.
Additional tests cover vector growth and rejection of out-of-range IDs without partial updates.
The repository's integration tests also check that outside callers cannot construct a `PersonId` directly or mutate the directory while a borrowed person is still in use.

This changes lifetime semantics as well as method signatures.
An `Rc` handle can keep a person alive independently of other callers.
A `PersonId` cannot: the directory controls that person's lifetime.
Choose this ownership model only when that is the intended contract.

### A smaller repair can keep shared ownership

If independently owned handles are a requirement, an associated function or free function could accept both `Rc` handles and compare them with `Rc::ptr_eq` before borrowing either person.
That avoids the self-borrow panic and distinguishes two allocations with the same name.
It cannot be added inside the existing method as written, because the caller has already borrowed `self` mutably and the method does not receive its `Rc` handle.

That repair alone still permits strong reference cycles.
Weak friendship links plus explicit owners are another design, but callers must handle failed upgrades and avoid overlapping borrows.
`Rc` and `RefCell` are not inherently wrong; they answer different ownership and access requirements from the single-owner solution.

### What these IDs do not guarantee

The vector is append-only and its storage is private.
Reallocation can move the records in memory without changing their indices, so existing IDs still identify the same records after more people are added.
There is no removal, sorting, or slot reuse.

Adding `Vec::remove` would shift later indices; `swap_remove` would move another person into the removed slot.
Either could make an old friendship refer to the wrong person.
If deletion becomes necessary, define what happens to incoming links and use stable, non-reused IDs or a generational collection that can reject stale handles.
Plain indices do not provide that guarantee.

An ID also does not identify its issuing directory.
The first person in two different `People` values has the same numeric ID.
Bounds checks cannot detect a foreign ID whose index happens to be valid locally, and the solution tests this limitation.
Use IDs only with the directory that issued them; an API that must enforce this needs an additional owner-identity design.
This is the same distinction between category, validity, and instance identity explored in [typed_ids](31-typed-ids.md).

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | One owner and person IDs |
|---|---|---|
| Type-safety | Runtime borrow checks; names used as identity | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Exclusive owner access; distinct local IDs |
| Readability | Borrow guards and cloned handles at call sites | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Ownership and relationships kept separate |
| API design | Each friendship owns its target | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Explicit directory lifetime; callers must migrate |
| Performance | Per-person `Rc` allocation and reference counting | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> No `Rc` allocation or counting for person links |
| Flexibility | Handles independently extend person lifetimes | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Append-only storage tied to one owner |
| Error reporting | Self-link panics; equal names silently rejected | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Self-link is a no-op; unknown index returns `None` |

The performance comparison names removed work, not a measured speed-up.
Names and friendship vectors still allocate, and resolving an ID requires a lookup.
`None` does not distinguish which argument was out of range and does not detect in-range IDs from another directory.
The solution fixes the two required behaviours and deliberately changes who controls lifetimes; it is not a drop-in replacement for independently owned handles.

</details>

## Takeaways For Our Own Codebase

> - Which graph edges identify another object, and which must keep that object alive?
> - Check whether a caller still holds a borrow guard when a method follows a link back to the same object.
> - Test self-links, two objects with the same display name, and cycles separately.
> - Before replacing references with IDs, specify their owner, deletion policy, and reuse rules.
> - Keep shared ownership when independent lifetimes are a real requirement, rather than removing it to satisfy a style rule.

## References

- [Object Soup is Made of Indexes](https://jacko.io/object_soup.html), Jack O'Connor: the teaching example behind the starter and the single-owner, index-based approach.
- [`RefCell::borrow`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow): a shared borrow panics while a mutable borrow is active.
- [`Rc::ptr_eq`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq): compare allocation identity without borrowing the contained values.
- [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html): refer to an allocation without keeping its value alive.
- [Reference cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html): strong cycles can prevent reference-counted values from being dropped.
- [Typed IDs](31-typed-ids.md): a newtype distinguishes a category, not a particular owner or a live entry.
