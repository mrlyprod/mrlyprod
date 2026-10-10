import { useMemo } from 'react';
import { ready, ink, fit } from '../../../lib/mrly.js';
import { demo, Page, Group, Slider, Check, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { useQuery } from '../../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../../lib/select.jsx';

const m = await ready();

const FIRST = { 2: '7', 3: '495', 4: '57343', 5: '33550335' };
const TALL = 640;
const PAD = 12;
const QUARTER = Math.PI / 2;

function sheet(code, base, level) {
  const grid = m.arcs_grid(code, base, level);
  const out = { side: grid.width, types: grid.types };
  grid.free();
  return out;
}

function arcs({ side: n, types }, x0, y0, cell) {
  const r = cell / 2;
  const loops = new Path2D();
  const strands = new Path2D();
  const fills = new Path2D();
  const sweep = (path, cx, cy, from) => {
    path.moveTo(cx + r * Math.cos(from), cy + r * Math.sin(from));
    path.arc(cx, cy, r, from, from + QUARTER);
  };
  for (let y = 0; y < n; y++) {
    const top = y0 + (n - 1 - y) * cell;
    const bottom = top + cell;
    for (let x = 0; x < n; x++) {
      const byte = types[y * n + x];
      const left = x0 + x * cell;
      const right = left + cell;
      const lower = byte & 2 ? loops : strands;
      const upper = byte & 4 ? loops : strands;
      if (byte & 1) {
        fills.rect(left, top, cell, cell);
        sweep(lower, left, bottom, -QUARTER);
        sweep(upper, right, top, QUARTER);
      } else {
        sweep(lower, right, bottom, Math.PI);
        sweep(upper, left, top, 0);
      }
    }
  }
  return { loops, strands, fills };
}

function App() {
  const s = useSeeds();
  const [q, set] = useQuery({ code: seeded(s, 2, 2, '7'), base: 2, level: 5, cells: true });

  const cap = m.arcs_cap(q.base);
  const level = Math.max(1, Math.min(q.level, cap));
  const code = q.code.trim();

  const view = useMemo(() => {
    try {
      return {
        name: m.name_of(code, 2, q.base),
        read: JSON.parse(m.arcs_read(code, q.base, level)),
        grid: sheet(code, q.base, level),
      };
    } catch (error) {
      return { error };
    }
  }, [code, q.base, level]);

  const choose = (patch) => {
    if (patch.base !== undefined && patch.code === undefined) {
      s.drop();
      set({ base: patch.base, code: FIRST[patch.base] });
      return;
    }
    set(patch);
  };

  const draw = (canvas) => {
    const [ctx, w, h] = fit(canvas, Math.min(canvas.clientWidth, TALL));
    ctx.clearRect(0, 0, w, h);
    if (!view.grid) return;
    const span = Math.min(w, h) - 2 * PAD;
    const cell = span / view.grid.side;
    const paths = arcs(view.grid, (w - span) / 2, (h - span) / 2, cell);
    if (q.cells) {
      ctx.fillStyle = ink.line;
      ctx.fill(paths.fills);
    }
    ctx.lineWidth = Math.max(0.6, Math.min(3, cell * 0.16));
    ctx.strokeStyle = ink.blue;
    ctx.stroke(paths.strands);
    ctx.strokeStyle = ink.orange;
    ctx.stroke(paths.loops);
  };

  const read = view.read;
  const law = read?.law;

  const controls = (
    <Group name="The design">
      <Picker dimension={2} bases={[2, 3, 4, 5]} code={q.code} base={q.base} seeds={s} onChange={choose} />
      <Slider label="level" value={level} min={1} max={cap} onChange={(v) => set({ level: v })} />
      <Check label="shade the filled cells" checked={q.cells} onChange={(v) => set({ cells: v })} />
    </Group>
  );

  return (
    <Page title="Loops in arcs" controls={controls}
      sub="Every cell of a design carries two quarter arcs, each joining the midpoints of two neighbouring edges: a filled cell takes the arcs around its lower-left and upper-right corners, an empty cell the other two. The arcs join into curves. The strands, in blue, run from border to border, and there are always twice the side of them. The loops, in orange, close on themselves, and how many there are is up to the design."
      foot={<>Every count on this page comes from the crate through wasm, by union-find over the edge midpoints; the page only draws. Row 0 is at the bottom, so code 7 at base 2 leaves its upper-right quarter empty. A law shows when one is proved: the carpet, base 3 code 495, and all sixteen codes at base 2. Nearby: <a href="../universe/">the universe</a> lists the designs, <a href="../tile/">the tile</a> repeats one.</>}>
      <Sketch draw={draw} deps={[view, q.cells]} role="img" aria-label={`${view.name ?? 'a design'} at level ${level} drawn in arcs`} />
      <Note error={view.error} />
      {read && (
        <Stats>
          <Stat label="name">{view.name}</Stat>
          <Stat label="side">{read.side}</Stat>
          <Stat label="filled">{`${read.filled} of ${read.cells}`}</Stat>
          <Stat label="loops">{read.loops}</Stat>
          <Stat label="strands">{read.strands}</Stat>
          <span className="chip proved">strands = 2 side</span>
        </Stats>
      )}
      {read && (
        <Stats>
          {law ? (
            <>
              <Stat label="law">{`L(n) = ${law.formula}`}</Stat>
              <Stat label={`L(${level})`}>{law.loops}</Stat>
              <span className="chip proved">{law.loops === read.loops ? 'the law meets the count' : 'the law misses the count'}</span>
            </>
          ) : (
            <Stat label="law">no closed form proved for this code</Stat>
          )}
        </Stats>
      )}
    </Page>
  );
}

export const { mount, unmount } = demo(<App />);
