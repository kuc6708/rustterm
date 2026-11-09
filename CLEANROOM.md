# Clean-room policy

This repository is a **clean-room** implementation. It is designed to reproduce
broadly-available terminal/session **functionality**, not to copy any specific
proprietary program.

## Rules

1. **No copied code.** No source, pseudocode, or decompiler output from any
   proprietary binary is included, translated, or adapted here.
2. **No copied assets or data.** No icons, images, skins, fonts, packaged
   binaries, plugins, or license/key material from the benchmark product.
3. **No copied identifiers.** Public behavior is reimplemented using original
   names and structure. Short factual names needed for interoperability (e.g.
   protocol keywords) come from public standards, not from the target.
4. **Public specifications.** Behavior is derived from public standards and
   documentation: RFC 4251-4256 (SSH), RFC 854/855 (Telnet), ECMA-48 / xterm
   control sequences, and the public documentation of the crates we depend on.
5. **Observable behavior only.** Where a behavioral benchmark is used, only
   externally observable behavior is considered, and implementations are
   written independently from that specification.
6. **Trademarks.** Product names are used only for descriptive/benchmarking
   purposes and do not imply affiliation or endorsement.

## Contributions

By contributing you affirm that your contribution is original or is
compatibly-licensed third-party code (e.g. crates.io dependencies), and that it
does not include material from a proprietary product.

## Provenance note

An external, unpublished reverse-engineering workbench may exist alongside this
project for behavioral study. It is intentionally **not** part of this
repository, is not a build input, and its contents are not copied into any
source file here.
