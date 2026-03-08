# rust-cohle — Claude Rules

## Branch Naming

Branches must follow the pattern: `<prefix>/<short-descriptor>`

- **Prefix:** the user's handle (e.g. `kingscott`)
- **Descriptor:** 5–7 words in kebab-case describing the work (e.g. `w1-cargo-init-rc-project-setup`)

Always create new branches using this convention before starting work on a new week or feature.

## Worktree Naming

Worktree folder names must match the branch descriptor, dropping the `<prefix>/` portion.

- Branch `kingscott/w1-cargo-init-rc-project-setup` → worktree folder `w1-cargo-init-rc-project-setup`
- Worktrees live under `~/workspace/trees/rust-cohle/`
