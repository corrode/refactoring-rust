# How to run a code review

This is the only chapter in the book with no Rust in it. We're going to
spend the next twenty-six chapters looking at small pieces of Rust and
asking what we'd say about them in a code review. Before we do that,
let's talk about what a code review is actually *for*, because most
teams have never agreed on an answer, and the disagreement quietly
colours every PR.

## The reviewer's job is not to be right

A reviewer's job is to help the author (and, more importantly, the
team) write better code six months from now. That's it. Catching bugs
is a side effect. Enforcing the style guide is a side effect. Making
the diff prettier is a side effect.

If you walk away from a review having "won" the argument, you've
probably failed. The PR merges or it doesn't, but the team's collective
taste only moves if the author and the reviewer both understand each
other a little better at the end. That requires a different kind of
conversation than a debate.

The book's premise is that the conversation that works is built on
**asking questions**.
Not for theatrical reasons. Questions are not a polite way of dressing
up a verdict. They're a more accurate description of what's actually
happening. A reviewer almost never *knows* that a piece of code is
wrong. They notice that it's *unfamiliar*, or *complicated*, or
*tightly coupled*, or *the third version of something we already have
twice*. Whether those things matter depends on context the reviewer
usually doesn't have. Asking is honest. Pronouncing is overreach.

## Three rhetorical postures

A reviewer's comment can take one of three forms. Most teams use them
all, and most use the wrong one most of the time.

**Ask.** *"What happens when this is called with an empty string?"*
You don't know the answer; you're not even sure there's a problem;
you're surfacing the question. The author either answers it (good, a
test case might come out of the exchange) or realises they don't know
either (better, a bug just got caught before merge). Asking is the
default. If you're not sure whether a comment belongs in the review at
all, phrase it as a question and you almost always make it land better.

**Suggest.** *"Consider returning `Option<&str>` here so the caller
doesn't have to check for the sentinel value."* You have an opinion;
you're sharing it; the author is free to take it or leave it. The key
word is *consider*. A suggestion that the author rejects is not a
failure of the review. The suggestion did its job by getting
articulated. Sometimes the author rejects it because they thought of it
already and chose the other path; in that case the comment thread
becomes the record of that decision, which is itself valuable. The PR
gets merged, and now the team has a richer shared model of why this
function looks the way it does.

**Require.** *"This needs to handle the disconnect case before this
ships."* You're blocking the merge. Use this for: correctness bugs,
security issues, public-API mistakes that would be expensive to undo,
and violations of agreements the team has already made and written
down. Don't use it for taste, idioms, structure, or anything that's
genuinely contested. If you find yourself reaching for *require* on a
matter of taste, the right move is to back off, write a *suggest*, and
take the taste argument to a separate conversation: a team discussion,
a design doc, an RFC. The PR isn't the place to settle whether the team
prefers `?` over `match`.

A useful exercise: count, across your last ten reviews, how often you
*asked* vs. how often you *required*. If the ratio is anywhere near
1:1, you're requiring too much.

## Every refactor has a cost. Name it

The single most common review failure I see is suggesting a refactor
without acknowledging what it costs. *"You could replace this loop with
a fold."* Could you? Sure. Should you? That depends. A fold is denser;
it requires the reader to hold the accumulator in their head; it's
harder to set a breakpoint inside; it's easier to write a closure that
captures the wrong variable. A loop is more verbose but reads top to
bottom. Neither is universally better.

When you suggest a change, name what the reader gives up by accepting
it. *"This would be shorter as a fold, but the loop is easier to debug.
Up to you."* That sentence does three things at once. It states the
benefit. It states the cost. It puts the decision back on the author,
where it belongs. It also models, for the rest of the team reading the
PR later, what an honest engineering trade-off looks like.

This book leans hard on this idea. Every chapter ends with a
*Trade-offs* section that scores the canonical solution against six
fixed axes (type-safety, readability, API design, performance,
flexibility, error reporting). Some chapters score the canonical
solution as *losing* on at least one axis, and we say so. That's not a
failure; that's how engineering works. There is no global optimum. A
review that pretends there is one is selling a fiction.

## Trust the type system, but don't worship it

A lot of the refactors in this book end with "and now the invariant
lives in the type system." That's usually a win, sometimes a big one.
But not always. Sometimes the invariant is *cheap to check at runtime
and expensive to express in the type system*, and the right move is to
keep the runtime check, write a good test, and move on. Sometimes
making illegal states unrepresentable requires a generic-parameter
machine that nobody on the team will ever extend, and the team would be
better served by a comment and a `debug_assert!`.

The book is going to push you toward type-driven design a lot, because
that's where Rust pays its dividends and because the canonical
solutions in our exercises mostly win on that axis. But type-driven
design is a tool, not an identity. A reviewer who reaches for "make
illegal states unrepresentable" on every PR has stopped reviewing and
started signalling.

When you suggest a type-level fix, ask yourself: *would I be comfortable
maintaining this in two years, with a different team?* If the answer
isn't yes, suggest something simpler.

## Disagreement is the point

Two senior reviewers will disagree about the same PR. They should. If
they don't, one of them isn't paying attention.

The bad version of this is when disagreement turns into ranking: whose
opinion wins. The good version is when disagreement becomes a record:
*here are two reasonable ways to think about this; here's why we chose
one this time; here's what we'd reconsider if conditions changed.*
That record is one of the most valuable things a PR can produce. It
outlives the author, the reviewer, and often the code itself.

Concretely: when you disagree with a teammate's review (or with the
canonical solution in this book, or with Clippy), say so. Out loud. In
writing. With your reasons. Don't quietly comply. Don't passively
approve. The team learns from your dissent in a way it cannot learn
from your silence.

This book disagrees with itself in places. We disagree with Clippy in
places. We're going to flag those moments explicitly, because they're
where the most learning happens.

## Distinguish bug, design, and preference

A useful triage before you write any review comment:

- **Bug.** The code is wrong. It crashes, it returns the wrong answer,
  it has a security implication, it violates a documented contract.
  This is *require*. Block the merge. Be specific about the failing
  case.

- **Design.** The code works, but the structure is wrong. The function
  does too much, the type leaks an implementation detail, the error
  case is hidden inside an `Option` that should be a `Result`. This is
  *suggest*, occasionally *require* if the design will be expensive to
  fix later (public API). Always include the cost of changing it now
  vs. later.

- **Preference.** The code works and the design is fine; you'd have
  written it differently. This is *ask*, occasionally *suggest*, never
  *require*. If you find yourself wanting to require a preference,
  what you actually want is a team conversation, not a PR comment.

Conflating these is how reviews go bad. A preference dressed as a
require feels like an ambush; a bug dressed as a question gets lost.

## Praise the things you'd otherwise overlook

If a function is well-named, say so. If an error type carries exactly
the right amount of information, say so. If a refactor of a function
you reviewed three months ago made the new function trivial to write,
say so, and tag the original author.

Praise is not a politeness ritual. It's a signal of what the team
agrees *good* looks like, and that signal is hard to send any other
way. A team that only ever writes critical review comments is a team
that's slowly losing track of its own taste.

This is the cheapest thing in a review and the most consistently
neglected. Build the habit.

## When you can't articulate it

Sometimes you look at a piece of code and your gut says *no* and you
can't say why. Most reviewers either approve-and-grumble or write a
vague "this feels off" comment that lands as either passive-aggressive
or actionable-by-nobody. Both are bad.

The honest move is to say so. *"Something about this is bothering me
and I haven't figured out what. Give me a day."* Then come back. Either
you'll articulate it (now you have a real review comment), or you
won't (apologise, approve, move on). Either outcome is fine; the
loitering "this feels off" comment is not.

This requires trust on the team: that a reviewer asking for time
isn't blocking the PR out of spite. You can't fake that culture; you
have to build it. The shortest path to building it is being the
reviewer who explicitly says "I approved this despite not loving X,
because Y" once or twice. It's contagious.

## The code is not your friend

Be ruthless about the code. Be gentle about the person.

The code is text. It does not have feelings. You can say *"this is
confusing"* and the code will be fine. The author has feelings; the
author has worked on the change for hours or days; the author is
reading the review at 4pm with a deadline.

Concretely: write about the code, not the author. *"This function does
two things"* lands. *"You're doing two things here"* lands worse, even
when both sentences are objectively true and equally specific. The
grammatical subject of a review comment should almost always be a noun
in the code, not the second person.

This sounds like a small thing. It is a small thing. Small things in
code reviews compound the way small things compound everywhere else.

## Receiving a review

This book is about giving reviews, but every author is also a
receiver, so a brief note.

The hardest skill on the author side is **separating the idea from
yourself**. A comment on your code is not a comment on your worth as
an engineer. This is easy to write and hard to feel. The reviewer
who suggests rewriting your function is not telling you that you're
bad at your job; they're telling you they think there's a better
design. They might be wrong. You're allowed to push back. *Engage with
the idea.*

The second hardest skill is recognising **when you'd write it the same
way again**. If, after a thorough review, you would write the function
exactly as you originally wrote it, the review hasn't taught you
anything. That's not always bad. Sometimes you were right the first
time. But it's worth noticing, because if it keeps happening, either
you're not engaging with reviews honestly or your reviewers aren't
engaging with your code honestly. Both are fixable. Neither is fixable
if nobody names it.

## How this book uses all this

Every chapter from here on follows the same format. We show you a piece
of code. We pose the questions a thoughtful reviewer would ask. We
walk through what the answers point at. We score the trade-offs
honestly, including the ones the canonical solution loses. We end
with follow-up questions you can take to your own team's code.

If you read it the right way, the book is a long worked example of the
posture this chapter describes. You're welcome to disagree with any of
it. The point is to give us (you, your team, and me) a shared
vocabulary for the disagreement.

Let's get to the code.
