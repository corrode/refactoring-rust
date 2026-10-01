# Part I. Warm-ups

These four reviews fit in your head, which is exactly why they are useful. With little
code to track, you can pay attention to the review itself: read the contract, notice what
the code demands from its callers, account for the cost of each change, and ask which
cases the tests don't cover. The shortest function isn't the goal. The implementation
and its interface should say what the code actually does.

We begin with borrowing and `Option`, then continue with nested patterns and lazy
iterators. The last warm-up contains a UTF-8 boundary bug even though its tests pass.
Part II applies the same habits to fallible code, where the information preserved by an
error becomes part of the contract.
