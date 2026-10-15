# How to run a code review

A code review should catch bugs, check the design, and help the team understand
why a change is appropriate. Explain your concerns so the author can act on them
and use the reasoning in later work.

This chapter describes how to give and receive reviews. The exercise chapters
apply these practices to small pieces of Rust.

## The reviewer's job

Check correctness, written team agreements, and how easy the code is to read
and maintain. Explain the reason for each requested change, not just the edit
you want.

Ask questions when you need context. Unfamiliar code, tight coupling, or
duplication may have a reason you don't yet know. Find out before recommending
a change. When you have found a definite bug, state it directly.

## Ask, suggest, or require

Make clear whether a comment asks for information, proposes an optional change,
or blocks the merge.

**Ask** when context could change your judgement. For example, ask how a
function handles an empty string if the answer isn't clear. The explanation
may be enough, or it may reveal a missing test or bug.

**Suggest** when you prefer a design but the author can reasonably reject it.
For example, returning `Option<&str>` instead of a sentinel value could remove
special-case checks from callers. Explain that benefit and any costs. If the
author chooses another approach, record why.

**Require** a change when the patch has a correctness bug, security issue,
public API mistake that would be expensive to undo, or a violation of a written
team agreement. State the problem and what must be resolved before merging.

Don't make a preference about idioms or structure a requirement. Suggest it,
and take any proposed team-wide rule to a separate discussion, design document,
or RFC. For example, a preference for `?` over `match` should not become a new
rule during a single pull request.

When reviewing your own comments, check whether each blocking request has a
concrete reason. The number of questions or requirements alone doesn't tell
you whether a review is useful.

## Explain the cost of a refactor

Naming an alternative is not enough. Explain what it improves and what becomes
harder. Replacing a loop with `fold` can shorten the code, but it can also make
the accumulator harder to follow or the code harder to debug. A closure can
capture the wrong variable. A loop uses more lines but keeps the steps explicit.
Neither is better in every context.

State these costs when suggesting a change, and leave the choice with the
author when both versions are reasonable. The review then records why the
team kept the loop or chose the fold.

Each exercise's *Trade-offs* section follows this approach. It scores the
canonical solution against type-safety, readability, API design, performance,
flexibility, and error reporting. A solution can improve some criteria and
make others worse. Whether to accept that cost depends on the requirements.

## Choose where to enforce invariants

Many refactors in this book use types to enforce invariants. This can prevent
invalid calls, but it also affects the API and the work callers must do.

An invariant may be cheap to check at runtime and complicated to express in a
type. In that case, keep the runtime check and test it. A design with many
generic parameters may prevent an invalid state while being too difficult for
the team to extend.

For an internal invariant whose violation cannot make release behaviour
incorrect, a comment and `debug_assert!` may be enough. Use a normal runtime
check when release code still needs the protection.

The exercises often use type-driven design because Rust supports it well.
Before recommending a type-level check, consider whether the team can maintain
it and whether a simpler runtime check would meet the same requirements.

## Record disagreements and decisions

Reviewers can reasonably reach different conclusions about the same code.
Record the alternatives, why the team chose one, and what would justify
reconsidering it. This gives future maintainers context for the decision.

When you disagree with a teammate, a solution in this book, or Clippy, explain
why. The book also marks disagreements with Clippy or earlier recommendations
and explains the relevant constraints.

## Distinguish bug, design, and preference

Before writing a comment, decide what kind of problem you're looking at:

- **Bug.** The code crashes, returns the wrong answer, creates a security
  problem, or violates a documented contract. Require a fix and name the
  failing case.
- **Design.** The code works, but its structure causes trouble. A function
  does too much, a type exposes an implementation detail, or an `Option` hides
  an error that callers need in a `Result`. Usually, suggest a change. Require
  it when delaying the decision would make it expensive to fix, as with a
  public API. Explain the cost of changing it now and later.
- **Preference.** The code works and the design is sound, but you'd write it
  differently. Ask about the choice or suggest an alternative; don't block
  the merge. Discuss proposed team-wide rules separately.

These distinctions help the author decide which comments need a fix, an
explanation, or further discussion.

## Point out what works well

Mention useful choices as well as problems: a clear function name, an error
type with the right information, or an earlier refactor that made this change
easier. Explain what helped so the team knows what to preserve in future
changes. Credit the person responsible when relevant.

## When you can't explain a concern

If code seems wrong but you can't yet explain why, say what you need to
investigate and agree on a time to return to the review. A vague objection
doesn't give the author enough information to make a change.

Follow up when promised. If you find a concrete problem, describe it. If you
don't, withdraw the objection and approve if nothing else blocks the merge.
Don't leave an unresolved concern without a next step.

When you approve despite a reservation, explain why it doesn't block the
change. This makes the difference between a preference and a requirement clear.

## Comment on the code, not the author

Describe the function, type, or behaviour that needs attention. For example,
identify the two responsibilities a function combines, then explain why
separating them would help. Avoid claims about the author's ability or intent.

Specific comments keep the discussion focused on something the author can
change, even when the review happens under time pressure.

## Receiving a review

Evaluate a review comment as a proposal about the code. Check the reviewer's
reasoning and ask for missing context. If you disagree, explain the requirement
or trade-off that supports your choice.

A review can improve the code, add context, or confirm why the original version
is appropriate. It does not have to produce an edit to be useful. If reviews
repeatedly provide none of these benefits, discuss the process with the team.
Check whether authors engage with comments, whether comments address the code,
and whether reviews focus on the right level of detail.

## How this book uses these practices

Each exercise gives you code to review, questions to consider, and a possible
solution. Compare the solution's costs and benefits with your own, then use
the follow-up questions to examine similar decisions in your code.

You do not need to agree with every solution. You should be able to explain
which constraints led to your choice.
