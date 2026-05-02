---
name: no-use-effect
description: Refactor React components away from direct useEffect usage. Use when a component is using useEffect for derived state, prop-to-state sync, remount emulation, or one-time setup that should instead be expressed with render-time derivation, event handlers, keyed remounts, or a small mount-only helper.
---

# No Use Effect

Prefer render-time logic and explicit component structure over ad-hoc `useEffect`.

## Rules

- Derive values during render. Do not use `useEffect` just to compute state from props or fetched data.
- Put user-driven mutations in event handlers, not effects.
- Use keyed remounts when the real intent is "treat this as a fresh component for a new entity".
- Use a tiny mount-only helper for one-time external synchronization when there is no clearer render-time or event-time expression.
- Keep the side-effect owner explicit in the component tree. The component that conceptually owns the external sync should mount the owner node.

## Preferred Patterns

### Derived state

- Compute directly from props, query results, and local state.
- Remove `useEffect + setState` pairs whose only job is to keep another value in sync.

### Reset on identity change

- Split the component.
- Put the resettable subtree behind a `key`.
- Use mount-only logic inside the keyed subtree if needed.

### One-time external setup

- Wrap the setup in a focused component such as `FeatureOwner`.
- Call a shared `useMountEffect(...)` helper there.
- Keep that owner component mounted only when the setup should happen.

## Smell Tests

- "This effect copies props into state."
- "This effect only exists to act like remount."
- "This effect runs because some ID changed, but the real intent is a fresh owner instance."
- "This effect exists only to trigger one external call when a feature becomes active."

## Component Order

1. Hooks
2. Local state
3. Computed values
4. Event handlers
5. Early returns
6. Render
