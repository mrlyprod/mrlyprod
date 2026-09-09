# savers

- The four legacy screensavers, rebuilt as vanilla ESM over one canvas: no React, no build step, no dependency.
- `saver(canvas, name, opts)` mounts one and returns `stop()`; `SAVERS` is `['matrix', 'sleep', 'mandelbrot', 'julia']`.
- `opts.seed` makes a saver deterministic through a tiny xorshift; no seed and it draws from `Math.random`.
- The seed reaches the choices, never the clock: the fractals pick their target, spin and rotation from `rand`, but zoom, fade and colour ride `performance.now()`, so two seeded mounts agree on what they draw and not on when.
- Every colour is read from the custom properties on the canvas: `--art` (with `--ground` behind it) is the ground, `--accent` is the primary ink of every saver. No literal ever, no random hue.
- `--accent` is the Tint setting, so switching tint recolours a running saver: `chrome.js` `tint()` fires the window `theme` event and `frame.js` re-reads the palette on it.
- `frame.js` owns the life: it sizes the canvas to its CSS box times `min(devicePixelRatio, 2)` under a ResizeObserver, pauses under an IntersectionObserver and on `document.hidden`, re-reads the palette on the window `theme` event and on a `prefers-color-scheme` change, and under `prefers-reduced-motion` draws one still frame and stops.
- A saver mounted offscreen never starts: with an IntersectionObserver present it waits for the first callback, so nothing runs until the canvas is seen, and pausing clears the rAF and the `every` interval both, never merely skipping draws.
- That still is `view.still`, handed to the maker so a saver whose motion is a fade can draw its one frame fully lit instead of at age zero.
- A canvas keeps the first context it is given, so a caller switching between a 2d saver and a WebGL one must mount on a fresh canvas element.
- A fractal that gets a context but no shader program still hands back a `stop()` that calls `WEBGL_lose_context`, so a failed mount never strands one of the browser's few WebGL contexts.
- The caller gates: `saver()` returns a no-op stopper when there is no canvas or no such name, and nothing runs without JS, so a page must ship a static fallback (a still image or a plain ground) inside or behind the canvas.

## SAVERS

- `matrix`: MrlyFont rain in `--accent`, one glyph of `0-9A-Z` per 20px column per 33ms tick, trails erased by the ground at 5% alpha.
- `sleep`: the DVD bouncer, one mrlytile of `min(w, h) * 0.15` at 4px a frame over a transparent ground, filled cells in `--accent`, re-rolling its design, its number and its level on every wall.
- `mandelbrot`: the set under `{-2, 1, -1.5, 1.5}`, zooming a whole cycle into a boundary point the wayfinder found.
- `julia`: the same engine over `{-1.5, 1.5, -1.5, 1.5}` with one of six c presets picked at mount.

## CONSTANTS

- Matrix: cell 20, tick 33ms, alphabet `0-9A-Z`, reset when `y > h` and the roll clears 0.975, `--accent` for the whole field.
- Sleep: side `0.15` of the smaller axis, velocity 4px a frame with random signs, marks are the five mrlytiles over numbers `3 5 7 9` at level 1 or 2, drawn as snapped rects so it stays crisp.
- Mrlytiles: `carpet` is on where at most one coordinate is odd, `net` where at least one is, `htree` on the even rows, `vtree` on the even columns, `void` where the two share a parity; level 2 is the level-1 grid Kroneckered with itself, `n^2` a side.
- Fractal: cycle 65536ms, fade 2048ms, zoom `pow(128, t)`, rotation `1/2048` a frame in a random direction, colour `1/256` a frame, start scale `1/2`, escape radius 128, `maxIter = 100 + floor(50 * log2(zoom))` under a hard cap of 1000.
- Every cycle re-rolls the target, the spin direction and the starting rotation; `stop()` clears the inline opacity the fade wrote.
- Fractal colour: inside is the ground, outside is `mix(ground, accent, 0.5 + 0.5 * cos(3.0 + sl * 0.15 + t))` over the smooth count `sl`, the accent being `--accent`.
- Wayfinder: 200 random points at 150 iterations a cycle, keeping the highest escape count that still escapes, which is the boundary.
- Julia presets: `-0.4+0.6i`, `-0.8+0.156i`, `0.285+0.01i`, `-0.7269+0.1889i`, `-0.1+0.651i`, `0.355+0.355i`.

## MOUNT

- Full screen, click anywhere to wake, the way the legacy did it:

```html
<canvas id="saver" style="position: fixed; inset: 0; width: 100vw; height: 100vh; background: var(--art)"></canvas>
<script type="module">
  import { saver } from '/kit/savers/index.js';
  const canvas = document.getElementById('saver');
  const stop = saver(canvas, 'matrix');
  addEventListener('click', () => (stop(), canvas.remove()), { once: true });
</script>
```

## FILES

- `index.js`: `saver()` and `SAVERS`, the only door.
- `frame.js`: the xorshift, the palette read, the colour parse and `run()`, the shared life every saver hangs from.
- `matrix.js`: the rain, and the pure `fell(y, h, roll)` reset rule.
- `sleep.js`: the bouncer and the pure `bounce(p, v, max)` wall.
- `tiles.js`: mrlytiles, the five 2D design rules ported from `crates/mrlymath` `two::designs`, plus `seed(design, n)`, `kron(base, level)` and `tile(design, n, level)` giving `{ size, cells }` over a `Uint8Array`.
- `fractal.js`: both fractals, the pure `fit`, `autoMaxIter`, `escaper` and `wayfind`.
- `gl.js`: the WebGL2 program: one vertex shader, one fragment shader branching on `u_julia`, the quad and the uniform table; `build()` checks every compile and the link and returns `null` on failure, and the fractal then draws nothing.
- `savers.test.js`: the pure rules, run with `bun test ui/savers/savers.test.js`.
