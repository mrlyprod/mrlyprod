import { useMemo } from 'react';
import { ready, ink, rgb, fit } from '../../../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Check, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { useQuery } from '../../../lib/query.js';

const m = await ready();

const CAPS = JSON.parse(m.apollonian_caps());
const ROOTS = CAPS.roots.map((name) => [name, name === 'strip' ? 'the strip (0, 0, 2, 2)' : `bounded (${name.split(',').join(', ')})`]);
const INKS = [['size', 'by curvature'], ['ford', 'the Ford circles apart'], ['one', 'one ink']];
const LOW = 5;
const HIGH = Math.round(Math.log2(CAPS.curvature));
const PAD = 16;
const BOX = 620;
const BAND = 160;

function App() {
  const [q, setQ] = useQuery({ root: 'strip', power: 11, order: 32, stack: true, fill: true, ink: 'size' });
  const cap = 2 ** Math.min(HIGH, Math.max(LOW, q.power));

  const view = useMemo(() => {
    const clock = performance.now();
    try {
      const read = JSON.parse(m.apollonian_read(q.root, cap, q.order));
      return {
        read,
        circles: m.apollonian(q.root, cap),
        roots: m.apollonian_root(q.root),
        marks: m.apollonian_touches(q.root, cap),
        nodes: read.strip ? JSON.parse(m.farey(q.order)) : [],
        ms: performance.now() - clock,
      };
    } catch (fault) {
      return { fault };
    }
  }, [q.root, cap, q.order]);

  const read = view.read;
  const shadow = read?.shadow;
  const stacked = q.stack && read?.strip;

  const draw = (canvas) => {
    if (!read) return;
    const [x0, y0, x1, y1] = read.frame;
    const wide = canvas.clientWidth;
    const scale = Math.min(wide - 2 * PAD, BOX) / Math.max(x1 - x0, y1 - y0);
    const deep = Math.round(scale * (y1 - y0)) + 2 * PAD;
    const band = stacked ? BAND : 0;
    const [ctx, w] = fit(canvas, deep + band);
    ctx.clearRect(0, 0, w, deep + band);
    const ox = (w - scale * (x1 - x0)) / 2 - x0 * scale;
    const X = (x) => ox + x * scale;
    const line = deep - PAD;
    const Y = (y) => line - (y - y0) * scale;
    const six = [ink.blue, ink.teal, ink.green, ink.yellow, ink.orange, ink.pink];
    const colour = (k, den) => {
      if (q.ink === 'one') return ink.fg;
      if (q.ink === 'ford') return den > 0 ? ink.blue : ink.dim;
      return six[Math.floor(Math.log2(Math.abs(k))) % 6];
    };

    ctx.save();
    ctx.beginPath();
    ctx.rect(X(x0), Y(y1), scale * (x1 - x0), scale * (y1 - y0));
    ctx.clip();
    if (read.strip) {
      ctx.strokeStyle = ink.dim;
      ctx.lineWidth = 1.5;
      for (const edge of [y0, y1]) {
        ctx.beginPath();
        ctx.moveTo(X(x0), Y(edge));
        ctx.lineTo(X(x1), Y(edge));
        ctx.stroke();
      }
    }
    const paint = (data, seed) => {
      for (let i = 0; i < data.length; i += 5) {
        const [cx, cy, r, k, den] = [data[i], data[i + 1], data[i + 2], data[i + 3], data[i + 4]];
        const tone = seed ? ink.fg : colour(k, den);
        const px = r * scale;
        ctx.beginPath();
        ctx.arc(X(cx), Y(cy), px, 0, Math.PI * 2);
        if (q.fill) {
          ctx.globalAlpha = 0.15;
          ctx.fillStyle = tone;
          ctx.fill();
          ctx.globalAlpha = 1;
        }
        ctx.strokeStyle = tone;
        ctx.lineWidth = Math.min(1.3, Math.max(0.4, px / 12));
        ctx.stroke();
      }
    };
    paint(view.roots, true);
    paint(view.circles, false);
    ctx.restore();

    if (read.strip) {
      ctx.strokeStyle = ink.blue;
      ctx.lineWidth = 1;
      for (let i = 0; i < view.marks.length; i += 4) {
        const at = X(view.marks[i]);
        ctx.beginPath();
        ctx.moveTo(at, line - 4);
        ctx.lineTo(at, line + 4);
        ctx.stroke();
      }
    }

    if (!stacked) return;
    const pale = rgb(ink.fg).join(', ');
    const tall = band - 34;
    for (const [num, den, bright] of view.nodes) {
      const at = X(num / den);
      ctx.strokeStyle = `rgba(${pale}, ${0.14 + 0.7 * bright / q.order})`;
      ctx.lineWidth = bright > q.order / 3 ? 1.5 : 0.7;
      ctx.beginPath();
      ctx.moveTo(at, line + 8);
      ctx.lineTo(at, line + 8 + tall * bright / q.order);
      ctx.stroke();
    }
    ctx.fillStyle = ink.dim;
    ctx.font = '11px ui-monospace, monospace';
    ctx.fillText('0', X(0) - 3, line + band - 8);
    ctx.textAlign = 'right';
    ctx.fillText('1', X(1) + 3, line + band - 8);
    ctx.textAlign = 'left';
  };

  const controls = (
    <>
      <section>
        <h3>The packing</h3>
        <Row>
          <Pick label="root quadruple" value={q.root} options={ROOTS} onChange={(v) => setQ({ root: v })} />
          <Slider label="curvature cap T" value={q.power} min={LOW} max={HIGH} show={cap} onChange={(v) => setQ({ power: v })} />
          <Pick label="ink" value={q.ink} options={INKS} onChange={(v) => setQ({ ink: v })} />
          <Check label="fill the discs" checked={q.fill} onChange={(v) => setQ({ fill: v })} />
        </Row>
      </section>
      <section>
        <h3>The stack</h3>
        <Row>
          <Check label="lay the Farey stack under the line" checked={q.stack} onChange={(v) => setQ({ stack: v })} />
          <Slider label="depth Q" value={q.order} min={2} max={CAPS.order} onChange={(v) => setQ({ order: v })} />
        </Row>
      </section>
    </>
  );

  const census = read && `root (${read.curvatures.join(', ')})   N(T) = ${read.circles} circles to curvature T = ${read.cap}, the root quadruple excluded   ${read.quads} quadruples, ${read.broken} broken, ${read.strayed} strayed`;
  const exponent = read?.exponent !== null && read?.exponent !== undefined
    ? `local exponent log(N(${read.cap})/N(${read.from}))/log 4 = ${read.exponent.toFixed(4)}, read on that one octave pair and nowhere finer`
    : '';
  const identified = read && (read.strip
    ? `line-tangent circles ${read.line}, of them Ford circles ${read.ford}, off-Ford ${read.line - read.ford}`
    : 'no line in this root, so no Ford circles and no stack to shadow');
  const agreed = read && read.strip && shadow && (shadow.covered
    ? `the stack at Q = ${shadow.order}: nodes lit in the open period ${shadow.nodes}, tangency points carrying them ${shadow.touched}, missed ${shadow.missed}, off-Ford below ${shadow.reach} ${shadow.offford}   brightness ${shadow.bright} against Q(Q + 1)/2 = ${shadow.want}`
    : `the packing stops at ${read.cap} and the stack at Q = ${shadow.order} needs curvature 2 Q^2 = ${shadow.reach}: raise T before reading the agreement`);

  return (
    <Page crumb="apollonian" title="The Apollonian gasket"
      sub="Four mutually tangent circles obey Descartes, and the second circle in a curvilinear triangle is the first reflected, k' = 2(k1 + k2 + k3) - k4, with no square root in it. Write a circle as the integer triple (k, kx, ky) and that reflection moves all three coordinates at once, so an integer root quadruple grows a whole packing in exact integers. The strip root is two lines a unit apart: the circles it grows that rest on the lower line are exactly the Ford circles, the circle over a reduced a/b carrying curvature 2 b squared and resting at a/b. Lay the Farey stack under that line and the bars stand at the tangency points, one bar to a circle, lit floor(Q/b) times."
      controls={controls}
      foot={<>The growth, the six exact invariants it checks on every quadruple, the census, the Ford test, the tangency fractions and the agreement with the stack are computed in Rust; the page strokes the circles it is handed at the centres and radii the crate gives. The stack is the one <a href="../farey">the Farey demo</a> builds, and its bars are placed from the same numerator and denominator the tangency points carry, so a bar and its tick land on one pixel. The census exponent is printed as a ratio of two counts over one octave pair and never fitted; the dimension it is reaching for, and why no design of any base has it, are in <a href="/research/apollonian/">the Apollonian note</a>. The desk's other gasket is the Sierpinski one and shares nothing with this but the name.</>}>
      <Sketch draw={draw} deps={[view, q.ink, q.fill, stacked, q.order]} role="img" aria-label="An integral Apollonian packing drawn over the Farey stack of its tangency points" />
      <Stats>
        <Stat label="circles drawn">{read?.drawn}</Stat>
        <Stat label="census N(T)">{read?.circles}</Stat>
        <Stat label="Ford circles">{read && (read.strip ? read.ford : '-')}</Stat>
        <Stat label="nodes lit">{read && (read.strip ? shadow.nodes : '-')}</Stat>
        <Stat label="brightness">{read && (read.strip ? shadow.bright : '-')}</Stat>
        <Stat label="broken">{read?.broken}</Stat>
        <Stat label="draw">{view.ms !== undefined && `${view.ms.toFixed(0)} ms`}</Stat>
      </Stats>
      {read && <pre>{[census, identified, agreed, exponent].filter(Boolean).join('\n')}</pre>}
      <Note error={view.fault} />
    </Page>
  );
}

mount(<App />);
