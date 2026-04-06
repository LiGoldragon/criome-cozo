# CLAUDE.md

criome-cozo — shared CozoDB wrapper for Criome agents. Thin wrapper around
cozo-ce providing DB lifecycle, CozoScript execution, multi-statement script
splitting, and immutable query variants. No business logic.

## VCS

Jujutsu (`jj`) is mandatory. Git is the backend only. Always pass `-m` to
`jj` commands.

## Language Policy

- **Rust** only for application logic.
- **Nix** only for builds and dev shells.
