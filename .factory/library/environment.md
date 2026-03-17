# Environment

Environment variables, external dependencies, and setup notes.

**What belongs here:** Required env vars, external services, setup notes, platform quirks.
**What does NOT belong here:** Service ports/commands (use `.factory/services.yaml`).

---

- Platform: Windows 10.
- Preferred shell for execution: PowerShell.
- Main local validation stack: `bun tauri dev` with frontend served on `http://localhost:3000`.
- Optional dev-only script source may reference `http://localhost:8097`, but the mission must not depend on it.
- No new third-party credentials are required for this mission.
- There is an existing dirty mission-context change in `src-tauri/src/utils/window.rs`; workers should treat it as in-scope context, not unrelated noise.
