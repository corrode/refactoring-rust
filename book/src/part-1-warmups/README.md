# Part I. Warm-ups

These five exercises cover boolean expressions, borrowing, `Option`, nested patterns, lazy iterators, and UTF-8 boundaries.
Each starts with a small function and its tests.
Check what the function does, what it requires from callers, and which cases the tests cover before changing the implementation.

The first three exercises simplify code without changing its results.
The fourth keeps the `Vec` return type in the first solution, then explores an iterator alternative that requires changes to callers.
The fifth fixes a UTF-8 boundary panic that the existing tests miss.
Part II continues with fallible code and the information callers need when an operation fails.
