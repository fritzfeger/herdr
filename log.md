# Workspace Log

## 2026-09-11 — fix(protocol): preserve underline styles during frame composition
<!-- pi-commit-log: reviewed-snapshot=fnv1a-80ec3820 -->
<!-- pi-commit-meta: {"schema":1,"sessionsSincePreviousCommit":{"since":"2026-09-07T23:53:01+02:00","items":[{"cwd":".","id":"01a075f6-8b67-7758-aa65-aea396f6974c","name":"herdr-agent-runtime","lastActivityAt":"2026-09-11T11:03:07.333Z"},{"cwd":".","id":"01a05e9b-fe65-7e5d-a9b0-53e8883c442d","name":"herdr-review","lastActivityAt":"2026-09-07T21:53:03.584Z"}]}} -->

- Review snapshot: `fnv1a-80ec3820`
- Review gate: fresh review
- Pi sessions since previous commit: `herdr-agent-runtime`, `herdr-review`
- Files: `src/protocol/render_ansi.rs`, `src/protocol/wire.rs`
- Summary: Preserve underline style modifier bits when converting composed frames back to ratatui buffers, with coverage for ANSI rendering.
- Why: Masking underline metadata during modifier conversion caused double, single, curly, and dotted underline styles to be lost during frame composition.
- Changelog decision: include — Rendered terminal output now preserves the intended underline style after frame composition.
