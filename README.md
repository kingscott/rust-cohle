# rc — Rust learning companion

A TUI app for tracking progress through Rust textbooks, built as a vehicle for learning Rust itself.

## What it does

`rc` tracks chapter completion across *The Rust Programming Language* and *Rust for Rustaceans*, and will eventually integrate with the Anthropic API for interactive features like explanations, quizzes, and exercise generation.

## Status

Early development — Week 1 of a phased build-out.

| Phase | Focus | Status |
|---|---|---|
| 1 | CLI foundation | In progress |
| 2 | TUI layer | Not started |
| 3 | LLM integration | Not started |
| 4 | Polish & depth | Not started |

## Usage

```
rc              # print welcome message
rc --help       # (coming soon)
rc list         # (Phase 1) list chapters
rc status       # (Phase 1) show progress summary
rc complete <chapter>  # (Phase 1) mark a chapter done
```

## Building

Requires Rust (stable). Clone the repo, then:

```bash
cargo build
cargo run
```
