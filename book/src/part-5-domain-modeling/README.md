# Part V. Domain modeling

So far, we've started by keeping the public API intact and improving the implementation behind it.
In this section, I encourage you to change the API.
Choosing types, constructors, and methods is kind of the point of these exercises.

We'll start by giving a collection of rooms its own operations, then work on identifiers, response handling, and configuration.
You'll also model buffer policies, separate ID types, control access to secrets, and restrict which configuration steps callers can take.
Ask what each type represents, which values it should allow, and what callers need to do with it.
Update callers and tests when the interface changes, but keep checking the behaviour they rely on.
In the browser, expand **Tests and callers** beneath a code block to edit its attached suite alongside your implementation.
Run uses those edits, not a different suite selected to fit your API.
Keep the assertions that express the required behaviour, including any that initially fail.
Changing a type does not automatically validate its contents or make an implementation better.
