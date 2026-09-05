# Workspace Log

## 2026-09-05 — feat(api): add conditional agent prompt requests
<!-- pi-commit-log: reviewed-snapshot=fnv1a-3a1d8d7f -->
<!-- pi-commit-meta: {"schema":1,"sessionsSincePreviousCommit":{"since":"2026-09-04T21:59:13+03:00","items":[{"cwd":".","id":"01a06e5c-f094-7758-aa65-ae945f514822","name":"herdr-update","lastActivityAt":"2026-09-05T06:17:17.695Z"}]}} -->

- Review snapshot: `fnv1a-3a1d8d7f`
- Review gate: fresh review
- Pi sessions since previous commit: `herdr-update`
- Files: `docs/next/api/herdr-api.schema.json`, `docs/next/website/src/content/docs/agent-automation.mdx`, `docs/next/website/src/content/docs/cli-reference.mdx`, `docs/next/website/src/content/docs/socket-api.mdx`, `src/api/mod.rs`, `src/api/schema.rs`, `src/api/schema/agents.rs`, `src/api/schema/tests.rs`, `src/api/server.rs`, `src/app/agents.rs`, `src/app/api.rs`, `src/app/api/agents.rs`, +10 more
- Summary: Add conditional agent prompt requests across the API, CLI, runtime, schema, documentation, and tests, including expected session/state metadata and input submission tokens.
- Why: Allows automation to submit prompts only when the targeted terminal and agent session still match the observed state, preventing stale requests from sending input to a changed session.
- Changelog decision: include — API and CLI users can now conditionally prompt agents with state checks, avoiding input delivery when the target session has changed.
