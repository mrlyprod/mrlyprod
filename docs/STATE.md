# State tiers

The rule: state belongs to the kernel if replaying the journal without it
would change what the screen means. It's shell-local if it only changes this
second's look.

## Tier 1 - kernel + journal (deterministic, replayable)

Every verb not prefixed `journal.` is stamped with `now`, pushed to the
journal, then run. Replay fires every stamped call bare, in order, on boot.
Covers user app verbs, `nav.*`, `sys.shot`, `sys.dismiss`, and the host
`.set` calls a pick produces.

**Beat** - the 8/s `setInterval` drives the focused app's `slot.beat` call.
Beats are journaled like any verb, but consecutive step beats coalesce: the
tail row's `n` grows in place (capped at the verb's kernel clamp), so a run
journals as one "advance n" row rewritten and persisted every beat. Replay
only needs the stamped calls, not the original cadence - visual speed and
replayed effects can diverge, never the outcome.

**Compaction** - once the journal hits 500 calls, the shell freezes the
kernel directly (bypassing the journal), stores the freeze as a snapshot, and
drops the trailing calls. A saved journal is "snapshot + up to 500 calls."
Freeze/thaw carries the route, tick, the kernel's own 100-entry call ring,
notices, and every app's saved state. Only one app is ever open; `nav.open`
replaces the route.

## Tier 2 - shell-intercepted verbs (client-side, never reaches the kernel)

- **`ask.*`** - yes/no resolve the shell's confirm promise (journal
  reset/import guards). Never journaled.
- **`face.full`** - calls fullscreen on a DOM element. No kernel involvement.
- **`splash.*`** - on/off hides or restores the whole shell (the splash
  screen, spec in SPLASH.md). Pure client state; beats pause while hidden.
- **`journal.*`** (reset/export/import) - operate straight on IndexedDB, not
  the kernel. Import only becomes real state on the next boot's replay.

## Tier 3 - plain shell locals (pixels only)

- **`toast`** - content comes from journaled kernel notices, but the 4s
  visibility window is a local timer. The build-mismatch discard toast has no
  kernel notice behind it at all.
- **Confirm** - the `ask` dialog on screen is pure client state, the one
  floating overlay left.
- **Tints** - header/card/footer colors from an unseeded `Math.random()` pool,
  cached on the DOM node. Deterministic only when `settings.fill` is pinned
  off `"random"`.
- **Mark frame** - footer animation index advances on wall-clock
  `setInterval`, decoupled from kernel `tick`; replays show different frames.
- **Projector** - in gpu render mode the shell tweens shader uniforms
  between kernel keyframes on rAF (mandelbrot, julia, solids). Cosmetic
  only: reduced motion switches it off and honest 8/s stepping returns.
  Frames are never journaled.
- **Strips** - life ships the generations of its last run as a `strip`; the
  shell plays them out across the beat window. Pixels only.

## Known leaks vs the labeled-film claim

The claim: every screenshot is a labeled projection of known kernel
state, so a lived session replays into perfectly labeled film. True
for kernel-known pixels, but a handful of unjournaled things break it for a
raw browser screenshot: the confirm dialog, toast timing (plus the
unjournaled discard toast), random tints, the mark frame, and mid-tween
projector/strip frames.

`sys.shot` is the exception, fully inside the claim: an ordinary journaled
verb, rendered by the kernel from state it already knows (the focused app's
`frame` field), never from the live page. A `sys.shot` capture replays
bit-for-bit; a real browser screenshot does not.

The face shot (docs/FACE.md) is a surface-side projection of known kernel
state - a pure read over `peek`, never a journaled verb.
