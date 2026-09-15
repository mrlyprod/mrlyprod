import { useMemo, useRef } from 'react';
import { ready, ink, rgb } from '../../lib/mrly.js';
import { mount, Page, Pick, Slider, Check, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();
const SIZE = 768;
const PAD = 10;
const SIDES = [2, 3, 5, 7];
const GROWTHS = [['prime', 'MrlyUlam: the primes grow'], ['every', 'MrlySpiral: every number grows']];
const FIRST = { growth: 'every', code: '7', base: 2, side: 2, top: 300, path: true };

const first = (seeds) => ({ ...FIRST, code: seeded(seeds, 2, FIRST.base, FIRST.code) });

function inked(grid, color) {
  const [w, h] = [grid.width, grid.height];
  const [r, g, b] = rgb(color);
  const rgba = new Uint8ClampedArray(w * h * 4);
  for (let i = 0; i < w * h; i += 1) {
    if (!grid.types[i]) continue;
    rgba[i * 4] = r;
    rgba[i * 4 + 1] = g;
    rgba[i * 4 + 2] = b;
    rgba[i * 4 + 3] = 255;
  }
  const canvas = document.createElement('canvas');
  canvas.width = w;
  canvas.height = h;
  canvas.getContext('2d').putImageData(new ImageData(rgba, w, h), 0, 0);
  return canvas;
}

function App() {
  const s = useSeeds();
  const [look, save] = useQuery(first(s));
  const kept = useRef({ cells: null, read: null, art: null });

  const view = useMemo(() => {
    try {
      const cells = m.snail_cells(look.side, look.top, look.growth);
      const read = JSON.parse(m.snail_read(look.side, look.top, look.growth));
      const art = [];
      for (let k = 1; k <= read.peak; k += 1) {
        const grid = m.two_grid(look.code, look.side, k, 0, look.base);
        art[k] = { lit: inked(grid, ink.blue), plain: inked(grid, ink.dim) };
      }
      kept.current = { cells, read, art };
      return { ...kept.current, error: null };
    } catch (error) {
      return { ...kept.current, error };
    }
  }, [look.growth, look.side, look.top, look.code, look.base]);

  const draw = (canvas) => {
    const { cells, read, art } = view;
    if (!cells) return;
    canvas.width = SIZE;
    canvas.height = SIZE;
    const ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;
    ctx.fillStyle = ink.deep;
    ctx.fillRect(0, 0, SIZE, SIZE);
    const inner = SIZE - 2 * PAD;
    const scale = inner / Math.max(read.width, read.height);
    const at = (x, y) => [
      PAD + (inner - read.width * scale) / 2 + (x - read.low[0]) * scale,
      SIZE - PAD - (inner - read.height * scale) / 2 - (y - read.low[1]) * scale,
    ];
    if (look.path) {
      ctx.strokeStyle = ink.line;
      ctx.lineWidth = 1;
      ctx.beginPath();
      for (let i = 0; i < cells.length; i += 5) {
        const half = cells[i + 2] / 2;
        const [px, py] = at(cells[i] + half, cells[i + 1] + half);
        if (i) ctx.lineTo(px, py);
        else ctx.moveTo(px, py);
      }
      ctx.stroke();
    }
    for (let i = 0; i < cells.length; i += 5) {
      const side = cells[i + 2];
      const level = cells[i + 3];
      const prime = cells[i + 4];
      const wide = side * scale;
      const [px, py] = at(cells[i], cells[i + 1] + side);
      if (level && wide >= 3) {
        ctx.drawImage(prime ? art[level].lit : art[level].plain, px, py, wide, wide);
      } else {
        ctx.globalAlpha = prime ? 1 : 0.5;
        ctx.fillStyle = prime ? ink.blue : ink.dim;
        ctx.fillRect(px, py, Math.max(wide, 1), Math.max(wide, 1));
        ctx.globalAlpha = 1;
      }
    }
  };

  const read = view.read;
  const tally = read ? read.levels.map((count, k) => `${count} at ${look.side}^${k}`).join(', ') : '';

  const controls = (
    <>
      <Group name="The winding">
        <Pick label="growth" value={look.growth} options={GROWTHS} onChange={(v) => save({ growth: v })} />
        <Slider label="numbers" value={look.top} min={12} max={2000} onChange={(v) => save({ top: v })} />
        <Check label="the path" checked={look.path} onChange={(v) => save({ path: v })} />
      </Group>
      <Group name="The tile">
        <Picker dimension={2} bases={[2, 3]} code={look.code} base={look.base} seeds={s} onChange={save} />
        <Pick label="base q" value={look.side} options={SIDES.map((q) => [q, q])} onChange={(v) => save({ side: +v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="snail" title="The snail"
      sub="Wind 1, 2, 3 and on around the Ulam spiral, but let a cell that grows carry a whole design instead of a dot, and let it grow with the number. Each tile's side is a power of the base, so every time the count gains a digit the winding widens by that factor and the path curls outward like a shell. Keep only the primes growing and the shell is built of primes; let every number grow and the same shell appears with no prime named anywhere."
      foot={<>The rule, exactly. The level of <code>n</code> is the count of its base <code>q</code> digits less one, so <code>k(n) = 0</code> below <code>q</code>, <code>1</code> from <code>q</code> to <code>q² - 1</code>, and on; a tile at level <code>k</code> is the chosen design grown <code>k</code> times, of side <code>q^k</code>, and level 0 is the bare unit cell. MrlyUlam grows a cell only when <code>n</code> is prime and leaves one and every composite a unit cell; MrlySpiral grows every <code>n</code> by the same law and names no prime. The layout is one deterministic step: tile 1 has its lower-left corner at the origin, and the corner of tile <code>n + 1</code> is the corner of tile <code>n</code> plus the unit step the square winding takes from <code>n</code> to <code>n + 1</code>, scaled by the side of tile <code>n</code>. Tiles overlap wherever the growth outruns the winding, so the drawn area is the sum of the tile squares and counts an overlap twice. This page is an exhibit: it states no theorem, and the two rules above are ours, fixed here and nowhere else. The same numbers on the plain unit grid are the <a href="../ulam">ulam</a> page, and the sieve that decides which of them grow is on the <a href="../primes">primes</a> page.</>}
      controls={controls}>
      <div className="arena">
        <div className="panel">
          <h2>The shell <span>{read && `${read.tiles} numbers, ${read.grown} grown, base ${look.side}`}</span></h2>
          <Sketch draw={draw} deps={[view, look.path]} aria-label="The snail, the whole numbers wound on the square spiral with every grown cell a design tile" />
        </div>
      </div>
      <Stats>
        <Stat label="primes at or below the top">{read?.primes}</Stat>
        <Stat label="tiles grown past a unit cell">{read?.grown}</Stat>
        <Stat label="tiles per level">{tally}</Stat>
        <Stat label="largest tile">{read && `${read.side} by ${read.side}`}</Stat>
        <Stat label="drawn area">{read?.area}</Stat>
        <Stat label="the box">{read && `${read.width} by ${read.height}`}</Stat>
      </Stats>
      <Note error={view.error} />
    </Page>
  );
}

mount(<App />);
