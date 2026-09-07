import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Slider, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Pixels, Sketch } from '../../lib/draw.jsx';
import { board, tag } from '../../lib/chart.js';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();
const RMAX = 242;

function places(read, nodes) {
  const starts = [];
  let total = 0;
  for (const row of read.levels) {
    starts.push(total);
    total += row.boxes;
  }
  const px = new Float64Array(total);
  const leaves = read.levels[0].boxes;
  for (let k = 0; k < leaves; k += 1) px[k] = (leaves - k - 0.5) / leaves;
  for (let j = 1; j <= read.depth; j += 1) {
    const wide = read.levels[j].boxes;
    const sum = new Float64Array(wide);
    const held = new Float64Array(wide);
    for (let k = 0; k < read.levels[j - 1].boxes; k += 1) {
      const parent = nodes[5 * (starts[j - 1] + k) + 3];
      if (parent >= wide) continue;
      sum[parent] += px[starts[j - 1] + k];
      held[parent] += 1;
    }
    for (let k = 0; k < wide; k += 1) px[starts[j] + k] = held[k] ? sum[k] / held[k] : 0;
  }
  return { starts, px };
}

function App() {
  const s = useSeeds();
  const [q, set] = useQuery({ code: seeded(s, 2, 2, '7'), r: 40, at: 2 });

  const view = useMemo(() => {
    try {
      const code = q.code.trim();
      const read = JSON.parse(m.shell_read(code, 3, 2, q.r));
      const at = Math.max(0, Math.min(q.at, read.depth));
      const nodes = m.shell_nodes(code, 3, 2, q.r);
      return {
        read, at, nodes,
        art: m.shell_pixels(code, 3, 2, q.r, at),
        name: m.name_of(code, 2, 2),
        layout: places(read, nodes),
        error: null,
      };
    } catch (error) {
      return { error };
    }
  }, [q.code, q.r, q.at]);

  const draw = (canvas) => {
    const { read, nodes, layout, at } = view;
    if (!read) return;
    const b = board(canvas, 340, { left: 44, right: 16, top: 14, bottom: 14 });
    const { starts, px } = layout;
    const depth = read.depth;
    const row = (j) => b.y(depth ? j / depth : 0.5);
    b.ctx.lineWidth = 1;
    b.ctx.strokeStyle = ink.pink;
    b.ctx.setLineDash([3, 4]);
    b.ctx.beginPath();
    b.ctx.moveTo(b.x(0), row(at));
    b.ctx.lineTo(b.x(1), row(at));
    b.ctx.stroke();
    b.ctx.setLineDash([]);
    const paths = [new Path2D(), new Path2D()];
    for (let j = 0; j < depth; j += 1) {
      for (let k = 0; k < read.levels[j].boxes; k += 1) {
        const node = 5 * (starts[j] + k);
        const parent = nodes[node + 3];
        if (parent >= read.levels[j + 1].boxes) continue;
        const path = paths[nodes[node + 4]];
        path.moveTo(b.x(px[starts[j] + k]), row(j));
        path.lineTo(b.x(px[starts[j + 1] + parent]), row(j + 1));
      }
    }
    b.ctx.lineWidth = 1;
    b.ctx.strokeStyle = ink.blue;
    b.ctx.stroke(paths[0]);
    b.ctx.strokeStyle = ink.gold;
    b.ctx.stroke(paths[1]);
    const dot = Math.max(1, Math.min(3, b.wide / read.levels[0].boxes));
    for (let j = 0; j <= depth; j += 1) {
      for (let k = 0; k < read.levels[j].boxes; k += 1) {
        b.ctx.fillStyle = nodes[5 * (starts[j] + k) + 4] ? ink.gold : ink.blue;
        b.ctx.fillRect(b.x(px[starts[j] + k]) - dot / 2, row(j) - dot / 2, dot, dot);
      }
    }
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    for (let j = 0; j <= depth; j += 1) b.ctx.fillText(`j=${j}`, b.left - 8, row(j) + 4);
    b.ctx.textAlign = 'left';
    tag(b, `${read.levels[at].boxes} boxes at level ${at}`, ink.pink, 'right', b.x(1), row(at) - 8);
  };

  const read = view.read;
  const rows = read ? read.levels : [];

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={2} code={q.code} seeds={s} onChange={set} />
      </Group>
      <Group name="Circle">
        <Slider label="radius r" value={q.r} min={1} max={RMAX} onChange={(v) => set({ r: v })} />
        <Slider label="level j" value={view.at ?? 0} min={0} max={read ? read.depth : 1} onChange={(v) => set({ at: v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="shell" title="A circle on a carpet is a tree"
      sub={<>Draw the circle of radius <code>r</code> cells about the corner and keep the cells it crosses. Zoom out by threes: the crossed cells fall into crossed boxes, those into fewer boxes, and at last into one. That is a rooted tree, its leaves the crossed cells, and the carpet keeps only the leaves whose path never sat in a centre seat. Drag <code>r</code> and both panels move together.</>}
      controls={controls}
      foot={<>The picture is the design at the least level that holds the circle: its cells are the faint ground, a crossed cell the design fills is gold, a crossed cell it drops is blue, and the boxes of the chosen level are outlined in pink so you can count them against the row of the table. The tree draws the same boxes, one row per level, an edge from every box to its parent, and the same two inks; the dashed rule marks the chosen level. Every box, every count and every colour comes from <code>mrlymath::shape::crossing_tree</code> through wasm, walked once along the arc in exact integers with no square root taken twice. The same circle counted radius by radius instead of level by level is <a href="../crop">crop</a>, and the whole count with its proofs is on <a href="/research/crop/">the crop page</a>.</>}>
      <p><span className="chip proved">Proved</span> The circle crosses exactly <code>2r + 1</code> cells of the whole grid at every integer <code>r &gt;= 1</code>, and the level-<code>j</code> boxes carrying a crossed cell are the whole grid's crossing shell at the real radius <code>r / 3^j</code>, so there are <code>2 floor(r / 3^j) + 1</code> of them. Summing the children over one level gives the branching identity, so a box of level <code>j + 1</code> has <code>3 + (2k - 2) / (2Q + 1)</code> children on average with <code>Q = floor(r / 3^(j+1))</code> and <code>floor(r / 3^j) = 3Q + k</code>: exactly three at every level where <code>floor(r / 3^j)</code> is <code>1 mod 3</code>.</p>
      <div className="arena">
        <div className="panel">
          <h2>The circle on the design <span>{read && `level ${read.depth}, side ${read.side}`}</span></h2>
          {view.art && <Pixels data={view.art} role="img" aria-label="The design with the crossed cells lit and one level's boxes outlined" />}
        </div>
        <div className="panel">
          <h2>The crossing shell as a tree <span>{read && `depth ${read.depth}, ${read.leaves} leaves`}</span></h2>
          <Sketch draw={draw} deps={[view]} className="bars" role="img" aria-label="The crossing shell drawn as a rooted tree, surviving leaves gold and pruned ones blue" />
        </div>
      </div>
      <Stats>
        <Stat label="design">{view.name}</Stat>
        <Stat label="radius">{q.r}</Stat>
        <Stat label="depth L">{read?.depth}</Stat>
        <Stat label="leaves 2r + 1">{read?.leaves}</Stat>
        <Stat label="kept leaves C(r)">{read?.live}</Stat>
        <Stat label="pruned">{read && read.leaves - read.live}</Stat>
        <Stat label="counts exact">{read && (read.exact ? 'yes' : 'no')}</Stat>
        <Stat label="boxes without a parent">{read?.orphans}</Stat>
      </Stats>
      <div className="panel">
        <h2>Every level, against the identity <span>click a row to outline that level</span></h2>
        <div className="scroll">
          <table>
            <thead><tr><th>level j</th><th>boxes</th><th>2 floor(r / 3^j) + 1</th><th>kept</th><th>children per box</th><th>reading</th></tr></thead>
            <tbody>
              {rows.map((row) => (
                <tr key={row.level} className={row.level === view.at ? 'on' : undefined} onClick={() => set({ at: row.level })}>
                  <td className="num">{row.level}</td>
                  <td className="num">{row.boxes}</td>
                  <td className="num">{row.want}</td>
                  <td className="num">{row.live}</td>
                  <td className="num">{row.level ? row.branch.toFixed(6) : '-'}</td>
                  <td className="num">{row.level ? (row.three ? 'exactly 3' : '') : `${row.boxes} crossed cells`}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
      <Note error={view.error} />
    </Page>
  );
}

mount(<App />);
