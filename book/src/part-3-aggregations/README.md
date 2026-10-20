# Part III. Aggregations & counting

These seven exercises compare iterator methods with explicit loops for aggregation and counting.
You'll collapse runs, check membership, collect spelling mistakes, sum room occupancies, compute text statistics, and count frequencies.
The `Option` and `Result` patterns from part II help express missing values and failures along the way.

The final exercise, [`list_sum`](19-list-sum.md), consumes a recursive list to aggregate its values.
You'll compare iteration with recursion, including how evaluation order can change which intermediate additions overflow when overflow checks are enabled.
Rust does not guarantee tail-call optimization, so a recursive implementation cannot rely on constant stack usage across compilers and build settings.
Part IV then applies standard library methods and custom iterators to strings and parsing.
