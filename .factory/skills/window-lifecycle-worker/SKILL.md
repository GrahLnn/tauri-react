---
name: window-lifecycle-worker
description: Rebuilds Rust/Tauri window lifecycle behavior, close accounting, and multi-window lifecycle regressions with tests-first discipline.
---

# Window Lifecycle Worker

NOTE: Startup and cleanup are handled by `worker-base`. This skill defines the work procedure.

## When to Use This Skill

Use this skill for backend-heavy or cross-layer features that change window identity, creation, visibility, activation, shutdown, support/prewarm handling, close accounting, or multi-window lifecycle flows.

## Work Procedure

1. Read `mission.md`, `AGENTS.md`, the assigned feature, and the specific assertion IDs in its `fulfills` array.
2. Inspect the current lifecycle path before editing: relevant Rust files, any frontend callers, and current tests covering the touched path.
3. Write failing tests first. Prefer Rust tests for lifecycle/close-accounting invariants; add TS tests only when a frontend bridge is part of the changed behavior.
4. Implement the smallest correct lifecycle redesign that satisfies the feature. If the old prewarm abstraction is the wrong model, remove or isolate it rather than preserving it.
5. Keep one authoritative notion of user-window identity. Visible windows must never remain hidden/support/prewarm windows.
6. If support/prewarm windows remain, ensure they cannot initiate user-visible creation flows, cannot recurse, and cannot affect user-window shutdown accounting.
7. Run targeted verification after each substantial change:
   - `cargo check --manifest-path src-tauri/Cargo.toml`
   - `bun test`
   - `bun run typecheck` when TS bindings or frontend call sites changed
8. Manually verify the real Tauri surface with `bun tauri dev` for the feature’s flows. Capture startup, open-window, close, reopen, and shutdown observations as applicable.
9. Before handoff, summarize exactly which lifecycle states changed, which assertions are now satisfiable, and any residual risks.

## Example Handoff

```json
{
  "salientSummary": "Replaced the old prewarm-first create path with a single user-window identity flow, removed support-window participation from visible creation, and fixed close accounting so only user windows drive app exit. Added regression tests for startup identity and mixed close events, then manually verified startup, second-window creation, close-one-of-many, and last-window shutdown in Tauri dev.",
  "whatWasImplemented": "Updated src-tauri/src/utils/window.rs and related callers so visible windows always resolve as user windows, support windows cannot recursively spawn or initiate user-visible creation, and should-exit logic filters to true user windows only. Added Rust tests covering startup identity classification, open-window count deltas, support-window isolation, and close accounting.",
  "whatWasLeftUndone": "",
  "verification": {
    "commandsRun": [
      {
        "command": "cargo check --manifest-path src-tauri/Cargo.toml",
        "exitCode": 0,
        "observation": "Rust backend compiled successfully after lifecycle redesign."
      },
      {
        "command": "bun test",
        "exitCode": 0,
        "observation": "Lifecycle regression tests and existing JS tests passed."
      },
      {
        "command": "bun run typecheck",
        "exitCode": 0,
        "observation": "TS bindings and frontend callers remained type-safe."
      }
    ],
    "interactiveChecks": [
      {
        "action": "Launch app via bun tauri dev and observe startup window state",
        "observed": "Exactly one visible interactive primary main window appeared; no ghost or support window surfaced."
      },
      {
        "action": "Open a second window, then close it while another visible window remains",
        "observed": "Visible user-window count changed by +1 then -1, and the app stayed alive with the remaining window still interactive."
      },
      {
        "action": "Close the final visible user window and relaunch",
        "observed": "App exited cleanly and next launch started with a clean primary window and no carried-over ghost state."
      }
    ]
  },
  "tests": {
    "added": [
      {
        "file": "src-tauri/src/utils/window.rs",
        "cases": [
          {
            "name": "visible windows never retain support identity",
            "verifies": "User-visible windows resolve as user windows rather than support/prewarm windows."
          },
          {
            "name": "support-window close events do not affect exit accounting",
            "verifies": "Only user windows participate in app-exit decisions."
          }
        ]
      }
    ]
  },
  "discoveredIssues": []
}
```

## When to Return to Orchestrator

- The feature requires a new testing hook, observability seam, or startup contract that would affect multiple future features and should be planned centrally.
- You discover the mission needs a third worker type because the remaining work is no longer backend-heavy.
- The only correct fix would violate mission boundaries or remove behavior the user explicitly asked to keep.
