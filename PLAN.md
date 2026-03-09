# rust-cohle Learning Plan

## Overview

**Goal:** Learn Rust throughout 2026 by building `rc`, a TUI learning companion app that tracks progress through Rust textbooks and integrates with the Anthropic API for interactive learning features (explanations, quizzes, exercise generation).

**Pace:** ~1hr/week on weekends

**Books:**
- *The Rust Programming Language* (TRPL)
- *Rust for Rustaceans* (RfR)

**CLI command:** `rc`

**Background:** Familiar with C from university systems and parallel computing courses. Started rustlings previously but didn't finish. Working software developer leveling up on both Rust and agentive AI development.

## Current Status

- **Phase:** 1 — CLI Foundation
- **Week:** 1
- **Last session:** 2026-03-08
- **Next goal:** Week 2 — Define `Chapter`, `Exercise`, and `Progress` structs in `model.rs`
- **Blockers:** None

## Tech Stack

| Concern | Crate | Rationale |
|---|---|---|
| TUI framework | `ratatui` + `crossterm` | Standard Rust TUI stack |
| CLI args | `clap` | Idiomatic CLI argument parsing |
| Async runtime | `tokio` | Required for HTTP/LLM calls |
| HTTP client | `reqwest` | LLM API calls |
| Serialization | `serde` + `serde_json` | Data persistence, API payloads |
| Local storage | JSON files → `rusqlite` (Phase 4) | Start simple, migrate when it teaches something |
| Config | `toml` (Phase 4) | Standard Rust config format |
| Error handling | `thiserror` | Ergonomic custom error types |
| LLM API | Anthropic Messages API | Already in the ecosystem |

## Repo Structure

```
rust-cohle/
├── src/
│   ├── main.rs            # Entry point, CLI/TUI startup
│   ├── lib.rs             # Library root
│   ├── model.rs           # Chapter, Exercise, Progress types
│   ├── storage.rs         # JSON persistence (→ SQLite in Phase 4)
│   ├── curriculum.rs      # Load/query curriculum data
│   ├── tui/               # (Phase 2+) TUI screens and widgets
│   └── llm/               # (Phase 3+) Anthropic API client
├── data/
│   └── curriculum.json    # Chapter definitions for both books
├── tests/                 # Integration tests
├── PLAN.md                # This file
└── Cargo.toml
```

## AI Development Progression

The project is also a vehicle for learning to work effectively with LLMs as a development tool. Each phase shifts how you use the LLM:

| Phase | Your Role | LLM's Role |
|---|---|---|
| 1 — CLI Foundation | Learner: study what it generates | Scaffolder: set up project, generate models, explain compiler errors |
| 2 — TUI Layer | Driver: write first, then ask for help | Pair programmer: review your code, suggest improvements |
| 3 — LLM Integration | Architect: describe what you want | Implementer: write async/API code to your spec |
| 4 — Polish & Depth | Lead: make design decisions, delegate | Senior dev: handle complex features end-to-end |

---

## Phase 1: CLI Foundation (Weeks 1–10)

**Book chapters:** TRPL Ch. 1–9, RfR Ch. 1

**Goal:** Build a working CLI tool that manages a learning curriculum. By the end of this phase, `rc` can list chapters from both books, track completion, and persist progress to disk.

**Key Rust concepts:** Ownership, borrowing, structs, enums, pattern matching, `Result`/`Option`, modules, basic error handling, traits (`Display`, `Serialize`/`Deserialize`), unit testing.

### Sessions

- [x] **W1** — `cargo init`, set up project structure, `rc` compiles and prints a welcome message
- [ ] **W2** — Define `Chapter`, `Exercise`, and `Progress` structs and enums in `model.rs`
- [ ] **W3** — Create `data/curriculum.json` with both books' chapters; deserialize with serde in `curriculum.rs`
- [ ] **W4** — `rc list` — pretty-print the chapter list to stdout using clap for CLI parsing
- [ ] **W5** — `rc status` — show overall progress summary (chapters completed / total)
- [ ] **W6** — Progress persistence: save and load completion state to a JSON file in `storage.rs`
- [ ] **W7** — `rc complete <chapter>` — mark chapters as done, update persisted state
- [ ] **W8** — Unit tests for the data model and storage layer
- [ ] **W9** — Custom error type with `thiserror`, remove all `unwrap()` calls
- [ ] **W10** — Review and refactor session: clean up modules, improve API boundaries

---

## Phase 2: TUI Layer (Weeks 11–24)

**Book chapters:** TRPL Ch. 10–13, RfR Ch. 2–3

**Goal:** Add a terminal UI on top of the Phase 1 core. By the end of this phase, `rc` launches an interactive TUI with a dashboard, chapter browser, detail views, and persistent state.

**Key Rust concepts:** Generics, traits (deep dive), lifetimes, closures, iterators, module system at scale, trait objects vs generics, `String` vs `&str`.

### Sessions

- [ ] **W11** — Add `ratatui` + `crossterm` dependencies; basic render loop that shows a message and quits on 'q'
- [ ] **W12** — Dashboard screen: show overall progress as a bar and percentage
- [ ] **W13** — Chapter list screen with selectable items (highlight current selection)
- [ ] **W14** — Keyboard navigation: up/down to select, enter to open detail, esc to go back
- [ ] **W15** — Chapter detail view: show exercises and their completion status
- [ ] **W16** — Refactor: split into `lib.rs` (core logic, data, storage) and `main.rs` (TUI entry point)
- [ ] **W17** — Styling: colors, borders, and layout with `ratatui::Layout`
- [ ] **W18** — Add a notes field per chapter, editable within the TUI
- [ ] **W19** — Status bar along the bottom with keybinding hints
- [ ] **W20** — Persist TUI state: remember last viewed chapter and scroll position across sessions
- [ ] **W21–22** — Study weeks: read TRPL Ch. 10–13 (generics, traits, lifetimes, closures/iterators)
- [ ] **W23–24** — Refactor pass: apply what you read — improve trait usage, use iterators idiomatically

---

## Phase 3: LLM Integration (Weeks 25–36)

**Book chapters:** TRPL Ch. 15–17, RfR Ch. 4–6

**Goal:** Make the app smart by integrating with the Anthropic Messages API. By the end of this phase, `rc` can explain concepts, generate exercises, and quiz you — all within the TUI with streaming responses.

**Key Rust concepts:** Async/await, `Future`s, error handling patterns, smart pointers (`Box`, `Arc`, `Mutex`), API client design, cargo features.

### Sessions

- [ ] **W25** — Add `tokio` + `reqwest`; make a basic Anthropic API call from a test or one-off binary
- [ ] **W26** — API key config: load from environment variable or a config file
- [ ] **W27** — "Explain this chapter" feature: send chapter context to the API, display the response in the TUI
- [ ] **W28** — Streaming API responses: render tokens as they arrive in a TUI pane
- [ ] **W29** — "Quiz me" feature: LLM generates multiple-choice or short-answer questions for a chapter
- [ ] **W30** — Quiz display: dedicated TUI screen for answering and reviewing quiz questions
- [ ] **W31** — "Generate exercise" feature: LLM creates a coding exercise tailored to the current chapter
- [ ] **W32** — Error handling for network failures, timeouts, and rate limits (retry logic, user-facing messages)
- [ ] **W33** — Loading states and spinners while waiting for API responses
- [ ] **W34–36** — Study and refactor: read TRPL chapters on smart pointers and concurrency; apply to codebase

---

## Phase 4: Polish & Depth (Weeks 37–48)

**Book chapters:** TRPL Ch. 18–20, RfR Ch. 7–10

**Goal:** Add depth, robustness, and polish. Migrate storage to SQLite, add a config system, build integration tests, and implement a code-review feature. By the end of this phase, `rc` is a complete, well-tested application.

**Key Rust concepts:** Advanced patterns, macros (`macro_rules!`), trait objects, dynamic dispatch, FFI-adjacent code (SQLite), testing strategies, project organization at scale.

### Sessions

- [ ] **W37–38** — Spaced repetition: track when chapters were last reviewed, prompt reviews at intervals
- [ ] **W39–40** — Migrate storage from JSON to SQLite using `rusqlite`
- [ ] **W41–42** — Config system: TOML config file for themes, keybindings, and API key
- [ ] **W43–44** — Integration tests for the full application
- [ ] **W45–46** — "Code review" feature: paste or pipe code into `rc`, get LLM feedback
- [ ] **W47–48** — Performance pass, polish, write a proper README

---

## Phase 5: Stretch Goals (Weeks 49+)

**Book chapters:** RfR Ch. 11–12

**Goal:** Optional extensions that push into advanced Rust territory. Pick whichever are most interesting.

**Key Rust concepts:** Concurrency (`Send`/`Sync`, channels, `rayon`), advanced type system, `unsafe`, FFI, publishing, CI/CD.

### Goals

- [ ] Publish `rc` to crates.io
- [ ] GitHub Actions CI/CD pipeline
- [ ] Plugin system or custom `ratatui` widgets
- [ ] Concurrent exercise runner (background threads)
- [ ] Contribute to an open-source Rust project using skills learned
