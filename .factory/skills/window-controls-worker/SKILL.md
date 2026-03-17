---
name: window-controls-worker
description: Reworks frontend bootstrap, titlebar controls, drag regions, and per-window state binding so each renderer only controls its own native window.
---

# Window Controls Worker

NOTE: Startup and cleanup are handled by `worker-base`. This skill defines the work procedure.

## When to Use This Skill

Use this skill for frontend features that change bootstrap role resolution, app-shell gating, titlebar/window-control mounting, drag-region behavior, maximize/fullscreen indicators, or per-window handle/state ownership.

## Work Procedure

1. Read `mission.md`, `AGENTS.md`, the assigned feature, and the validation assertions it fulfills.
2. Inspect the full renderer path before editing: bootstrap logic, `App` shell gating, control portals, titlebar components, and window-state hooks.
3. Write failing tests first. Prefer focused TS tests for bootstrap decisions, shell/control mounting, and per-window state ownership.
4. Remove shared or module-scope control assumptions that can bind one renderer to the wrong native window.
5. Keep one consistent control-binding pattern across visible main windows. Secondary windows must not accidentally run primary-main-only startup work.
6. Hidden/support/non-user windows must not mount user-facing titlebar controls or the full interactive shell.
7. After implementation, run targeted validation:
   - `bun test`
   - `bun run typecheck`
   - `bun run lint`
   - `cargo check --manifest-path src-tauri/Cargo.toml` if command bindings or Rust-side metadata changed
8. Manually verify the real Tauri window surface with at least two visible windows. Exercise close, minimize, maximize/fullscreen, drag, and reopen flows relevant to the feature.
9. In the handoff, be explicit about which controls were tested in which window and what proved they stayed window-local.

## Example Handoff

```json
{
  "salientSummary": "Reworked bootstrap and titlebar control ownership so each renderer binds to its own native window, hidden/support windows stop mounting user-facing controls, and maximize state stays local per window. Added TS tests for bootstrap role handling and control mounting, then manually verified control isolation with two windows in Tauri dev.",
  "whatWasImplemented": "Updated bootstrap logic, App shell gating, and window-control components so primary-only startup work runs only in the primary visible main window, hidden/support windows do not mount the shell or titlebar portals, and close/minimize/maximize/drag actions are bound to the current renderer’s native window handle instead of a shared module-scope handle.",
  "whatWasLeftUndone": "",
  "verification": {
    "commandsRun": [
      {
        "command": "bun test",
        "exitCode": 0,
        "observation": "Bootstrap and control-binding tests passed."
      },
      {
        "command": "bun run typecheck",
        "exitCode": 0,
        "observation": "Frontend bootstrap and control changes remained type-safe."
      },
      {
        "command": "bun run lint",
        "exitCode": 0,
        "observation": "Lint passed with the repo's existing warnings only."
      }
    ],
    "interactiveChecks": [
      {
        "action": "Open two visible windows and click minimize/maximize/close controls in the secondary window",
        "observed": "Only the secondary window changed state; the primary window remained unchanged until OS focus transfer after close."
      },
      {
        "action": "Drag the titlebar background and then click explicit titlebar buttons",
        "observed": "Background drag moved the current window, while buttons stayed clickable and did not start dragging."
      },
      {
        "action": "Open, close, and reopen an additional window",
        "observed": "The reopened window had fresh control ownership and its maximize indicator reflected its own state rather than a stale sibling state."
      }
    ]
  },
  "tests": {
    "added": [
      {
        "file": "tests/bootstrap.logic.test.ts",
        "cases": [
          {
            "name": "secondary windows skip primary-only startup work",
            "verifies": "Only the primary visible main window runs primary-main-only bootstrap effects."
          },
          {
            "name": "hidden windows do not mount user-facing controls",
            "verifies": "Non-user windows skip shell and titlebar control mounting."
          }
        ]
      }
    ]
  },
  "discoveredIssues": []
}
```

## When to Return to Orchestrator

- The correct fix depends on backend window metadata or lifecycle guarantees that do not exist yet.
- Multi-window manual verification is impossible because the desktop stack cannot be started reliably in the current environment.
- You discover a broader UI architecture issue outside window/bootstrap/control scope that should be decomposed separately.
