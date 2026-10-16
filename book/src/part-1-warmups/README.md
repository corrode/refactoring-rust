# Part I. Warm-ups

These four exercises cover borrowing, `Option`, nested patterns, lazy iterators, and
UTF-8 boundaries. Each starts with a small function and its tests. Check what the
function does, what it requires from callers, and which cases the tests cover before
changing the implementation.

The first two exercises simplify code without changing its results. The third changes
a return type from `Vec` to an iterator, which also requires changes to callers. The
fourth fixes a UTF-8 boundary panic that the existing tests miss. Part II continues with
fallible code and the information callers need when an operation fails.
