# Part I. Warm-ups

These first four reviews are small enough to hold in your head at once. That leaves
room to practise the book's method: read the contract, ask what the code requires from
its callers, name the cost of a proposed change, and check the cases the current tests
miss. The goal is not to compress each function into the fewest tokens. It is to make
the implementation and interface say what they mean.

We start with borrowing and `Option`, move through nested patterns and lazy iterator
APIs, then finish with a UTF-8 boundary bug hidden by green tests. The next part builds
on the same review habits with fallible operations, where preserving error information
becomes part of the contract.
