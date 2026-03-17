# User Testing

Validation surface findings and runtime testing guidance.

**What belongs here:** Real-surface validation notes, setup constraints, concurrency guidance, gotchas.
**What does NOT belong here:** Generic architecture notes (use `architecture.md`).

---

## Validation Surface

- Primary validation surface: the Tauri desktop window itself.
- Core flows to validate:
  - Startup reaches one visible interactive primary main window.
  - Opening a new window increases visible user-window count by exactly one.
  - Closing one of many visible windows keeps the app alive.
  - Closing the last visible user window exits cleanly.
  - Open-close-reopen preserves fresh control ownership and fresh maximize/fullscreen state.
- Evidence should include screenshots, console errors, and terminal traces for window count / role / shutdown behavior.

## Validation Concurrency

- Real desktop-window validation max concurrent validators: **1**.
- Rationale:
  - Windows + Tauri desktop instances are relatively heavy.
  - Dry run observed roughly +20 processes and ~876 MB peak working-set growth during startup/build activity.
  - Port `3000` ownership can be noisy in this environment, so parallel desktop validation would add avoidable flakiness.

## Runtime Notes
- Desktop unsafe validation for the typed-prewarm mission is currently deferred/manual; implementation workers should still preserve desktop-surface correctness assumptions.
- Manual desktop validation for this mission should include homepage-triggered prewarm, discard-prewarm, discard-then-open, and open-close-reopen flows in addition to baseline startup/open/close checks.
- For `window-lifecycle-hardening`, final contract coverage is limited to manual desktop checks for `VAL-CONTROLS-006`, `VAL-CONTROLS-007`, `VAL-CROSS-004`, and `VAL-CROSS-005`; automated evidence from Bun/typecheck/lint and scrutiny handoffs is supporting evidence only and must not be treated as final user-testing proof.
- No automated flow validators should be spawned for `window-lifecycle-hardening` on this mission revision because Mission AGENTS.md explicitly defers desktop-surface evidence to the user.

- Check ownership of `http://localhost:3000` before assuming a fresh dev stack.
- Validation runs should disable optional dev overlays when they obscure the real app shell; real-window evidence must come from the actual Tauri-visible surface, not overlay UI.
- `http://localhost:3000` is a readiness endpoint for this mission, not the authoritative user-testing surface.
- If `bun tauri dev --no-watch` exits early, inspect `.tmp/tauri-dev.log` and `.tmp/tauri-dev.err.log` before concluding that desktop validation is impossible.
- Validation launches should overwrite `.tmp/tauri-dev.log` and `.tmp/tauri-dev.err.log` on each run so validators do not rely on stale startup evidence.
- `bun run lint` currently exits successfully with warnings; treat unrelated warnings as background noise unless your feature changes them.
- If the redesign removes support/prewarm windows entirely, that is an acceptable and preferred outcome as long as user-visible flows remain correct.
