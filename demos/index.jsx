import { ready, ink, fit, paint } from './lib/mrly.js';
import { web, board, bars } from './lib/chart.js';
import { mount } from './lib/app.jsx';
import { Grid, Signs, Pixels, Markup, Sketch } from './lib/draw.jsx';

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

const sequences = (canvas) => {
  const b = board(canvas, canvas.clientWidth / 1.5, { top: 8, bottom: 8 });
  bars(b, m.ledger_terms('7', 2, 2, 'fills', 'level', 8, '500000').map((t) => Math.log10(Number(t))), { color: ink.gold, inset: 3 });
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

const TILES = [
  ['tour', 'The tour', 'A dozen sequences the designs write, each drawn live with its first terms and the OEIS record that holds them.', <Sketch draw={tour} className="" />],
  ['race', 'The race', 'Two designs of the same mass carry random walkers at different speeds.', <Grid grid={m.two_grid('127', 3, 4, 0, 3)} on={ink.blue} className="" />],
  ['sponge', 'The sponge', 'Any cube design, any level, in orbit.', <Markup className="thumb" svg={m.hex_svg('23', 3, 2, 2, 'iso', 3)} />],
  ['cuts', 'The cuts', 'A diagonal plane through one solid, the same size at every height.', <Markup className="thumb" svg={m.diagonal_svg('126', 2, 5, 2, JSON.parse(m.diagonal_profile('126', 2, 5, 2)).central, 6)} />],
  ['crop', 'The crop', 'A rational shape keeps only the cells of a design it reaches, counted before it is drawn.', <Grid grid={m.crop_grid('7', 3, 3, 2, 'ball', 55, 120, false, 'touching')} on={ink.green} className="" />],
  ['slices', 'The slices', 'The middle plane of an odd cube, a hexagon of triangles a design fills.', <Markup className="thumb" svg={m.hex_svg('23', 7, 1, 2, 'cut', 8)} />],
  ['spectra', 'The spectra', 'The Laplacian of a design, its degenerate families and the slope that reads a dimension.', <Grid grid={m.two_grid('7', 2, 5, 0, 2)} on={ink.pink} className="" />],
  ['universe', 'The universe', 'Every distinct design of the plane and the cube, grown on click.', <Grid grid={m.two_grid('9', 3, 3, 0, 2)} on={ink.gold} className="" />],
  ['words', 'The words', 'One design per level, folded by the Kronecker product, and what changes when the letters swap places.', <Pixels data={m.magic_pixels(['7', '14'], [3, 7], [2, 2])} className="" />],
  ['life', 'Life', 'Cellular automata with rules drawn from named sequences.', <Grid grid={{ width: 48, height: 48, types: m.life_noise(48, 48, 0.4, 3) }} on={ink.green} className="" />],
  ['moire', 'Moire', 'One design at many scales, stacked into interference.', <Pixels data={m.moire('weave', 11, 120, 'fire', 2, false)} className="" />],
  ['morse', 'The Thue-Morse word', 'The famous aperiodic sequence as a digit rule, a plus-minus Kronecker power, and the schedule the tree already uses.', <Signs grid={m.morse_lift('parity', 7)} className="" />],
  ['spin', 'The spin', 'A design on a turntable, and the exact bullseye it becomes at infinite speed.', <Pixels data={m.wheel(m.profile(Float32Array.from(m.two_grid('495', 3, 4, 0, 3).types), 81, 256), 180, 'fire', 64, false)} className="" />],
  ['radial', 'The radial stack', 'Turned copies of one design laid on each other, and the harmonics each stack keeps.', <Pixels data={m.sheet(m.radial(Float32Array.from(carpet.types), 27, 180, 5, 72, 'mean', 2), 180, 'fire', 64, false)} className="" />],
  ['volume', 'The volume', 'The moire stack of a cube design as a solid, cut on any plane, seen down the diagonal.', <Pixels data={m.paint_span(m.plane_field(solid, 48, [1, 1, 1], 0.5, 180), 180, range.min, range.max, 'fire', 16, false)} className="" />],
  ['farey', 'The Farey stack', 'Scales light the fractions; the primes light the most.', <Sketch draw={farey} className="" />],
  ['primes', 'The primes', 'Stones that make no rectangle: sieved, counted, and found by the carpet stack on its own.', <Grid grid={sieve.grid(15)} on={ink.gold} className="" />],
  ['ulam', 'The Ulam spiral', 'The whole numbers wound on squares or hexagons; the primes fall into diagonals nobody ordered.', <Pixels data={m.spiral_pixels('square', 61, 4, -2, 41, 'prime', false, 180)} className="" />],
  ['gaussian', 'Primes in the plane', 'The Gaussian and Eisenstein primes as snowflakes, coloured by how an ordinary prime broke up.', <Pixels data={m.ring_pixels('gaussian', 24, 'class', false, 180)} className="" />],
  ['graphs', 'The graphs', 'Every filled cell joined to its neighbours: tips, junctions and pieces, flat, in the cube, on the hexagon, and relaxed by force.', <Sketch draw={graphs} className="" />],
  ['sequences', 'The sequences', 'Every integer sequence the designs write, searched by terms, name, record or code, with the OEIS entry each one matches.', <Sketch draw={sequences} className="" />],
  ['integers', 'The integers', 'Type a number and see every row of the ledger that writes it, or the verdict that nothing does.', <Grid grid={{ width: 40, height: counts.length / 40, types: Uint8Array.from(counts, (rows) => (rows ? 1 : 0)) }} on={ink.gold} className="" />],
  ['zeta', 'The critical line', 'Zeta walked at one half plus it; every pass through the origin is a zero, and the zeros rebuild the prime staircase.', <Sketch draw={zeta} className="" />],
];

function App() {
  return (
    <>
      <header>
        <nav>mrlyprod / demos</nav>
        <h1>The eyes of MrlyMath</h1>
        <p className="sub">Every number and pixel on these pages comes out of the Rust crates through wasm. The browser only draws.</p>
      </header>
      <main>
        <div className="gallery">
          {TILES.map(([href, title, blurb, art]) => (
            <a key={href} className="tile" href={`./${href}`}>{art}<h2>{title}</h2><p>{blurb}</p></a>
          ))}
        </div>
      </main>
    </>
  );
}

mount(<App />);
