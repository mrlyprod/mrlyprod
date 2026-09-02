import { ready, ink, fit, paint } from './lib/mrly.js';
import { web, board, bars, line } from './lib/chart.js';
import { mount } from './lib/app.jsx';
import { Grid, Signs, Pixels, Markup, Sketch } from './lib/draw.jsx';
import manifest from './pages.json';

const m = await ready();

const tour = (canvas) => {
  const [ctx, w, h] = fit(canvas, canvas.clientWidth / 1.5);
  ctx.imageSmoothingEnabled = false;
  const codes = ['1', '3', '7', '9', '11', '15'];
  const slot = (w - 12) / codes.length, size = Math.min(slot - 6, h - 12);
  codes.forEach((code, i) => {
    const tile = document.createElement('canvas');
    paint(tile, m.two_grid(code, 5, 1, 0, 2), ink.gold);
    ctx.drawImage(tile, 6 + i * slot + (slot - size) / 2, (h - size) / 2, size, size);
  });
};

const graphs = (canvas) => {
  web(canvas, canvas.clientWidth / 1.5, m.graph_nodes('flat', '495', 3, 2, 3, 'core').subarray(2), m.graph_branches('flat', '495', 3, 2, 3, 'core'), null, 3);
};

const farey = (canvas) => {
  const [ctx, w, h] = fit(canvas, canvas.clientWidth / 1.5);
  ctx.strokeStyle = ink.pink;
  for (const [num, den, bright] of JSON.parse(m.farey(24))) {
    const x = 6 + (w - 12) * num / den;
    ctx.globalAlpha = 0.25 + 0.75 * bright / 24;
    ctx.beginPath();
    ctx.moveTo(x, h - 6);
    ctx.lineTo(x, h - 6 - (h - 12) * bright / 24);
    ctx.stroke();
  }
};

const spectrometer = (canvas) => {
  const b = board(canvas, canvas.clientWidth / 1.5, { top: 10, bottom: 10 });
  const rows = JSON.parse(m.walsh_spectrum('105', 13)).law.slice(1);
  line(b, rows.map((row, i) => [(i + 0.5) / rows.length, row.ink]), ink.blue, { dots: 2.5 });
};

const sequences = (canvas) => {
  const b = board(canvas, canvas.clientWidth / 1.5, { top: 8, bottom: 8 });
  bars(b, m.ledger_terms('7', 2, 2, 'fills', 'level', 8, '500000').map((t) => Math.log10(Number(t))), { color: ink.gold, inset: 3 });
};

const plot = (canvas) => {
  const b = board(canvas, canvas.clientWidth / 1.5, { top: 8, bottom: 8 });
  const logs = JSON.parse(m.blend_series('23', 3, 2, 'surface', 'level', 12, '500000', 1)).log10;
  const peak = Math.max(...logs);
  bars(b, logs, { peak, color: ink.gold, inset: 3 });
  line(b, logs.map((v, i) => [(i + 0.5) / logs.length, v / peak]), ink.blue, { width: 1.4, dots: 2.4 });
};

const tower = (canvas) => {
  const [ctx, w, h] = fit(canvas, canvas.clientWidth / 1.5);
  ctx.imageSmoothingEnabled = false;
  const slot = (w - 12) / 4, size = Math.min(slot - 6, h - 12);
  for (let k = 1; k <= 4; k++) {
    const block = document.createElement('canvas');
    paint(block, k === 1 ? m.two_grid('7', 2, 1, 0, 2) : m.magic_grid(Array(k).fill('7'), Array(k).fill(2), Array(k).fill(2)), ink.gold);
    ctx.drawImage(block, 6 + (k - 1) * slot + (slot - size) / 2, (h - size) / 2, size, size);
  }
};

const carry = (canvas) => {
  const rows = JSON.parse(m.carry_signs(m.carry_cap(3)));
  const b = board(canvas, canvas.clientWidth / 1.5, { top: 8, bottom: 8 });
  const step = b.wide / rows.length;
  const mid = (b.roof + b.floor) / 2;
  const tall = (b.floor - b.roof) / 2 - 4;
  rows.forEach((row, i) => {
    const up = row.three.sign > 0;
    b.ctx.fillStyle = up ? ink.orange : ink.blue;
    b.ctx.fillRect(b.x(i / rows.length) + 1, up ? mid - tall : mid + 4, Math.max(1, step - 2), tall);
  });
  line(b, [[0, 0.5], [1, 0.5]], ink.dim, { width: 1, dash: [3, 3] });
};

const zeta = (canvas) => {
  const [ctx, w, h] = fit(canvas, canvas.clientWidth / 1.5);
  const path = m.zeta_line(0, 50, 600);
  let reach = 1;
  for (let k = 0; k < path.length; k += 4) reach = Math.max(reach, Math.abs(path[k + 1]), Math.abs(path[k + 2]));
  const scale = (h / 2 - 6) / reach;
  ctx.strokeStyle = ink.blue;
  ctx.lineWidth = 1.2;
  ctx.beginPath();
  for (let k = 0; k < path.length; k += 4) {
    const x = w / 2 + path[k + 1] * scale, y = h / 2 - path[k + 2] * scale;
    if (k) ctx.lineTo(x, y);
    else ctx.moveTo(x, y);
  }
  ctx.stroke();
  ctx.strokeStyle = ink.gold;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(w / 2, h / 2, 4, 0, Math.PI * 2);
  ctx.stroke();
};

const carpet = m.two_grid('495', 3, 3, 0, 3);
const solid = m.volume('23', 2, 7, 'sum', 1, 48);
const range = JSON.parse(m.volume_stats(solid, 48));
const sieve = new m.Sieve(150);
sieve.finish();
m.census_walk(JSON.parse(m.census_window()).tiers[0].keys);
const counts = m.census_counts();

const ART = {
  tour: <Sketch draw={tour} className="" />,
  race: <Grid grid={m.two_grid('127', 3, 4, 0, 3)} on={ink.blue} className="" />,
  sponge: <Markup className="thumb" svg={m.hex_svg('23', 3, 2, 2, 'iso', 3)} />,
  tile: <Markup className="thumb" svg={m.tile_svg('23', 3, 1, 2, 'cut', 5, 3, true, 6)} />,
  cuts: <Markup className="thumb" svg={m.diagonal_svg('126', 2, 5, 2, JSON.parse(m.diagonal_profile('126', 2, 5, 2)).central, 6)} />,
  crop: <Grid grid={m.crop_grid('7', 3, 3, 2, 'ball', 55, 120, false, 'touching')} on={ink.green} className="" />,
  slices: <Markup className="thumb" svg={m.hex_svg('23', 7, 1, 2, 'cut', 8)} />,
  spectrometer: <Sketch draw={spectrometer} className="" />,
  spectra: <Grid grid={m.two_grid('7', 2, 5, 0, 2)} on={ink.pink} className="" />,
  universe: <Grid grid={m.two_grid('9', 3, 3, 0, 2)} on={ink.gold} className="" />,
  words: <Pixels data={m.magic_pixels(['7', '14'], [3, 7], [2, 2])} className="" />,
  life: <Grid grid={{ width: 48, height: 48, types: m.life_noise(48, 48, 0.4, 3) }} on={ink.green} className="" />,
  moire: <Pixels data={m.moire('weave', 11, 120, 'fire', 2, false)} className="" />,
  morse: <Signs grid={m.morse_lift('parity', 7)} className="" />,
  spin: <Pixels data={m.wheel(m.profile(Float32Array.from(m.two_grid('495', 3, 4, 0, 3).types), 81, 256), 180, 'fire', 64, false)} className="" />,
  radial: <Pixels data={m.sheet(m.radial(Float32Array.from(carpet.types), 27, 180, 5, 72, 'mean', 2), 180, 'fire', 64, false)} className="" />,
  volume: <Pixels data={m.paint_span(m.plane_field(solid, 48, [1, 1, 1], 0.5, 180), 180, range.min, range.max, 'fire', 16, false)} className="" />,
  tower: <Sketch draw={tower} className="" />,
  carry: <Sketch draw={carry} className="" />,
  farey: <Sketch draw={farey} className="" />,
  primes: <Grid grid={sieve.grid(15)} on={ink.gold} className="" />,
  ulam: <Pixels data={m.spiral_pixels('square', 61, 4, -2, 41, 'prime', false, 180)} className="" />,
  gaussian: <Pixels data={m.ring_pixels('gaussian', 24, 'class', false, 180)} className="" />,
  graphs: <Sketch draw={graphs} className="" />,
  sequences: <Sketch draw={sequences} className="" />,
  plot: <Sketch draw={plot} className="" />,
  integers: <Grid grid={{ width: 40, height: counts.length / 40, types: Uint8Array.from(counts, (rows) => (rows ? 1 : 0)) }} on={ink.gold} className="" />,
  zeta: <Sketch draw={zeta} className="" />,
};

const BLANK = <Grid grid={m.two_grid('105', 2, 5, 0, 2)} on={ink.dim} className="" />;

const GROUPS = manifest.shelves.reduce((groups, shelf) => {
  const last = groups[groups.length - 1];
  if (last && last.name === shelf.group) last.shelves.push(shelf);
  else groups.push({ name: shelf.group, shelves: [shelf] });
  return groups;
}, []);

function Shelf({ shelf }) {
  const rows = manifest.pages.filter((page) => page.category === shelf.key);
  if (!rows.length) return null;
  return (
    <>
      <div className="shelf">
        <h2>{shelf.title}</h2>
        <p>{shelf.blurb}</p>
      </div>
      <div className="gallery">
        {rows.map((page) => (
          <a key={page.name} className="tile" href={`./${page.name}`}>
            {ART[page.name] ?? BLANK}
            <h2>{page.title}</h2>
            <p>{page.blurb}</p>
          </a>
        ))}
      </div>
    </>
  );
}

function App() {
  return (
    <>
      <header>
        <nav className="links">
          <a href="./">Demos</a>
          <a href="./papers/">Papers</a>
          <a href="./research/">Research</a>
        </nav>
        <h1>The eyes of MrlyMath</h1>
        <p className="sub">Every number and pixel on these pages comes out of the Rust crates through wasm. The browser only draws.</p>
      </header>
      <main>
        {GROUPS.map((group) => (
          <section key={group.name}>
            <h2 className="group">{group.name}</h2>
            {group.shelves.map((shelf) => <Shelf key={shelf.key} shelf={shelf} />)}
          </section>
        ))}
      </main>
    </>
  );
}

mount(<App />);
