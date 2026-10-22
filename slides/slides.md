---
theme: the-unnamed
title: 'Writing Better Rust: Fully Using the Type System'
info: |
  ## Writing Better Rust
  A refactoring workshop by Matthias Endler (Corrode)
class: text-left
highlighter: shiki
drawings:
  persist: false
transition: slide-left
mdc: true
colorSchema: dark
fonts:
  sans: 'Inter'
  serif: 'Source Serif Pro'
  mono: 'JetBrains Mono'
defaults:
  layout: default
themeConfig:
  color: "#F3EFF5"
  background: "#161C2C"
  code-background: "#0F131E"
  code-border: "#242d34"
  accents-teal: "#44FFD2"
  accents-yellow: "#FFE45E"
  accents-red: "#FE4A49"
  accents-lightblue: "#15C2CB"
  accents-blue: "#5EADF2"
  accents-vulcan: "#0E131F"
---

<AccentStripe />

# Writing Better Rust

## Fully Using the Type System

<div class="mt-12 text-xl opacity-80">
A refactoring workshop
</div>

<div class="absolute bottom-10 left-12 text-sm opacity-60">
Matthias Endler · Corrode
</div>

---
layout: about-me
helloMsg: Hello!
name: Matthias Endler
imageSrc: /me.png
position: left
job: Rust Consultant
line1: Host of Rust in Production
social1: "@mre"
social2: corrode.dev
---

<style>
.about-me {
  --slidev-theme-aboutme-background: var(--slidev-theme-background);
  --slidev-theme-aboutme-color: var(--slidev-theme-color);
  --slidev-theme-aboutme-helloColor: var(--slidev-theme-background);
}
</style>

---

# About you

<div class="opacity-80 mb-8">
Let's get acquainted. 
</div>

<div class="grid grid-cols-2 gap-x-12 gap-y-6 text-lg">

<div>

### 👋 Who
- Name and where you're from
- What do you do day to day?
- What are your main programming languages?

</div>

<div>

### 🦀 Rust
- How long have you been writing Rust?
- What do you use it for? (work, side project, learning)

</div>

<div>

### 🔥 Wildcard
- What do you want to take away from this workshop?
- Favourite crate of the moment

</div>

</div>

---
src: ./pages/00_motivation.md
---

---
src: ./pages/00_schedule.md
---

---
src: ./pages/00_how_it_works.md
---

---
src: ./pages/00_arc_overview.md
---

---
layout: section
---

<AccentStripe />

<div class="text-xs opacity-50 tracking-widest uppercase mb-6">Phase 1 of 6</div>

# Warm-ups

<div class="text-2xl opacity-80 mt-4">
Types, signatures, simple matches.
</div>

<div class="mt-10 text-base opacity-60 max-w-2xl">
The point is to get talking. Start with the existing signatures;
then compare API alternatives and the tradeoffs they introduce.
</div>

<div class="absolute bottom-12 right-12 text-sm opacity-50 tracking-wide">
Exercises 01–05
</div>

---
src: ./pages/01_is_yes.md
---

---
src: ./pages/02_starts_with_uppercase.md
---

---
src: ./pages/03_better_match.md
---

---
src: ./pages/04_even_numbers.md
---

---
src: ./pages/05_truncate_string.md
---

---
layout: section
---

<AccentStripe />

<div class="text-xs opacity-50 tracking-widest uppercase mb-6">Phase 2 of 6</div>

# `Option` · `Result` · `?`

<div class="text-2xl opacity-80 mt-4">
The idioms that show up in almost every Rust file.
</div>

<div class="absolute bottom-12 right-12 text-sm opacity-50 tracking-wide">
Exercises 06–12
</div>

---
src: ./pages/06_path.md
---

---
src: ./pages/07_nesting.md
---

---
src: ./pages/08_optional_values.md
---

---
src: ./pages/09_parse_ints.md
---

---
src: ./pages/10_floor.md
---

---
src: ./pages/11_error_handling.md
---

---
src: ./pages/12_parse_port.md
---

---
layout: section
---

<AccentStripe />

<div class="text-xs opacity-50 tracking-widest uppercase mb-6">Phase 3 of 6</div>

# Aggregations & counting

<div class="text-2xl opacity-80 mt-4">
Iterator chains, explicit loops, and consuming a recursive list.
</div>

<div class="absolute bottom-12 right-12 text-sm opacity-50 tracking-wide">
Exercises 13–19
</div>

---
src: ./pages/13_dedup_runs.md
---

---
src: ./pages/14_excluded_path.md
---

---
src: ./pages/15_spell_check.md
---

---
src: ./pages/16_room_occupancy.md
---

---
src: ./pages/17_text_stats.md
---

---
src: ./pages/18_mode.md
---

---
src: ./pages/19_list_sum.md
---

---
layout: section
---

<AccentStripe />

<div class="text-xs opacity-50 tracking-widest uppercase mb-6">Phase 4 of 6</div>

# Strings, parsing & iterators

<div class="text-2xl opacity-80 mt-4">
Reach for the standard library before hand-rolling a loop.
</div>

<div class="absolute bottom-12 right-12 text-sm opacity-50 tracking-wide">
Exercises 20–25
</div>

---
src: ./pages/20_trim_log_line.md
---

---
src: ./pages/21_parse_srt_timestamp.md
---

---
src: ./pages/22_iterators.md
---

---
src: ./pages/23_transformer.md
---

---
src: ./pages/24_fun_strings_ext.md
---

---
src: ./pages/25_quicksort.md
---

---
layout: section
---

<AccentStripe />

<div class="text-xs opacity-50 tracking-widest uppercase mb-6">Phase 5 of 6</div>

# Domain modeling

<div class="text-2xl opacity-80 mt-4">
Here you can change the API from the first solution.
Choose types and methods, then update callers and tests.
</div>

<div class="absolute bottom-12 right-12 text-sm opacity-50 tracking-wide">
Exercises 26–35
</div>

---
src: ./pages/26_rooms.md
---

---
src: ./pages/27_iban_prefix_check.md
---

---
src: ./pages/28_http_response_router.md
---

---
src: ./pages/29_config_loader.md
---

---
src: ./pages/30_buffer_policy.md
---

---
src: ./pages/31_typed_ids.md
---

---
src: ./pages/32_validation_feedback.md
---

---
src: ./pages/33_secret_token.md
---

---
src: ./pages/34_config_typestate.md
---

---
src: ./pages/35_friends.md
---

---
layout: section
---

<AccentStripe />

<div class="text-xs opacity-50 tracking-widest uppercase mb-6">Phase 6 of 6</div>

# Capstone

<div class="text-2xl opacity-80 mt-4">
Lastly, two larger bonus problems.
</div>

<div class="absolute bottom-12 right-12 text-sm opacity-50 tracking-wide">
Exercises 36–37
</div>

---
src: ./pages/36_env_file_parser.md
---

---
src: ./pages/37_mini_redis.md
---

---
layout: center
---

# Want more of this?

<div class="grid grid-cols-2 gap-8 mt-8 items-center">

<div>

## [corrode.dev/pro](https://corrode.dev/pro)

<div class="mt-4 text-lg opacity-80">
Personalized Rust code reviews and 1:1 mentorship from experienced Rust engineers.
</div>

<div class="mt-6 text-base opacity-70">
Bring your own code. Get focused feedback, idiomatic refactors, and answers to the questions your team can't.
</div>

</div>

<div>
<img src="./public/corrode-pro.png" class="rounded shadow-lg" />
</div>

</div>

---
layout: center
---

# Podcast 

<div class="grid grid-cols-2 gap-8 mt-8 items-center">

<div>

## [corrode.dev/podcast](https://corrode.dev/podcast)

<div class="mt-4 text-lg opacity-80">
A podcast about how Rust is used in the real world. We talk to engineers from all kinds of companies about their Rust codebases, the problems they solve with Rust, and the lessons they've learned along the way.
</div>

</div>

<div>
<img src="./public/podcast.png" class="rounded shadow-lg" />
</div>

</div>


---
layout: center
---

# That's a wrap!

<div class="mt-12 text-xl opacity-80">
Thank you.
</div>

<div class="mt-8 text-base opacity-70">
Questions or your own code you want to discuss?
</div>

<div class="absolute bottom-10 left-12 text-sm opacity-60">
corrode.dev · @matthiasendler
</div>
