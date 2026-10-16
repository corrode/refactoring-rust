# Part V. Domain modeling

So far, we've started by keeping the public API intact and improving the implementation behind it.
In this section, I encourage you to change the API.
Choosing types, constructors, and methods is kind of the point of these exercises.

We'll start by giving a collection of rooms its own operations, then work on identifiers, response handling, and configuration.
Ask what each type represents, which values it should allow, and what callers need to do with it.
Update callers and tests when the interface changes, but keep checking the behaviour they rely on.
Changing a type does not automatically validate its contents or make an implementation better.
