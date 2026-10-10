import { useMemo } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { board, axis } from '../../../lib/chart.js';
import { demo, Page, Group, Pick, Slider, Text, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { useQuery } from '../../../lib/query.js';

const m = await ready();

const PRESETS = [
  ['0,1|0', 'Cantor'],
  ['0,1,2|0', 'smallest loose'],
  ['0,1,2,4|0', 'the figure'],
  ['0,1,4,6|0', 'minimum redundancy'],
  ['0,1,2,3|0', 'one bound tight'],
  ['0,1,4,5,7|0', 'five, loose'],
  ['0,1,3,5,8,9|0', 'six, loose'],
  ['0,1,2|4', 'three at base 4'],
  ['0,1,4,6|8', 'ruler at base 8'],
  ['0,1,2,3,7|13', 'five at base 13'],
  ['0,1,4,7,9|12', 'five at base 12'],
  ['0,1,2,3,7,11|15', 'six at base 15'],
];
const FIRST = { g: '0,1,2,4', base: 0, level: 2, out: 7 };
const SHOWN = 6;
const APART = { marginTop: 8 };

const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, Math.round(v)));
const set = (list) => `{${list.join(', ')}}`;
const ratio = (a, b) => `${a}/${b}`;

const attempt = (fn) => {
  try {
    return { value: fn(), error: null };
  } catch (error) {
    return { value: null, error };
  }
};

function nearest(sensors, p) {
  let lo = 0, hi = sensors.length - 1;
  while (hi - lo > 1) {
    const mid = (lo + hi) >> 1;
    if (sensors[mid] <= p) lo = mid;
    else hi = mid;
  }
  return p - sensors[lo] <= sensors[hi] - p ? lo : hi;
}

function legend(b, items) {
  const { ctx } = b;
  let x = b.x(1);
  ctx.textAlign = 'right';
  for (const [text, color] of [...items].reverse()) {
    ctx.fillStyle = color;
    ctx.fillText(text, x, 14);
    x -= ctx.measureText(text).width + 14;
  }
  ctx.textAlign = 'left';
}

function App() {
  const [q, put] = useQuery(FIRST);
  const head = useMemo(() => attempt(() => JSON.parse(m.arrays_generator(q.g))), [q.g]);
  const g = head.value;
  const base = g ? (q.base ? clamp(q.base, g.span + 1, g.full) : g.full) : 0;
  const cap = useMemo(() => (g ? attempt(() => m.arrays_cap(q.g, base)).value ?? 1 : 1), [q.g, base, g]);
  const level = clamp(q.level, 1, cap);

  const view = useMemo(() => (g ? attempt(() => JSON.parse(m.arrays_read(q.g, base, level))) : head), [q.g, base, level, g, head]);
  const read = view.value;
  const total = read?.total ?? 0;
  const out = q.out >= 0 && q.out < total ? q.out : -1;
  const weights = useMemo(() => (read ? attempt(() => m.arrays_weights(q.g, base, level, out < 0 ? total : out)).value : null), [read, out]);
  const knock = useMemo(() => (read && out >= 0 ? attempt(() => JSON.parse(m.arrays_knock(q.g, base, level, out))).value : null), [read, out]);

  const preset = `${q.g}|${q.base}`;
  const known = PRESETS.some(([v]) => v === preset);
  const choose = (v) => {
    if (v === 'custom') return;
    const [code, at] = v.split('|');
    put({ g: code, base: +at, out: 1 });
  };

  const strip = (canvas) => {
    const b = board(canvas, 96, { top: 26, bottom: 22 });
    if (!read) return;
    const { ctx } = b;
    const reach = read.reach;
    const unit = b.wide / reach;
    const mid = (b.roof + b.floor) / 2;
    const dots = unit >= 5;
    const radius = Math.min(7, unit * 0.42);
    const thin = Math.max(1, Math.min(3, unit * 0.7));
    const paths = { on: new Path2D(), off: new Path2D() };
    read.sensors.forEach((p, i) => {
      if (i === out) return;
      const path = read.essential[i] ? paths.on : paths.off;
      const x = b.x(p / reach);
      if (dots) {
        path.moveTo(x + radius, mid);
        path.arc(x, mid, radius, 0, Math.PI * 2);
      } else {
        path.rect(x - thin / 2, b.roof, thin, b.tall);
      }
    });
    axis(b, [[0, '0'], [1, String(reach)]]);
    ctx.fillStyle = ink.blue;
    ctx.fill(paths.off);
    ctx.fillStyle = ink.orange;
    ctx.fill(paths.on);
    if (out >= 0) {
      const x = b.x(read.sensors[out] / reach);
      ctx.strokeStyle = ink.pink;
      ctx.lineWidth = 2;
      ctx.beginPath();
      if (dots) ctx.arc(x, mid, radius, 0, Math.PI * 2);
      else {
        ctx.moveTo(x, b.roof - 4);
        ctx.lineTo(x, b.floor);
      }
      ctx.stroke();
    }
    ctx.fillStyle = ink.dim;
    ctx.fillText(`${total} sensors`, b.x(0), 14);
    legend(b, [['essential', ink.orange], ['inessential', ink.blue], ...(out >= 0 ? [['knocked out', ink.pink]] : [])]);
  };

  const coarray = (canvas) => {
    const b = board(canvas, 200, { top: 26, bottom: 22 });
    if (!read || !weights) return;
    const { ctx } = b;
    const n = weights.length;
    const reach = read.reach;
    const peak = Math.log1p(total);
    const cols = Math.max(1, Math.min(n, Math.floor(b.wide)));
    const per = n / cols;
    const step = b.wide / cols;
    const gap = step >= 5 ? 1 : 0;
    const bars = new Path2D();
    for (let c = 0; c < cols; c++) {
      const lo = Math.floor(c * per);
      const hi = Math.max(lo + 1, Math.floor((c + 1) * per));
      let top = 0;
      for (let i = lo; i < hi; i++) top = Math.max(top, weights[i]);
      if (!top) continue;
      const h = Math.max(1, (b.tall * Math.log1p(top)) / peak);
      bars.rect(b.x(c / cols) + gap / 2, b.floor - h, Math.max(0.5, step - gap), h);
    }
    ctx.fillStyle = ink.blue;
    ctx.fill(bars);
    const opened = new Set(knock?.lost ?? []);
    const wide = Math.max(2, (b.wide / n) - gap);
    for (let i = 0; i < n; i++) {
      if (weights[i]) continue;
      const x = b.x((i + 0.5) / n);
      ctx.fillStyle = opened.has(i - reach) ? ink.pink : ink.yellow;
      ctx.globalAlpha = 0.3;
      ctx.fillRect(x - wide / 2, b.roof, wide, b.tall);
      ctx.globalAlpha = 1;
      ctx.fillRect(x - wide / 2, b.floor - 6, wide, 6);
    }
    axis(b, [[0, String(-reach)], [0.5, '0'], [1, String(reach)]]);
    ctx.fillStyle = ink.dim;
    ctx.fillText(`${n} lags`, b.x(0), 14);
    legend(b, [['log weight', ink.blue], ['hole opened', ink.pink], ...(read.holes ? [['hole', ink.yellow]] : [])]);
  };

  const seek = (f) => {
    if (!read) return;
    put({ out: nearest(read.sensors, Math.max(0, Math.min(1, f)) * read.reach) });
  };

  const full = read && read.base === read.full;
  const law = read?.law;
  const u = read?.paired.length;
  const lost = (knock?.lost ?? []).filter((t) => t > 0);

  const controls = (
    <>
      <Group name="The generator">
        <Pick label="preset" value={known ? preset : 'custom'} onChange={choose} options={[...PRESETS, ...(known ? [] : [['custom', 'your own']])]} />
        <Text label="sensors" value={q.g} onChange={(v) => put({ g: v, base: 0 })} />
      </Group>
      {g && (
        <Group name="The array">
          <Slider label="base" value={base} min={g.span + 1} max={g.full} onChange={(v) => put({ base: v === g.full ? 0 : v })} />
          <Slider label="level" value={level} min={1} max={cap} onChange={(v) => put({ level: v })} />
          <Slider label="knock out" value={out} min={-1} max={Math.max(0, total - 1)} onChange={(v) => put({ out: v })} show={out < 0 ? 'none' : read?.sensors[out]} />
        </Group>
      )}
    </>
  );

  return (
    <Page title="Sparse arrays" controls={controls}
      sub="A sparse array is a set of sensor positions, and its difference coarray is every gap between two of them, counted by how many pairs make it. Write each sensor as a number whose digits come from a small generator, in a base, and the coarray has no holes. Click a sensor to knock it out: a hole opens exactly when the sensor carried some gap alone. From the second level on at base 2a + 1, Theorem A says which sensors those are, every digit paired, so u^r of the L^r sensors are essential, and the page counts them."
      foot={<>Every number on this page comes from the crate through wasm: the array, the weights, the essential test by pair counts, the lags a loss deletes and the law; the page only draws. A digit is paired when some other digit of the generator meets it at a gap only that pair makes; u counts them. Below base 2a + 1 the digits carry and the count can leave the product law. Nearby: <a href="../sumset/">three plus four</a> carries the same product over digits.</>}>
      <Sketch draw={strip} deps={[read, out]} onSeek={seek} role="img" aria-label={read ? `the array of ${total} sensors, ${read.count} essential` : 'the array'} />
      <Sketch draw={coarray} deps={[read, weights, knock]} style={APART} role="img" aria-label="the difference coarray and its weights" />
      <Note error={view.error} />
      {read && (
        <Stats>
          <Stat label="generator">{set(read.generator)}</Stat>
          <Stat label="L">{read.size}</Stat>
          <Stat label="a">{read.span}</Stat>
          <Stat label="base">{full ? `${read.base} = 2a + 1` : `${read.base} of ${read.full}`}</Stat>
          <Stat label="r">{read.level}</Stat>
          <Stat label="lags">{2 * read.reach + 1}</Stat>
          <Stat label="holes">{read.holes}</Stat>
          {read.holes === 0 && <span className="chip proved">hole-free</span>}
        </Stats>
      )}
      {read && (
        <Stats>
          <Stat label="U(G)">{set(read.paired)}</Stat>
          <Stat label="E(G)">{set(read.kernel)}</Stat>
          <Stat label="essential, counted">{`${read.count} of ${read.total}`}</Stat>
          <Stat label="fragility">{read.fragility.toFixed(4)}</Stat>
        </Stats>
      )}
      {read && (
        <Stats>
          {law ? (
            <>
              <Stat label="Theorem A, (u/L)^r">{`(${u}/${read.size})^${read.level} = ${ratio(law.count, law.total)}`}</Stat>
              <span className="chip proved">{law.count === read.count ? 'the law meets the count' : 'the law misses the count'}</span>
            </>
          ) : (
            <Stat label="Theorem A">{read.seed ? 'needs a hole-free generator' : !full ? 'holds at base 2a + 1 only' : 'holds from r = 2 on'}</Stat>
          )}
          {full && <Stat label="bound E/L">{ratio(...read.bounds.cohen)}</Stat>}
          {full && <Stat label="bound (E/L)^r">{ratio(...read.bounds.yang)}</Stat>}
        </Stats>
      )}
      {read && (
        <Stats>
          {knock ? (
            <>
              <Stat label="knocked out">{knock.sensor}</Stat>
              <Stat label="digits, lowest first">{knock.digits.join(', ')}</Stat>
              <Stat label="all in U(G)">{knock.paired ? 'yes' : 'no'}</Stat>
              <Stat label="lags lost">{lost.length ? `${2 * lost.length}: ${lost.slice(0, SHOWN).map((t) => `+-${t}`).join(', ')}${lost.length > SHOWN ? ', ...' : ''}` : 'none'}</Stat>
              <span className="chip proved">{knock.essential ? 'a hole opens' : 'no hole opens'}</span>
            </>
          ) : (
            <Stat label="knocked out">none: click a sensor</Stat>
          )}
        </Stats>
      )}
    </Page>
  );
}

export const { mount, unmount } = demo(<App />);
