# How to run a code review

A code review can catch a bug and still leave the team worse off. The patch gets
fixed, but nobody learns why. Or a reviewer wins an argument about style and the
author learns to keep quiet next time.

This is the only instructional chapter without a Rust exercise. The twenty-seven
exercise chapters ask what we'd say about small pieces of Rust in review. First we need
to agree on what that conversation is for.

## The reviewer's job is not to be right

The reviewer's job is to help the author and the wider team write better code six
months from now. Catching bugs, enforcing written agreements,
and making a diff easier to read all matter. They're still side effects of
helping the team improve, not reasons to treat the review as a contest.

If you leave a review having won the argument, you may still have failed. The
PR either merges or it doesn't. The team's judgement improves only when author
and reviewer understand the decision better than they did before.

The most reliable way to get there is to **ask questions**.

This isn't politeness theatre. A question isn't a verdict with softer
punctuation. It reflects what a reviewer usually knows: something is unfamiliar,
complicated, tightly coupled, or the third version of code that already exists
twice. Whether that observation matters depends on context the reviewer may not
have. Asking exposes the missing context. Pronouncing skips over it.

## Three rhetorical postures

Every review comment takes one of three postures: ask, suggest, or require. Most
teams use all three. Trouble starts when they use them interchangeably.

**Ask.** *"What happens when this is called with an empty string?"*

You don't know the answer, and you may not know whether there's a problem. You
want the case considered. The author may explain it, which could lead to a test.
Or they may realise they don't know either, which could reveal a bug before
merge. Ask by default when context could change your judgement. If you're not
sure a comment belongs in the review at all, a real question usually gives the
author a better way to answer it.

That doesn't mean turning a known bug into a timid question. If the code is
wrong, say so. Questions are useful because they're accurate, not because
they're gentle.

**Suggest.** *"Consider returning `Option<&str>` here so the caller doesn't
have to check for the sentinel value."*

You have a preferred design, but the author can reasonably reject it. The word
*consider* carries real meaning. A rejected suggestion hasn't failed. It made
the alternative and its trade-offs explicit. If the author already considered
it, the thread records why the team chose another path. That record can matter
long after the patch merges.

**Require.** *"This needs to handle the disconnect case before this ships."*

A requirement blocks the merge. Reserve it for correctness bugs, security
issues, public API mistakes that would be expensive to undo, and violations of
agreements the team has already written down. Don't use it for taste, idioms,
structure, or a genuinely contested design.

If you want to require a matter of taste, step back. Write a suggestion, then
move the larger argument to a team discussion, design document, or RFC. A pull
request is a poor place to settle whether the team prefers `?` over `match`.

Look at your last ten reviews and count asks against requirements. If the ratio
is close to 1:1, you're requiring too much.

## Every refactor has a cost. Name it

A comment such as *"You could replace this loop with a fold"* names an
alternative but hides the decision. Yes, you could. Should you?

A fold is denser. You have to keep the accumulator in your head, setting a
breakpoint inside it is harder, and its closure can capture the wrong variable.
A loop takes more lines but reads from top to bottom. Neither is better in every
context.

When you suggest a change, name what the author gives up by accepting it:
*"This would be shorter as a fold, but the loop is easier to debug. Up to you."*
The comment states the benefit, states the cost, and leaves a contested choice
with the author. It also shows everyone who reads the PR later what an honest
engineering trade-off looks like.

Every chapter in this book follows that discipline. Its *Trade-offs* section
scores the canonical solution against six fixed axes: type-safety, readability,
API design, performance, flexibility, and error reporting. Some canonical
solutions lose on an axis. We say so.

That's not a defect in the exercise. Engineering has no global optimum. A
review that pretends otherwise is selling a fiction.

## Trust the type system, but don't worship it

Many refactors in this book end with an invariant moving into the type system.
That's often a win, sometimes a large one. It isn't free.

An invariant may be cheap to check at runtime and expensive to express in a
type. In that case, keep the runtime check, write a good test, and move on.
Making an illegal state unrepresentable may also require a machine of generic
parameters that nobody on the team will extend. For an internal invariant whose
violation cannot make release behaviour incorrect, a comment and `debug_assert!` may
serve that team better. Use a normal runtime check when release code still needs the
protection.

The exercises lean toward type-driven design because Rust is unusually good at
it, and because the canonical solutions often improve on that axis. Still,
type-driven design is a tool, not an identity. Repeating "make illegal states
unrepresentable" on every PR isn't review. It's signalling.

Before suggesting a type-level fix, ask: *would I be comfortable maintaining
this in two years with a different team?* If not, suggest the less elaborate
option.

## Disagreement is the point

Two senior reviewers can look at the same code and reach different conclusions.
The useful outcome isn't a ranking of whose opinion wins. It's a record:
*here are two reasonable ways to think about this; here's why we chose one this
time; here's what would make us reconsider.*

That record is one of the most valuable things a PR can produce. It can outlive
the author, the reviewer, and the code itself.

So when you disagree with a teammate's review, the canonical solution in this
book, or Clippy, say so and give your reasons. Say it out loud. Put it in
writing. Don't quietly comply or passively approve. Your team can learn from an explicit disagreement. It can't learn from silence.

This book disagrees with itself in places, and it sometimes disagrees with
Clippy. Those moments are marked because they expose the judgement behind the
code.

## Distinguish bug, design, and preference

Before writing a comment, decide what kind of problem you're looking at:

- **Bug.** The code is wrong. It crashes, returns the wrong answer, has a
  security implication, or violates a documented contract. This is *require*.
  Block the merge and name the failing case.

- **Design.** The code works, but its structure causes trouble. The function
  does too much, the type leaks an implementation detail, or an error is hidden
  in an `Option` that should be a `Result`. This is usually *suggest*. It can be
  *require* when delay would make the design expensive to fix, as with a public
  API. State the cost of changing it now and the cost of changing it later.

- **Preference.** The code works and the design is sound, but you'd have written
  it differently. This is *ask*, sometimes *suggest*, and never *require*. If a
  preference needs to become a rule, take it to the team rather than smuggling
  it into one PR.

Conflating these categories is how reviews go bad. A preference dressed as a
requirement feels like an ambush. A bug dressed as a question gets lost.

## Praise the things you'd otherwise overlook

If a function is well named, say so. If an error type carries the right amount
of information, say so. If an earlier refactor made today's change easy, say so
and tag the person who wrote it.

Praise isn't a politeness ritual. It tells the team what should survive the
next refactor. A review culture that only records criticism eventually loses
track of what the team already does well.

This costs almost nothing, which makes its absence hard to defend. Build the
habit.

## When you can't articulate it

Sometimes the code bothers you before you know why. Approving while grumbling
doesn't help. Neither does *"this feels off"*, which leaves the author with
nothing they can act on.

Be precise about the uncertainty: *"Something about this is bothering me and I
haven't figured out what. Give me a day."* Then come back. If you can explain
the concern, write the real comment. If you can't, apologise, approve, and move
on. Don't leave a vague objection hanging over the PR.

This only works when the team trusts that a request for time isn't a spiteful
block. You can't fake that trust. You build it by making your own boundaries
visible, for example: *"I approved this despite not loving X, because Y."*
After people see that disagreement doesn't automatically block a merge, they're
more likely to be equally candid.

## The code is not your friend

Be ruthless about the code. Be gentle about the person.

Code is text. You can call it confusing and it won't mind. Its author has spent
hours or days on the change and may be reading your comment at 4pm against a
deadline.

Write about the code, not its author. *"This function does two things"* lands
better than *"You're doing two things here."* Both statements may be accurate,
but only one keeps the discussion attached to the thing everyone can change.
The grammatical subject of a review comment should usually be a noun in the
code, not *you*.

It's a small distinction. Small distinctions accumulate.

## Receiving a review

Giving and receiving reviews are the same practice from opposite sides. The
hardest part for an author is separating the idea from themselves. A comment on
your code isn't a verdict on your worth as an engineer. That's easy to write
and hard to feel.

A reviewer who suggests rewriting a function is saying they see a better
design. They may be wrong. Push back when you think they are, but engage with
the idea rather than defending yourself from a claim they didn't make.

The next question is whether the review changed your reasoning, confidence, or code. It
may change none of them. Sometimes the first version was right, and the discussion only
confirmed why. But if reviews repeatedly add neither new context nor confidence, say so.
The author may not be engaging with the comments, the comments may not engage with the
code, or the team may be reviewing at the wrong level. You can only fix that once it is
visible.

## How this book uses all this

From here on, each chapter follows the same sequence. You see a piece of code,
consider the questions a thoughtful reviewer would ask, and follow the answers
toward a possible solution. Then you score its costs as well as its benefits
and take the follow-up questions back to your own code.

Treat the book as a long worked example of this review posture. Disagree with
it where your context points somewhere else. The goal isn't agreement. It's a
shared vocabulary precise enough to make the disagreement useful.

Now we can get to the code.
