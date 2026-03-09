# rc

> *"I know what I am. And after all these years, there's a comfort in that."*

A TUI learning companion for working through Rust textbooks. Named after the detective. Built with the language. The irony isn't lost on anyone.

## What it does

Tracks chapter completion across *The Rust Programming Language* and *Rust for Rustaceans*. Eventually it'll hook into the Anthropic API — explanations, quizzes, generated exercises. The kind of thing that makes you feel like the universe is becoming aware of itself through your compiler errors.

## Status

Early days. The borrow checker will humble you. That's the point.

| Phase | Focus | Status |
|---|---|---|
| 1 | CLI foundation | In progress |
| 2 | TUI layer | Not started |
| 3 | LLM integration | Not started |
| 4 | Polish & depth | Not started |

## Usage

```
rc                     # hear what rc has to say
rc --help              # (coming soon)
rc list                # (Phase 1) list chapters
rc status              # (Phase 1) show progress summary
rc complete <chapter>  # (Phase 1) mark a chapter done
```

## Building

Requires Rust (stable). Clone the repo, then:

```bash
cargo build
cargo run
```
