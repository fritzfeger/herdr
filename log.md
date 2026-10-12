# Workspace Log

## 2026-10-10 — feat(api): add conditional agent prompt request schema
<!-- pi-commit-log: reviewed-snapshot=fnv1a-c4878ce2 -->
<!-- pi-commit-meta: {"schema":1,"sessionsSincePreviousCommit":{"since":"2026-09-29T22:06:04+03:00","items":[{"cwd":".","id":"01a075f6-8b67-7758-aa65-aea396f6974c","name":"herdr-agent-runtime","lastActivityAt":"2026-10-10T15:47:15.657Z"}]}} -->

- Review snapshot: `fnv1a-c4878ce2`
- Review gate: fresh review
- Pi sessions since previous commit: `herdr-agent-runtime`
- Files: `docs/next/api/herdr-api.schema.json`, `docs/next/website/src/content/docs/agent-automation.mdx`, `docs/next/website/src/content/docs/cli-reference.mdx`, `docs/next/website/src/content/docs/socket-api.mdx`, `src/api/mod.rs`, `src/api/schema.rs`, `src/api/schema/agents.rs`, `src/api/schema/tests.rs`, `src/api/server.rs`, `src/app/agents.rs`, `src/app/api.rs`, `src/app/api/agents.rs`, +15 more
- Summary: Add protocol v23 conditional agent prompting with `agent.prompt_if`, including expected terminal, agent session, revision, state sequence, and input submission token checks. Track submission tokens through terminal state, wire the request across API, CLI, headless, and runtime layers, and update schemas, docs, and integration coverage.
- Why: Allows automation to submit a prompt only when the target agent is still in the state previously observed, preventing stale or duplicate input after concurrent state changes.
- Changelog decision: include — Introduces a new public socket API request, updates the protocol version, and exposes conditional prompt behavior for CLI/API integrators.

## 2026-10-12 — fix(terminal): notify children after applying host palette updates
<!-- pi-commit-log: reviewed-snapshot=fnv1a-cfed341d -->
<!-- pi-commit-meta: {"schema":1,"sessionsSincePreviousCommit":{"since":"2026-10-10T18:01:19+02:00","items":[{"cwd":".","id":"01a05e9b-fe65-7e5d-a9b0-53e8883c442d","name":"herdr-review","lastActivityAt":"2026-10-10T16:44:48.906Z"},{"cwd":".","id":"01a075f6-8b67-7758-aa65-aea396f6974c","name":"herdr-agent-runtime","lastActivityAt":"2026-10-10T16:44:25.718Z"}]}} -->

- Review snapshot: `fnv1a-cfed341d`
- Review gate: fresh review
- Pi sessions since previous commit: `herdr-review`, `herdr-agent-runtime`
- Files: `docs/next/website/src/content/docs/configuration.mdx`, `src/app/mod.rs`, `src/app/theme_sync.rs`, `src/pane.rs`, `src/pane/terminal.rs`, `src/server/headless.rs`, `src/terminal/runtime.rs`
- Summary: Notify subscribed embedded applications after applying host palette updates, including updates with unchanged light/dark appearance.
- Why: Applications need to re-query current colors after reconnecting or receiving appearance and palette reports separately. Add regression coverage and document the notification behavior.
- Changelog decision: include — Embedded applications receive color-scheme notifications after host colors are updated, allowing them to refresh their colors correctly.
