# Architecture

Architectural decisions, invariants, and patterns for this mission.

**What belongs here:** Lifecycle invariants, ownership rules, preferred abstractions.
**What does NOT belong here:** Per-command execution details (use `.factory/services.yaml`).

---

- Prefer deleting the old prewarm abstraction over preserving a broken optimization path.
- Keep one authoritative notion of user-window identity.
- A visible user-facing window must never retain hidden/support/prewarm identity.
- Hidden/support windows, if they remain, must stay outside user-visible creation, titlebar controls, and close accounting.
- Primary-main-only startup work belongs only to the true primary visible main window.
- Window controls and window-state subscriptions must bind to the owning renderer's native window, not a shared module-level handle.
- Multi-window correctness is the primary goal; performance optimizations are acceptable only if they preserve these invariants.
