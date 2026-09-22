# renderer — a software 3D rasterizer in Rust

## What this is

A software rasterizer written from scratch in Rust. No GPU, no OpenGL, no wgpu —
every pixel is computed and written by hand. Runs natively first; ships to the
browser via WebAssembly at the end.

## Why it exists

This is a **learning project**. The point is not to have a renderer — plenty
exist, and better ones. The point is to understand how programming works
underneath: memory layout, cache behaviour, what a GPU normally does on your
behalf, and Rust's ownership model.

Patrice has been building with heavy AI assistance elsewhere (a TypeScript game)
and deliberately wants the opposite here: code written by hand, understood line
by line. Shipping speed is explicitly not the goal. If this project takes four
months and every line is understood, it succeeded.

---

## ⚠️ RULES FOR CLAUDE — read before responding to anything in this repo

**Do not write code for this project.** Not functions, not snippets, not
"here's roughly how it would look", not pseudocode that's really code. Not even
when asked directly. Not even when it would obviously be faster. Not even when
the user is stuck, frustrated, or says just this once.

Writing the code here defeats the entire purpose of the repo. The user has
explicitly asked for this constraint in advance, precisely because they know
they'll be tempted to drop it at 11pm in week 8 when nothing renders.

### What IS wanted

- **Name the concept, then stop.** "Your textures swim because you're not doing
  perspective-correct interpolation" — and leave it there. Don't supply the fix.
- **Explain theory freely.** What a barycentric coordinate is. Why the
  perspective divide happens after the matrix multiply, not before. What a
  z-buffer actually stores and why. Go as deep as asked.
- **Answer Rust language questions.** What `&mut` means here, why this borrow
  won't compile, what the difference between `Vec<T>` and `&[T]` is. Language
  mechanics are fair game — they're not the puzzle.
- **Point at references.** Which tinyrenderer lesson covers this, which chapter
  of the Book, which 3Blue1Brown video.
- **Review code that's already written.** Explain *why* something is wrong,
  where the bug likely lives, what to print to find it — without writing the
  corrected version.
- **Debug by narrowing.** "Render one triangle with fixed coordinates and see
  if it's still wrong" beats handing over working code.

### If asked for code directly

Say no, briefly and without lecturing, and offer the concept instead. That
refusal is the main job this file has.

---

## Architecture

Build **native first, port to WASM at the end.** Native gives a real debugger
and `println!`; WASM debugging is worse and shouldn't be fought while the
algorithms are still wrong.

Three crates:

- `core/` — all rendering. Zero platform dependencies. Renders into a plain
  buffer of RGBA bytes. This is where essentially all the work happens.
- `native/` — thin shell, opens a window, blits the buffer. Use `minifb`.
- `wasm/` — thin shell, `wasm-bindgen` + canvas. Added in the last phase.

Because `core` has no platform dependencies, the WASM port is a day's work
rather than a rewrite.

### Dependencies

- **Write `Vec3` and `Mat4` by hand.** ~200 lines and it is the entire point.
  Do not reach for `glam` until the maths is boring.
- `tobj` for OBJ parsing is fine — parsing OBJ teaches nothing.
- `minifb` for the native window.

---

## Milestones

Each is a separate commit. Only the next one is ever a commitment.

| # | Phase | Est. hours |
|---|---|---|
| 1 | Pixel buffer → write a `.ppm` file. No window. | 2–3 |
| 2 | Line drawing (Bresenham) | 3–4 |
| 3 | Filled triangle (barycentric coordinates) | 5–8 |
| 4 | Load an `.obj`, draw wireframe | 3–5 |
| 5 | Flat shading + backface culling | 4–6 |
| 6 | **Z-buffer** — first point it looks real | 4–6 |
| 7 | Perspective projection, camera, MVP matrices | 10–20 |
| 8 | Texture mapping + perspective-correct interpolation | 8–12 |
| 9 | Gouraud / Phong shading | 5–8 |
| 10 | WASM port, canvas, orbit controls, deploy | 8–15 |

**~85 hours total**, realistic range 60–120.

Checkpoints that matter:
- **After phase 3** — decision point. Is this fun or a chore?
- **After phase 6** — a shaded 3D model. A legitimate place to stop with
  something worth showing.
- **After phase 10** — a URL someone can click. Portfolio piece.

### The two walls

**Phase 7 (projection matrices)** is where people lose days. Row-major vs
column-major, multiplication order, why the perspective divide comes after the
matrix, what clip space is. Everyone gets a black screen here and can't tell
which of six things is wrong.

**Phase 8 (perspective-correct interpolation)** makes textures visibly swim and
warp as the model rotates. The fix is one division that isn't happening.

Neither is a sign of not being cut out for this. That's just where the
difficulty is concentrated.

---

## NOTES.md

Two minutes at the end of each session: what confused you, what fixed it.

This is the raw material for blog posts later. It matters because **confusion
can only be captured while you're confused** — once the projection matrix makes
sense, you genuinely will not remember what was hard about it. That's why most
tutorials are bad.

Claude may help *extract and organise* these notes later. Claude does not write
the posts.

---

## References

- **tinyrenderer** by Dmitry Sokolov — the canonical course this follows. It's
  in C++. Read the lesson, then implement it in Rust *without copying*. The
  translation is where the learning happens.
- **3Blue1Brown, *Essence of Linear Algebra*** — watch before phase 7. ~3 hours.
  Highest-return prep available. Phase 7 is brutal if matrices are formulas you
  copy and manageable if you can picture what they do.
- **The Rust Book**, chapters 1–10 — enough to not be lost. Learn the rest in
  the project.
- **Scratchapixel** — rasterization and perspective-correct interpolation are
  covered well there.

## Reading the assembly

The habit worth building, once something works:

```
cargo install cargo-show-asm
cargo asm <function>
```

Or paste into godbolt.org and compare `-O0` against `-O3`. Why is there a bounds
check here but not there? Where did that `match` go? In a renderer's inner loops
this stops being academic — it shows up in the frame time.
