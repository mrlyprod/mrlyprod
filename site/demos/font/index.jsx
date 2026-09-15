import { useEffect, useRef, useState } from 'react';
import { ready, paint } from '../../lib/mrly.js';
import { mount, Page, Row, Slider, Check, Stats, Stat } from '../../lib/app.jsx';

const m = await ready();
const FPS = 25;
const SECTIONS = [['Uppers', 26], ['Lowers', 26], ['Digits', 10], ['Extras', 42], ['Specials', 4]];

function study(char) {
  const { rows, cols, frames } = JSON.parse(m.font_animate(char, 1));
  const cells = [];
  let prev = new Set();
  for (const frame of frames.slice(1)) {
    cells.push(frame.find((i) => !prev.has(i)));
    prev = new Set(frame);
  }
  const at = (i) => [Math.floor(i / cols), i % cols];
  let lifts = 0;
  for (let i = 1; i < cells.length; i++) {
    const [ar, ac] = at(cells[i - 1]);
    const [br, bc] = at(cells[i]);
    if (Math.abs(ar - br) + Math.abs(ac - bc) !== 1) lifts++;
  }
  const strokes = cells.length ? lifts + 1 : 0;
  const floor = m.font_floor(char);
  const chip = strokes > floor ? 'over floor' : lifts >= 5 ? `${lifts} lifts` : null;
  return { char, rows, cols, frames, strokes, floor, lifts, chip };
}

const GLYPHS = [...m.font_chars()].map(study);
const GROUPS = SECTIONS.reduce((out, [name, count]) => {
  const from = out.reduce((n, g) => n + g.glyphs.length, 0);
  out.push({ name, glyphs: GLYPHS.slice(from, from + count) });
  return out;
}, []);
const CONTENTS = GROUPS.map((g) => ({ id: g.name.toLowerCase(), text: g.name, level: 2 }));

function frameOf(glyph, tick, hold) {
  const n = glyph.frames.length;
  const i = tick % (n + hold);
  return glyph.frames[Math.min(i, n - 1)];
}

function draw(canvas, glyph, frame) {
  const types = new Uint8Array(glyph.rows * glyph.cols);
  for (const i of frame) types[i] = 1;
  paint(canvas, { width: glyph.cols, height: glyph.rows, types });
}

function Glyph({ glyph, canvases }) {
  const label = glyph.char === ' ' ? 'space' : glyph.char;
  return (
    <div className={glyph.chip ? 'card on' : 'card'}>
      <canvas ref={(node) => { canvases.current.set(glyph.char, node); }} role="img" aria-label={`${label} written stroke by stroke`} />
      <p><b>{label}</b> <span>{glyph.strokes} of {glyph.floor}</span>{glyph.chip && <> <span className="chip refuted">{glyph.chip}</span></>}</p>
    </div>
  );
}

function App() {
  const [hold, setHold] = useState(FPS);
  const [slow, setSlow] = useState(false);
  const canvases = useRef(new Map());
  const tick = useRef(0);
  useEffect(() => {
    const step = () => {
      for (const glyph of GLYPHS) {
        const canvas = canvases.current.get(glyph.char);
        if (canvas) draw(canvas, glyph, frameOf(glyph, tick.current, hold));
      }
      tick.current++;
    };
    step();
    const timer = setInterval(step, slow ? 2000 / FPS : 1000 / FPS);
    return () => clearInterval(timer);
  }, [hold, slow]);
  const flagged = GLYPHS.filter((g) => g.chip);

  const controls = (
    <Row>
      <Slider label="hold" value={hold} min={0} max={2 * FPS} onChange={setHold} />
      <Check label="half speed" checked={slow} onChange={setSlow} />
    </Row>
  );

  return (
    <Page crumb="font" title="The 108 pens"
      sub="Every glyph of MrlyFont writes itself in the order its pen table gives, one cell a frame at 25 a second, then holds. Under each one: its stroke count against its floor, the least strokes that can write it. A chip marks a glyph penned over its floor or lifting the pen five times or more."
      controls={controls} contents={CONTENTS}
      foot={<>The frames come from the wasm bridge, so this page shows the crate's <code>pens.rs</code> as it is now. A stroke walks 4-adjacent cells; a lift is any step that is not. The floor is the crate's exact minimum cover of the glyph's cells by 4-adjacent paths, so a glyph over its floor is penned that way on purpose, like the wordmark letters, and a glyph on its floor with many lifts can only be helped by a different shape.</>}>
      <Stats>
        <Stat label="glyphs">{GLYPHS.length}</Stat>
        <Stat label="strokes">{GLYPHS.reduce((n, g) => n + g.strokes, 0)}</Stat>
        <Stat label="flagged">{flagged.map((g) => g.char).join(' ') || 'none'}</Stat>
      </Stats>
      {GROUPS.map((group) => (
        <section key={group.name} id={group.name.toLowerCase()} aria-label={group.name}>
          <h2 className="group">{group.name}</h2>
          <div className="cards pens">
            {group.glyphs.map((glyph) => <Glyph key={glyph.char} glyph={glyph} canvases={canvases} />)}
          </div>
        </section>
      ))}
    </Page>
  );
}

mount(<App />);
