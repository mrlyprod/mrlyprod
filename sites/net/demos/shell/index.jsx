import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Slider, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Pixels, Sketch } from '../../lib/draw.jsx';
import { board, tag } from '../../lib/chart.js';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();
const RMAX = 242;
const LEAVES = 220;

function widths(nodes) {
  const out = [];
  for (let i = 0; i < nodes.length; i += 5) {
    const j = nodes[i];
    while (out.length <= j) out.push(0);
    out[j] += 1;
  }
  return out;
}

function fits(read) {
  let root = 0;
  for (let j = 0; j <= read.depth; j += 1) if (read.levels[0].boxes <= LEAVES * read.levels[j].boxes) root = j;
  return root;
}

function places(boxes, nodes) {
  const starts = [];
  let total = 0;
  for (const wide of boxes) {
    starts.push(total);
    total += wide;
  }
  const px = new Float64Array(total);
  const leaves = boxes[0];
  for (let k = 0; k < leaves; k += 1) px[k] = (leaves - k - 0.5) / leaves;
  for (let j = 1; j < boxes.length; j += 1) {
    const wide = boxes[j];
    const sum = new Float64Array(wide);
    const held = new Float64Array(wide);
    for (let k = 0; k < boxes[j - 1]; k += 1) {
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
  const [q, set] = useQuery({ code: seeded(s, 2, 2, '7'), r: 40, at: 2, zoom: -1, box: 0 });

  const view = useMemo(() => {
    try {
      const code = q.code.trim();
      const read = JSON.parse(m.shell_read(code, 3, 2, q.r));
      const at = Math.max(0, Math.min(q.at, read.depth));
      const root = q.zoom < 0 ? fits(read) : Math.max(0, Math.min(q.zoom, read.depth));
      const whole = root === read.depth;
      const pick = Math.max(0, Math.min(q.box, read.levels[root].boxes - 1));
      const nodes = m.shell_nodes(code, 3, 2, q.r, root, pick);
      const boxes = widths(nodes);
      return {
        read, at, nodes, boxes, root, pick, whole,
        art: m.shell_pixels(code, 3, 2, q.r, at, whole ? undefined : root, whole ? undefined : pick),
        name: m.name_of(code, 2, 2),
        layout: places(boxes, nodes),
        error: null,
      };
    } catch (error) {
      return { error };
    }
  }, [q.code, q.r, q.at, q.zoom, q.box]);

  const draw = (canvas) => {
    const { read, nodes, layout, boxes, at, whole } = view;
    if (!read) return;
    const b = board(canvas, 340, { left: 44, right: 16, top: 14, bottom: 14 });
    const { starts, px } = layout;
    const depth = boxes.length - 1;
    const row = (j) => b.y(depth ? j / depth : 0.5);
    if (at <= depth) {
      b.ctx.lineWidth = 1;
      b.ctx.strokeStyle = ink.pink;
      b.ctx.setLineDash([3, 4]);
      b.ctx.beginPath();
      b.ctx.moveTo(b.x(0), row(at));
      b.ctx.lineTo(b.x(1), row(at));
      b.ctx.stroke();
      b.ctx.setLineDash([]);
    }
    const paths = [new Path2D(), new Path2D()];
    for (let j = 0; j < depth; j += 1) {
      for (let k = 0; k < boxes[j]; k += 1) {
        const node = 5 * (starts[j] + k);
        const parent = nodes[node + 3];
        if (parent >= boxes[j + 1]) continue;
        const path = paths[nodes[node + 4]];
        path.moveTo(b.x(px[starts[j] + k]), row(j));
        path.lineTo(b.x(px[starts[j + 1] + parent]), row(j + 1));
      }
    }
    b.ctx.lineWidth = 1;
    b.ctx.strokeStyle = ink.blue;
    b.ctx.stroke(paths[0]);
    b.ctx.strokeStyle = ink.yellow;
    b.ctx.stroke(paths[1]);
    const dot = Math.max(1, Math.min(3, b.wide / boxes[0]));
    for (let j = 0; j <= depth; j += 1) {
      for (let k = 0; k < boxes[j]; k += 1) {
        b.ctx.fillStyle = nodes[5 * (starts[j] + k) + 4] ? ink.yellow : ink.blue;
        b.ctx.fillRect(b.x(px[starts[j] + k]) - dot / 2, row(j) - dot / 2, dot, dot);
      }
    }
    if (!whole) {
      b.ctx.strokeStyle = ink.green;
      b.ctx.lineWidth = 1.5;
      b.ctx.beginPath();
      b.ctx.arc(b.x(px[starts[depth]]), row(depth), 5, 0, Math.PI * 2);
      b.ctx.stroke();
    }
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    for (let j = 0; j <= depth; j += 1) b.ctx.fillText(`j=${j}`, b.left - 8, row(j) + 4);
    b.ctx.textAlign = 'left';
    if (at <= depth) {
      const count = whole ? `${boxes[at]}` : `${boxes[at]} of ${read.levels[at].boxes}`;
      tag(b, `${count} boxes at level ${at}`, ink.pink, 'right', b.x(1), row(at) - 8);
    }
  };

  const read = view.read;
  const rows = read ? read.levels : [];
  const roots = read ? read.levels[view.root].boxes : 1;

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={2} code={q.code} seeds={s} onChange={set} />
      </Group>
      <Group name="Circle">
        <Slider label="radius r" value={q.r} min={1} max={RMAX} onChange={(v) => set({ r: v })} />
        <Slider label="level j" value={view.at ?? 0} min={0} max={read ? read.depth : 1} onChange={(v) => set({ at: v })} />
      </Group>
      <Group name="Zoom">
        <Slider label="root" value={q.zoom} min={-1} max={read ? read.depth : 1}
          show={q.zoom < 0 ? `fit j=${view.root ?? 0}` : view.whole ? 'whole tree' : `j=${view.root ?? q.zoom}`}
          onChange={(v) => set({ zoom: v, box: 0 })} />
        <Slider label="box" value={view.pick ?? 0} min={0} max={Math.max(0, roots - 1)}
          show={`${(view.pick ?? 0) + 1}/${roots}`} onChange={(v) => set({ box: v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="shell" title="A circle on a carpet is a tree"
      sub={<>Draw the circle of radius <code>r</code> cells about the corner and keep the cells it crosses. Zoom out by threes: the crossed cells fall into crossed boxes, those into fewer boxes, and at last into one. That is a rooted tree, its leaves the crossed cells, and the carpet keeps only the leaves whose path never sat in a centre seat. Drag <code>r</code> and both panels move together. A wide radius gives more leaves than a panel has pixels, so the zoom picks one box of one level and draws only the branch hanging under it, ringed in the picture and at the top of the tree, with every count in that branch still exact.</>}
      controls={controls}
      foot={<>The picture is the design at the least level that holds the circle: its cells are the faint ground, a crossed cell the design fills is yellow, a crossed cell it drops is blue, and the boxes of the chosen level are outlined in pink so you can count them against the row of the table. The tree draws the same boxes, one row per level, an edge from every box to its parent, and the same two inks; the dashed rule marks the chosen level. Every box, every count and every colour comes from <code>mrlymath::shape::crossing_tree</code> through wasm, walked once along the arc in exact integers with no square root taken twice. The zoom is the same walk rooted lower down: a box's children are contiguous among its own level, so one branch is one range a level and nothing is dropped or thinned to make it fit, which a leaf cap could not promise. The <code>root</code> slider rests on <code>fit</code>, the deepest branch that still fits the panel, until you move it, and its top step is the tree's own root, which is the whole tree again. The table stays the whole circle, so the row beside the drawn level says how many of its boxes the branch carries. The same circle counted radius by radius instead of level by level is <a href="../crop">crop</a>, and the whole count with its proofs is on <a href="/research/crop/">the crop page</a>.</>}>
      <p><span className="chip proved">Proved</span> The circle crosses exactly <code>2r + 1</code> cells of the whole grid at every integer <code>r &gt;= 1</code>, and the level-<code>j</code> boxes carrying a crossed cell are the whole grid's crossing shell at the real radius <code>r / 3^j</code>, so there are <code>2 floor(r / 3^j) + 1</code> of them. Summing the children over one level gives the branching identity, so a box of level <code>j + 1</code> has <code>3 + (2k - 2) / (2Q + 1)</code> children on average with <code>Q = floor(r / 3^(j+1))</code> and <code>floor(r / 3^j) = 3Q + k</code>: exactly three at every level where <code>floor(r / 3^j)</code> is <code>1 mod 3</code>.</p>
      <div className="arena">
        <div className="panel">
          <h2>The circle on the design <span>{read && `level ${read.depth}, side ${read.side}`}</span></h2>
          {view.art && <Pixels data={view.art} role="img" aria-label="The design with the crossed cells lit and one level's boxes outlined" />}
        </div>
        <div className="panel">
          <h2>The crossing shell as a tree <span>{read && (view.whole ? `depth ${read.depth}, ${read.leaves} leaves` : `box ${view.pick + 1} of level ${view.root}, ${view.boxes[0]} of ${read.leaves} leaves`)}</span></h2>
          <Sketch draw={draw} deps={[view]} className="bars" role="img" aria-label="The crossing shell drawn as a rooted tree, surviving leaves yellow and pruned ones blue" />
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
