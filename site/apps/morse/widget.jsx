import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { next } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DESIGNED, DIM, VIEWS } from './engine.js';
import { make } from './scene.js';

const yes = (on) => (on ? 'yes' : 'no');

const site = ([row, col]) => `row ${row}, col ${col}`;

const CLOSED = { sign: 'the tile repeated', design: 'the tile complement' };

export const ROWS = {
  plane: [
    ['Side', (f) => `${f.side} by ${f.side}`],
    ['Plus', (f) => f.plus],
    ['Minus', (f) => f.minus],
    ['Tile', (f) => f.tile],
    ['Kronecker power', (f) => (f.folds ? 'yes' : `no, ${f.faults} faults`)],
    ['First fault', (f) => (f.first ? site(f.first) : '-')],
    ['Same as', (f) => (f.twin ? f.twin.toLowerCase() : '-')],
  ],
  word: [
    ['Letters', (f) => f.letters],
    ['Plus', (f) => f.plus],
    ['Minus', (f) => f.minus],
    ['Runs', (f) => f.runs],
    ['Longest run', (f) => f.longest],
    ['Runs of one', (f) => f.singles],
    ['Runs of two', (f) => f.doubles],
    ['Cube free', (f) => yes(f.cube)],
    ['Boundary', (f) => (f.doubling ? 'the doubling word' : 'not the doubling word')],
    ['Digit rule', (f) => (f.rule ? 'the same word' : 'a different word')],
  ],
  lift: [
    ['Tile', (f) => f.tile],
    ['Side', (f) => `${f.side} by ${f.side}`],
    ['Filled', (f) => f.filled],
    ['Plus', (f) => f.plus],
    ['Agree', (f) => f.agree],
    ['Differ', (f) => f.differ],
    ['Thue-Morse', (f) => yes(f.exact)],
  ],
  filter: [
    ['Tile', (f) => f.tile],
    ['Side', (f) => `${f.side} by ${f.side}`],
    ['Differ', (f) => f.differ],
    ['Closed form', (f) => (f.closed ? CLOSED[f.fold] : 'no')],
    ['Off Thue-Morse', (f) => (f.morse === null ? '-' : `${f.morse} of ${f.cells}${f.half ? ', a coin' : ''}`)],
  ],
};

export function status(f) {
  if (f.view === 'plane') return `${f.kind.toLowerCase()}, ${f.formula}`;
  if (f.view === 'word') return `${f.letters} letters, ${f.runs} runs`;
  const name = f.name ? `${f.name} ${f.code}` : f.title;
  if (f.view === 'filter') return `${name}, ${f.fold === 'sign' ? 'plus-minus' : 'design'} filter`;
  return f.exact ? `${name}, the Thue-Morse grid` : name;
}

export function labels(f) {
  if (f.view === 'lift') return 'design, then its sign power';
  if (f.view === 'filter') return `level ${f.level} blown up, level ${f.level + 1}, their difference`;
  return '';
}

export const hands = (handle, now, onChange) => ({
  deeper: (by) => onChange({ level: now.current.level + by }),
  turn: (by) => onChange({ view: next(VIEWS.map(([key]) => key), now.current.view, by) }),
  again: () => (now.current.grow ? handle.again() : onChange({ grow: 1 })),
});

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport, units: given }) {
  const [loaded, setLoaded] = useState(null);
  const units = given ?? loaded;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  const [sheet, setSheet] = useState(false);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, num, ready }) => ready.then(() => on && setLoaded({ math, num })));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units }), [units]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    setSheet(Boolean(handle?.svg));
    onReady?.(handle && { ...handle, ...hands(handle, now, onChange) });
  };
  const design = ({ dim, ...patch }) => onChange(patch);
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `morse-${now.current.seed}`, 'svg');
  };
  useEffect(() => onExport?.(sheet ? [['SVG', keep]] : []), [sheet]);
  const designed = DESIGNED.includes(value.view);
  const f = facts;
  return (
    <>
      {units && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Group name="Design" hidden={!designed}>{units && designed && <Pick value={{ ...read(value), dim: DIM }} onChange={design} math={units.math} dims={[DIM]} />}</Group>
      </Bar>
      <Bar side="status">
        {!units && <span>loading</span>}
        {f && <span>{status(f)}</span>}
        {f && labels(f) && <span>{labels(f)}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS[f.view].map(([name, get]) => <Fact key={name} name={name} value={get(f)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
