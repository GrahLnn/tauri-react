# Windows Lifecycle Surface

Mission-specific notes about the window lifecycle code surface.

**What belongs here:** Hot files, ownership boundaries, and pitfalls specific to this mission.
**What does NOT belong here:** Generic environment setup or service commands.

---

## Hot Files

- `src-tauri/src/utils/window.rs`
- `src-tauri/src/utils/core.rs`
- `src-tauri/src/lib.rs`
- `src/flow/bootstrap/index.ts`
- `src/flow/bootstrap/logic.ts`
- `src/App.tsx`
- `src/windowctrl/windows.tsx`
- `src/windowctrl/macos.tsx`
- `src/flow/windowMaximized.ts`
- `src/flow/template_board/events.ts`

## Known Pitfalls

- Recursive preparation can happen when hidden/support windows are allowed to participate in visible-window creation flows.
- Module-scope window-handle caching can bind controls to the wrong window in multi-window sessions.
- Startup ordering matters: role resolution, app shell mounting, and primary-main-only side effects must not fight each other.
- Close accounting must distinguish visible user windows from hidden/support windows.
- If logs show `startup: page load finished for main` and the process still exits immediately afterward, treat the remaining blocker as a post-page-load native shutdown issue rather than an initial bootstrap/platform-detection issue.
