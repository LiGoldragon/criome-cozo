# criome-cozo

Shared CozoDB wrapper for Criome agents (Samskara, Lojix transpiler, and others).

This crate provides DB primitives on top of the `cozo` crate using Sema-style
conventions. It handles database lifecycle (open, health-check), CozoScript
execution, and multi-statement script splitting for `.cozo` files.

## Modules

- `db` — `CriomeDb` wrapper around `cozo::DbInstance` (memory and SQLite backends)
- `error` — `CozoError` enum for all fallible operations
- `script` — utilities for loading and splitting multi-statement `.cozo` files
