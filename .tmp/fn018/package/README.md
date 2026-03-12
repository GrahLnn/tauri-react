# `@grahlnn/fn`

Small functional helpers for TypeScript, centered around a practical matcher API.

It is designed for code that wants to stay in `match` / `catch` / `in` / `is`
form across:

- primitive literal unions
- Rust-enum-like single-key objects
- discriminated objects such as `{ kind: "edit", ... }`
- plain objects with top-level key selection
- recursively wrapped UI projection objects

## Install

```bash
bun add @grahlnn/fn
```

```bash
npm install @grahlnn/fn
```

## Exports

```ts
import {
  me,
  Ok,
  Err,
  Result,
  futry,
  I,
  K,
  S,
  B,
  C,
  W,
  T,
  call0,
  vec,
  udf,
  set,
  nul,
} from "@grahlnn/fn";
```

## `me(...)`

`me(...)` builds a matcher from a value.

It routes by shape:

- `null` / `undefined` -> empty matcher
- `string | number | boolean` -> enum matcher
- `{ Variant: payload }` with a single uppercase key -> union matcher
- other objects -> object matcher

### Primitive unions

```ts
import { me } from "@grahlnn/fn";

const status = me<"idle" | "loading" | "error">("loading");

const label = status.match({
  idle: () => "Idle",
  loading: () => "Loading",
  _: () => "Error",
});

const upper = status.into()((value) => value.toUpperCase());
const hit = status.catch("loading", "error")((value) => value);
```

### Rust-enum-like single-key objects

```ts
const shape = me<{ Circle: { r: number } } | { Rect: { w: number; h: number } }>({
  Circle: { r: 2 },
});

const area = shape.match({
  Circle: ({ r }) => Math.PI * r * r,
  Rect: ({ w, h }) => w * h,
});

const radius = shape.catch("Circle")((payload) => payload.r);
```

### Plain objects

Plain objects match by top-level keys.

```ts
const form = me({
  title: "Hello",
  count: 1,
  published: true,
});

const picked = form.catch("title", "count")(({ title, count }) => ({
  title,
  count,
}));

const one = form.catch("count")((value) => value + 1);
```

For a single-key lowercase object, `catch(...)` also keeps compatibility with
branch-like selectors:

```ts
const state = me({ detail: "detail_idle" as const });

const hit = state.catch("detail", "load_detail")((value) => value);
// "detail_idle"
```

### Discriminated objects

For objects like `{ kind: "edit", panel: "meta" }`, use `.as("kind")` or
`match("kind", handlers)`.

```ts
type Editor =
  | { kind: "idle" }
  | { kind: "edit"; panel: "meta" | "token"; validation: "ok" | "err" };

const editor = me<Editor>({
  kind: "edit",
  panel: "meta",
  validation: "ok",
}).as("kind");

const panel = editor.match({
  idle: () => null,
  edit: (payload) => payload.panel,
});
```

Equivalent explicit-key form:

```ts
const panel = me<Editor>({
  kind: "edit",
  panel: "meta",
  validation: "ok",
}).match("kind", {
  idle: () => null,
  edit: (payload) => payload.panel,
});
```

### Common matcher methods

Most matcher variants expose these operations where they make sense:

- `match(...)` for branching
- `is(...)` / `not(...)`
- `in(...)` / `not_in(...)`
- `into()(fn)` for piping the current payload/value
- `catch(...keysOrBranches)(fn)` for conditional extraction
- `path(pathSegments, wrap?)` for deep-path traversal

## `path(...)`

`path(...)` walks nested XState-style single-key objects and can optionally wrap
the result.

### Raw leaf extraction

```ts
type ShotValue = {
  view: {
    detail: {
      construct: "edit_platform" | "edit_token";
    };
  };
};

const leaf = me<ShotValue>({
  view: { detail: { construct: "edit_platform" } },
}).path(["view", "detail", "construct"]);

// "edit_platform" | "edit_token" | null
```

### Wrapped extraction

```ts
type ShotValue =
  | "summary"
  | {
      view:
        | "summary"
        | {
            detail:
              | "detail_idle"
              | { construct: "edit_platform" | "edit_token" };
          };
    };

const branch = me<ShotValue>({
  view: { detail: { construct: "edit_platform" } },
}).path(["view", "detail", "construct"], me);

const hit = branch.catch("edit_platform", "edit_token")((value) => value);
```

If the path misses, the wrapped form returns the empty matcher.

```ts
const miss = me({ view: "summary" as const }).path(
  ["view", "detail", "construct"],
  me,
);

miss.value; // null
```

## `me.struct(...)`

`me.struct(...)` recursively wraps projection outputs so nested values stay
matchable instead of becoming raw values that need manual re-wrapping.

This is mainly useful at selector boundaries.

```ts
const ui = me.struct({
  root: "detail" as const,
  editor: {
    kind: "edit" as const,
    panel: "meta" as const,
    validation: "ok" as const,
  },
  command: {
    kind: "load_detail" as const,
  },
});

const root = ui.catch("root")((value) =>
  value.match({
    detail: () => "DETAIL",
    _: () => "MISS",
  }),
);

const panel = ui.catch("editor")((editor) =>
  editor.match({
    edit: (payload) =>
      payload.catch("panel")((panel) =>
        panel.match({
          meta: () => "META",
          _: () => "MISS",
        }),
      ),
    _: () => "MISS",
  }),
);

const command = ui.catch("command")((command) =>
  command.catch("load_detail")(() => "LOAD"),
);
```

Recommended pattern for UI selectors:

```ts
export const hook = {
  useUi: () => useSelector(actor, (shot) => me.struct(project_ui(shot))),
};
```

`me.struct(...)` now also uses by-reference caching for object inputs:

```ts
const wrapped1 = me.struct(sameObject);
const wrapped2 = me.struct(sameObject);

wrapped1 === wrapped2; // true
```

This is only same-reference reuse. New immutable objects still produce new
wrappers by design.

## `me.eq`

`me.eq` provides explicit equivalence builders for selector boundaries.

```ts
const strictEq = me.eq.strict<string>();
const shallowEq = me.eq.shallow<{ root: string; visual: string }>();
const arrayEq = me.eq.arrayShallow<number>();
const byIdEq = me.eq.by((user: { id: number }) => user.id);
const optionalEq = me.eq.optional(me.eq.strict<"idle" | "loading">());
const tupleEq = me.eq.tuple(me.eq.strict<number>(), me.eq.strict<string>());

const uiEq = me.eq.struct({
  root: me.eq.strict<"detail" | "list">(),
  visual: me.eq.strict<"brief" | "detail">(),
  editor: me.eq.struct({
    kind: me.eq.strict<"idle" | "edit">(),
  }),
});
```

Available builders:

- `me.eq.strict()` -> `Object.is`
- `me.eq.shallow()` -> top-level shallow object / array comparison
- `me.eq.arrayShallow()` -> array item shallow comparison with `Object.is`
- `me.eq.by(project, eq?)` -> derive equality from a projection
- `me.eq.tuple(...eqs)` -> compare tuples positionally
- `me.eq.struct(shape)` -> compare only declared keys
- `me.eq.optional(eq)` -> lift an equality into `null | undefined`

## `me.select(...)` and `me.selector(...)`

`me.select(project, compare?)` packages a selector projection together with the
comparator to use at the subscription boundary.

```ts
const selectUi = me.select(
  (shot: { root: string; visual: string }) => ({
    root: shot.root,
    visual: shot.visual,
  }),
  me.eq.shallow(),
);

useSelector(actor, selectUi.project, selectUi.compare);
```

`me.selector(...)` is an alias with small combinators:

```ts
const base = me.select((shot: { root: string; visual: string }) => ({
  root: shot.root,
  visual: shot.visual,
}), me.eq.shallow());

const selectRoot = me.selector.map(base, (ui) => ui.root);
const selectTuple = me.selector.tuple(
  me.select((shot: { root: string; visual: string }) => shot.root),
  me.select((shot: { root: string; visual: string }) => shot.visual),
);
```

## `me.selectStruct(...)`

`me.selectStruct(project, compare?)` is a convenience helper for the common
pattern “project a plain object, wrap it with `me.struct`, and compare by the
raw projected value”.

```ts
const selectUi = me.selectStruct(
  (shot: Snapshot) => projectUi(shot),
  me.eq.struct({
    root: me.eq.strict(),
    visual: me.eq.strict(),
    editor: me.eq.struct({
      kind: me.eq.strict(),
    }),
  }),
);

const ui = useSelector(actor, selectUi.project, selectUi.compare);
```

## Empty matcher

`me(null)` and `me(undefined)` return an empty matcher.

```ts
const empty = me(null);

empty.value; // null
empty.into()(() => "x"); // null
empty.catch("anything")(() => "x"); // null
empty.match({ _: () => "ignored" }); // null
```

## `Result`

`Result<T, E>` is a small Rust-style result wrapper.

### Constructors

```ts
const ok = Ok(42);
const err = Err("boom");
```

### Pattern matching

```ts
const label = ok.match({
  Ok: (value) => `value:${value}`,
  Err: (error) => `error:${error}`,
});
```

### Main methods

```ts
ok.isOk();
ok.isErr();

ok.unwrap();
err.unwrap_err();

ok.unwrap_or(0);
err.or_else((error) => error.length);

ok.map((value) => value + 1);
err.map_err((error) => new Error(String(error)));

ok.to((value) => Ok(value * 2));

ok.tap(console.log);
err.tap_err(console.error);

ok.answer();
ok.name();
ok.raw;
```

### `futry(...)`

Convert a promise into `Promise<Result<T, E>>`.

```ts
const result = await futry(fetch("/api/user").then((r) => r.json()));

result.match({
  Ok: (user) => user,
  Err: (error) => {
    throw error;
  },
});
```

## Combinators and tiny helpers

The package also exports a few small functional helpers.

### Bird / SK-style combinators

```ts
I(x); // identity
K(x)(y); // constant
S(f)(g)(x);
B(g)(f)(x); // composition in this package's order
C(f)(b)(a);
W(f)(x);
T(x)(f);
call0(() => value);
```

### Tiny constructors

```ts
vec(); // []
udf(); // undefined
set(); // {}
nul(); // null
```

## Notes

- `me.struct(...)` is recursive by design and is best used at projection or
  selector boundaries.
- `path(...)` is intentionally conservative: it walks object keys and can match
  a final primitive segment, but it does not treat arrays as traversable paths.
- Uppercase single-key objects are treated as union-style variants. Other
  objects remain plain object matchers unless you opt into discriminant matching
  with `.as(key)` or `match(key, handlers)`.

## License

MIT
