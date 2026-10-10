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
