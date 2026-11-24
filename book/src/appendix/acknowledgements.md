# Acknowledgements

Thanks to everyone who challenged an early workshop solution.
Many of those objections now appear in the trade-off sections.

The exercises also draw on these articles and projects:

- [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1) supplies the problem behind [11: floor](../part-2-option-result/11-floor.md).
- Jack O'Connor's [Object Soup is Made of Indexes](https://jacko.io/object_soup.html) is the teaching source for [39: friends](../part-5-domain-modeling/39-friends.md).
- **hyper**, with code credited to Sean McArthur, provides the historical buffer setters adapted in [32: buffer_policy](../part-5-domain-modeling/32-buffer-policy.md).
- **The Nushell Project Developers** provide the ID refactor behind [33: typed_ids](../part-5-domain-modeling/33-typed-ids.md).
- **secrecy and iqlusion** provide the explicit-access and redacted-debugging design discussed in [35: secret_token](../part-5-domain-modeling/35-secret-token.md).
- **rustls**, with code credited to Joseph Birr-Pixton, provides the typed-builder design simplified in [36: config_typestate](../part-5-domain-modeling/36-config-typestate.md).
- **nom**, with code credited to Geoffroy Couprie, provides the parser-associated-types refactor adapted in [38: mapped_parser](../part-5-domain-modeling/38-mapped-parser.md).

The Rust documentation, Google Rust Style Guide, and vimspell's iterator API also inform the explanations.
[Further reading](further-reading.md) collects their links, pinned sources, and the distinctions between upstream changes and local teaching examples.
These source credits do not imply participation in or endorsement of the book.

The repository's [third-party notices](https://github.com/corrode/refactoring-rust/blob/main/THIRD_PARTY_NOTICES.md) retain the full copyright and permission notices for exercises 32, 33, 35, 36, and 38.
Keep the applicable notices when redistributing adapted code; an acknowledgement does not replace them.
