import { useMemo, useRef, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { cubes } from '../../lib/stage.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Grid, Markup } from '../../lib/draw.jsx';
import { Stage } from '../../lib/stage.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();
const DIMS = [[2, 'the plane'], [3, 'the cube'], [6, 'the hexagon']];
const PROJECTIONS = [['cut', 'the middle slice'], ['pro', 'three facing sides'], ['iso', 'the isometric skin']];
const PRESETS = [[5, 5], [3, 9]];
const PLANE_CELLS = 262144;
const SOLID_CELLS = 1000000;
const HEX_DRAWN = 24000;
const SOLID_FILLS = 150000;
const SEED = { 2: ['495', 3], 3: ['23', 2], 6: ['23', 2] };

function capOf(q, reps) {
  const copies = reps.reduce((a, b) => a * b, 1);
  try {
    if (q.dim === 2) return m.level_cap(q.number, 2, Math.floor(PLANE_CELLS / copies));
    if (q.dim === 3) {
      const room = m.level_cap(q.number, 3, Math.floor(SOLID_CELLS / copies));
      return Math.min(room, m.fill_cap(q.code.trim(), q.number, 3, q.base, Math.floor(SOLID_FILLS / copies)));
    }
    return Math.min(m.level_cap(q.number, 1, 81), m.level_cap(q.number, 2, Math.floor(HEX_DRAWN / (8 * copies))));
  } catch {
    return 1;
  }
}

function App() {
  const s = useSeeds();
  const [q, set] = useQuery({ dim: 2, code: seeded(s, 2, 3, '495'), base: 3, number: 3, level: 2, x: 5, y: 5, z: 5, proj: 'cut', overhang: true });
  const [spin, setSpin] = useState(true);
  const live = useRef(null);

  const reps = q.dim === 3 ? [q.x, q.y, q.z] : [q.x, q.y];
  const top = capOf(q, reps);
  const level = Math.min(q.level, top);
  const code = q.code.trim();
  const solid = q.dim !== 2;
  const crop = !q.overhang && q.x >= 2 && q.y >= 2;

  const data = useMemo(() => {
    const out = {};
    try {
      out.name = m.name_of(code, solid ? 3 : 2, q.base);
      out.census = JSON.parse(m.tile_census(code, q.number, level, q.base, q.dim, q.proj, reps, crop));
      if (q.dim === 2) out.grid = m.tile_grid(code, q.number, level, q.base, q.x, q.y);
      else if (q.dim === 3) out.cells = m.tile_cells(code, q.number, level, q.base, q.x, q.y, q.z);
      else {
        if (out.census.triangles > HEX_DRAWN) throw new Error(`${out.census.triangles} triangles is more than this page draws; lower the level or the repeats.`);
        const scale = Math.max(1, Math.round(900 / out.census.sheet[0]));
        const art = m.tile_svg(code, q.number, level, q.base, q.proj, q.x, q.y, crop, scale);
        if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level or the repeats.');
        out.art = art;
      }
    } catch (error) {
      out.error = error;
    }
    return out;
  }, [code, q.base, q.number, level, q.dim, q.proj, q.x, q.y, q.z, crop]);

  const census = data.census ?? {};
  const sheet = `${data.name ?? ''} repeated into a sheet`;

  const shift = (value) => {
    const dim = +value;
    const [code, base] = SEED[dim];
    s.drop();
    set({ dim, code, base, number: 3, level: dim === 2 ? 2 : 1 });
  };

  const preset = ([a, b]) => set({ x: a, y: b, z: a });

  const turn = (on) => {
    setSpin(on);
    if (live.current) live.current.spin = on ? 0.004 : 0;
  };

  const onStage = (stage) => {
    live.current = stage;
    stage.spin = spin ? 0.004 : 0;
    if (!data.cells) {
      stage.clear();
      return;
    }
    const span = [census.sheet[1], census.sheet[0], census.sheet[2]];
    const side = Math.max(...span);
    const mesh = cubes(data.cells, side, ink.orange);
    mesh.position.set(...span.map((n) => 1 - n / side));
    stage.show(mesh);
  };

  const controls = (
    <>
      <Group name="Design">
        <Pick label="dimension" value={q.dim} options={DIMS} onChange={shift} />
        <Picker dimension={solid ? 3 : 2} bases={q.dim === 2 ? [3, 2] : [2, 3]} code={q.code} base={q.base} seeds={s} onChange={set} />
        <Pick label="side" value={q.number} options={[[3, 3], [5, 5], [7, 7]]} onChange={(v) => set({ number: +v })} />
        <Slider label="level" value={level} min={1} max={top} onChange={(v) => set({ level: v })} />
      </Group>
      <Group name="Repeats">
        <Slider label="across" value={q.x} min={1} max={12} onChange={(v) => set({ x: v })} />
        <Slider label="down" value={q.y} min={1} max={12} onChange={(v) => set({ y: v })} />
        {q.dim === 3 && <Slider label="deep" value={q.z} min={1} max={12} onChange={(v) => set({ z: v })} />}
        <span className="tabs" role="group" aria-label="Repeat presets">
          {PRESETS.map(([a, b]) => <Btn key={`${a}.${b}`} on={q.x === a && q.y === b} onClick={() => preset([a, b])}>tile({a},{b})</Btn>)}
        </span>
      </Group>
      {q.dim === 6 && (
        <Group name="View">
          <Pick label="projection" value={q.proj} options={PROJECTIONS} onChange={(v) => set({ proj: v })} />
          <Check label="keep the overhang" checked={q.overhang} onChange={(v) => set({ overhang: v })} />
        </Group>
      )}
      {q.dim === 3 && (
        <Group name="View">
          <Check label="spin" checked={spin} onChange={turn} />
        </Group>
      )}
    </>
  );

  return (
    <Page crumb="tile" title="One design repeated tiles the plane, the cube and the hexagon" controls={controls}
      sub="One design, repeated. The same rule that fills the corners of a cube lays its tile side by side on the square lattice in the plane and in the cube, and interlocks it as a hexagon on the triangular one. On an uncropped sheet the fills multiply by the copy count exactly; the exposed faces do not, because every copy buries the faces it shares with a neighbour, and a cropped hexagon sheet trims fills away as well."
      foot={<>The tile is the design at its level and the sheet is that tile repeated, both built in Rust before a pixel is drawn. The hexagon interlocks rather than butts: the crop trims the jagged overhang to a clean rectangle and slides the whole triangle grid by one interlocking step, so the crate flips the start parity to keep every triangle pointing the way the tile's do, and backs a tall sheet of wide hexagons onto a wider frame so the renderer still reads the hexagon the right way up. Exposed means perimeter in the plane, surface in the cube and the boundary edges of the filled sub-mesh on the hexagon; buried is the difference between the copies' own exposure and the sheet's, and under the crop that difference also counts what the trim removed. Why a design's symmetries are symmetries of the infinite tiling and not of a truncation to a finite side is in <a href="/research/core/">the core</a>. Nearby: <a href="../words">the words</a> nests one design inside another instead of repeating it, <a href="../slices">the slices</a> counts the single hexagon, <a href="../sponge">the sponge</a> grows the cube.</>}>
      {q.dim === 2 && data.grid && <Grid grid={data.grid} on={ink.gold} role="img" aria-label={sheet} />}
      <Stage hidden={q.dim !== 3} role="img" aria-label={sheet} deps={[data]} onStage={onStage} />
      {q.dim === 6 && <Markup svg={data.art ?? ''} role="img" aria-label={sheet} />}
      <Stats>
        <Stat label="name">{data.name}</Stat>
        <Stat label="tile side">{census.side}</Stat>
        <Stat label="tile">{census.tile?.join(' x ')}</Stat>
        <Stat label="copies">{census.copies}</Stat>
        <Stat label="sheet">{census.sheet?.join(' x ')}</Stat>
        <Stat label={q.dim === 6 ? 'triangles' : 'cells'}>{census.cells}</Stat>
      </Stats>
      <Stats>
        <Stat label="filled">{census.fills}</Stat>
        <Stat label="empty">{census.voids}</Stat>
        <Stat label="density">{census.ratio?.toFixed(6)}</Stat>
        <Stat label="exposed">{census.exposed}</Stat>
        <Stat label="one copy exposed">{census.tile_exposed}</Stat>
        <Stat label={crop ? "buried or trimmed" : "buried by the tiling"}>{census.buried}</Stat>
        {census.walked && <span>corners <b>{census.vertices}</b> edges <b>{census.edges}</b>{census.faces ? <> faces <b>{census.faces}</b></> : null} euler <b>{census.euler}</b></span>}
      </Stats>
      <Note error={data.error} />
    </Page>
  );
}

mount(<App />);
