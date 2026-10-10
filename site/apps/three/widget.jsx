import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { roll } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, VIEWS, cutting, staged, tag } from './engine.js';
import { make } from './scene.js';

const STEP = 0.05;

const ROWS = [
  ['Name', (f) => f.title],
  ['Level', (f) => f.level],
  ['Side', (f) => f.side],
  ['Fills', (f) => f.fills],
  ['Voids', (f) => f.voids],
  ['Faces', (f) => f.faces],
  ['Surface', (f) => f.surface],
  ['Euler', (f) => f.euler],
  ['Dimension', (f) => f.dimension.toFixed(3)],
  ['Filled in', (f) => f.crop?.in],
  ['Filled cut', (f) => f.crop?.cut],
  ['Filled out', (f) => f.crop?.out],
];

const CUTS = {
  slice: [['Fills', (c) => c.fills], ['Voids', (c) => c.voids], ['Euler', (c) => c.euler]],
  diagonal: [['Cells', (c) => c.cells], ['Support', (c) => c.support], ['Least', (c) => c.least], ['Most', (c) => c.most], ['Constant', (c) => c.constant]],
  hex: [['Triangles', (c) => c.triangles], ['Fills', (c) => c.fills], ['Voids', (c) => c.voids], ['Pieces', (c) => c.pieces], ['Holes', (c) => c.holes], ['Euler', (c) => c.euler]],
};

const label = (view) => VIEWS.find(([key]) => key === view)?.[1] ?? view;

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

const Facts = ({ rows: list, of }) => list.filter(([, get]) => get(of) !== undefined).map(([name, get]) => <Fact key={name} name={name} value={get(of)} />);

export function useUnit(given) {
  const [math, setMath] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setMath(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  return given ?? math;
}

export function useThree(wanted, given) {
  const [three, setThree] = useState(null);
  useEffect(() => {
    if (given || !wanted || three) return undefined;
    let on = true;
    import('three').then((mod) => on && setThree(mod));
    return () => {
      on = false;
    };
  }, [given, wanted, three]);
  return given ?? three;
}

export function Widget({ value, onChange, onReady, onExport, unit, three: given }) {
  const math = useUnit(unit);
  const shown = staged(value);
  const solid = shown.view === 'solid';
  const three = useThree(solid, given);
  const live = useRef(null);
  const now = useRef(value);
  const from = useRef(null);
  const [facts, setFacts] = useState(null);
  const [cut, setCut] = useState(null);
  now.current = value;
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    setCut(handle?.cut ?? null);
    const hands = {
      random: () => onChange({ code: roll(math, DIM, read(now.current).base) }),
      slide: (by) => {
        const { at } = cutting(now.current);
        if (at !== undefined) onChange({ at: at + by * STEP });
      },
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const down = (e) => {
    if (e.button) return;
    from.current = [e.clientX, e.clientY];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    if (!from.current) return;
    const rect = e.currentTarget.getBoundingClientRect();
    live.current?.turn?.((e.clientX - from.current[0]) / rect.width, (e.clientY - from.current[1]) / rect.height);
    from.current = [e.clientX, e.clientY];
  };
  const up = () => {
    from.current = null;
  };
  const keep = (body, kind) => body && text(body, `three-${now.current.seed}`, kind);
  const obj = () => keep(live.current?.obj?.(), 'obj');
  const svg = () => live.current?.svg?.().then((body) => keep(body, 'svg'));
  const have = solid ? Boolean(facts) : Boolean(cut);
  useEffect(() => onExport?.(have ? [solid ? ['OBJ', obj] : ['SVG', svg]] : []), [solid, have]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three }), [math, three]);
  const pick = ({ base, code }) => onChange({ base, code });
  return (
    <>
      {math && (!solid || three) && <Scene make={scene} value={shown} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="right">
        <Group name="Design">{math ? <Pick value={{ ...read(value), dim: DIM }} onChange={pick} math={math} dims={[DIM]} /> : <span>loading</span>}</Group>
        {facts && (
          <Group name="Counts">
            <Facts rows={ROWS} of={facts} />
          </Group>
        )}
        {facts && cut && CUTS[shown.view] && (
          <Group name={label(shown.view)}>
            <Facts rows={CUTS[shown.view]} of={cut} />
          </Group>
        )}
      </Bar>
      <Bar side="status">
        <span>{facts ? (cut ? cut.reading : tag(facts)) : 'loading'}</span>
      </Bar>
    </>
  );
}
