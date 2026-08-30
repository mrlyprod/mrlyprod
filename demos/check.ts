import init, * as m from './pkg/mrlyweb.js';

const bytes = await Bun.file(new URL('./pkg/mrlyweb_bg.wasm', import.meta.url)).arrayBuffer();
await init({ module_or_path: bytes });

const blinker = new Uint8Array(25);
for (const site of [7, 12, 17]) blinker[site] = 1;
const grid = m.two_grid('7', 3, 3, 0, 2);
const faces = m.three_faces('23', 3, 3, 2);
const race = new m.Race('127', 3, 4, 3, 300, 1);
const cut = JSON.parse(m.diagonal_profile('126', 2, 4, 2));
const art = m.diagonal_svg('126', 2, 3, 2, [10, 11], 4);
const slice = JSON.parse(m.slice_census('23', 3, 1, 2));
const deep = JSON.parse(m.slice_census('23', 3, 2, 2));
const series = JSON.parse(m.slice_series('23', 6));
const flat = JSON.parse(m.spectrum('flat', '7', 2, 4, true, 0.1));
const piece = JSON.parse(m.spectrum('slice', '23', 3, 1, true, 0.1));
const plain = JSON.parse(m.spectrum('flat', '7', 2, 2, false, 0.1));
const rings = m.profile(Float32Array.from(m.two_grid('495', 3, 3, 0, 3).types), 27, 1000);
const solid = m.volume('23', 2, 3, 'sum', 1, 9);
const hexcut = m.paint_span(m.plane_field(solid, 9, [1, 1, 1], 0.5, 64), 64, 0, 2, 'fire', 8, false);
const spun = JSON.parse(m.spin_stats(rings, 27));
const square = new Float32Array(64).fill(1);
const star = m.radial(square, 8, 64, 2, 45, 'union', 1);

const checks: [string, unknown, unknown][] = [
  ['two_grid 7 side', grid.width, 27],
  ['two_grid 7 fills', grid.types.reduce((a, b) => a + b, 0), 512],
  ['fill sponge level 3', m.fills('23', 3, 3, 3, 2), '8000'],
  ['void sponge level 3', m.voids('23', 3, 3, 3, 2), '11683'],
  ['universe counts D=1..4', m.counting_sequence(4).join(','), '3,6,22,402'],
  ['base 3 counts D=1..2', m.baseq_sequence(3, 2).join(','), '4,26'],
  ['sponge level 3 quads', faces[0] / 36, 18048],
  ['sponge level 3 buffer', faces.length, 2 + faces[0]],
  ['three_surface level 3', m.three_surface('23', 3, 3, 2), '18048'],
  ['three_cells level 1', m.three_cells('23', 3, 1, 2).length / 3, 20],
  ['three_census euler', JSON.parse(m.three_census('23', 3, 1, 2)).euler, -4],
  ['universe 3 distinct', JSON.parse(m.universe(3)).distinct, 22],
  ['universe 2 orbit of 1', JSON.parse(m.universe(2)).designs[1].orbit, 4],
  ['name_of 127', m.name_of('127', 2, 3), 'mrly_bang_d2_q3_127'],
  ['name_parse sponge', JSON.parse(m.name_parse('mrly_bang_d3_23')).code, '23'],
  ['press members', m.press_members('2', 1, 2, 5).join(','), '1,3,7,15,31'],
  ['press count_below', m.press_count_below('7', 2, 2, '27'), '18'],
  ['life blinker loop', JSON.parse(m.life_run(blinker, 5, 5, [3], [2, 3], false, 16)).loop, 2],
  ['life primes', Array.from(m.life_sequence('primes', 8)).join(','), '2,3,5,7'],
  ['moire heatmap bytes', m.moire('heatmap', 9, 32, 'fire', 64, false).rgba.length, 4096],
  ['hex_svg polygons', m.hex_svg('23', 3, 1, 2, 'iso', 10).includes('<polygon'), true],
  ['slice_census 23 mesh', `${slice.triangles},${slice.fills},${slice.vertices},${slice.euler}`, '54,42,37,1'],
  ['slice_census 23 pieces', `${slice.components},${slice.holes}`, '1,1'],
  ['slice_census 23 closed', slice.closed.vertices, '37'],
  ['slice_census 232 fills', JSON.parse(m.slice_census('232', 3, 1, 2)).fills, 12],
  ['slice_census 23 level 2', `${deep.fills},${deep.holes}`, '306,7'],
  ['slice_series 23 components', series.map((row: {components: number}) => row.components).join(','), '1,1,7,1,19,1'],
  ['slice_series 23 holes', series.map((row: {holes: number}) => row.holes).join(','), '0,1,0,7,0,19'],
  ['spectrum flat 7 nodes', `${flat.nodes},${flat.distinct}`, '81,43'],
  ['spectrum flat 7 degeneracy', `${flat.classes},${flat.one}`, '9,27'],
  ['spectrum flat 7 pair', flat.pair.join(','), '4,4'],
  ['spectrum slice 23 nodes', piece.nodes, 42],
  ['spectrum slice 23 exponent', piece.exponent.toFixed(2), '0.91'],
  ['spectrum slice 23 fit slope', (piece.fit[1] * 2).toFixed(4), piece.exponent.toFixed(4)],
  ['spectrum slice 23 stair', `${piece.stair.at(-1)[1]},${piece.fitted},${piece.stair.length < piece.nodes}`, '1,4,true'],
  ['spectrum flat 7 plain nodes', `${plain.nodes},${plain.distinct}`, '9,8'],
  ['spectrum flat 7 plain degeneracy', `${plain.classes},${plain.one}`, '1,2'],
  ['profile 495 disc opens', rings.findIndex((v: number) => v > 0), 236],
  ['profile 495 ring 600', rings[600].toFixed(4), '0.8972'],
  ['spin_stats 495 mass', spun.mass.toFixed(1), '512.0'],
  ['spin_stats 495 disc', spun.disc.toFixed(1), '4.5'],
  ['wheel bytes', m.wheel(rings, 64, 'fire', 16, false).rgba.length, 16384],
  ['slice_grid 23 centre', m.slice_grid('23', 3, 1, 2, 101).types[50 * 101 + 50], 0],
  ['profile slice 23 centre', m.profile(Float32Array.from(m.slice_grid('23', 3, 1, 2, 101).types), 101, 10)[0], 0],
  ['profile heatmap steps', m.profile(m.moire_field('heatmap', 9, 32), 32, 16).length, 16],
  ['volume 23 count at 2', `${solid.length},${m.volume_count(solid, 9, 2)}`, '729,540'],
  ['volume 23 faces at 2', m.volume_faces(solid, 9, 2)[0] / 36, 648],
  ['plane_frame diagonal width', JSON.parse(m.plane_frame([1, 1, 1], 0.5)).width.toFixed(4), '3.2660'],
  ['paint_span hexagon alpha', `${hexcut.rgba[3]},${hexcut.rgba[(32 * 64 + 32) * 4 + 3]}`, '0,255'],
  ['radial star centre', `${star.length},${star[32 * 64 + 32]}`, '4096,1'],
  ['harmonics square order', m.turns(m.harmonics(square, 8, 64, 8)), 4],
  ['petals 6 of order 4', m.petals(6, 4), 12],
  ['sheet bytes', m.sheet(star, 64, 'heat', 8, false).rgba.length, 16384],
  ['moire_field heatmap', m.moire_field('heatmap', 9, 32).length, 1024],
  ['diagonal 126 side', cut.side, 16],
  ['diagonal 126 support', cut.support.join(','), '15,30'],
  ['diagonal 126 flat', `${cut.min},${cut.max},${cut.constant}`, '81,81,true'],
  ['diagonal 126 central', m.diagonal_count('126', 2, 7, 2, 190), '2187'],
  ['diagonal svg circles', art.includes('<circle') ? art.split('<circle').length - 1 : 0, 54],
  ['diagonal 127 max', JSON.parse(m.diagonal_profile('127', 2, 4, 2)).max, '162'],
  ['race side', race.side(), 81],
  ['farey 5 nodes', JSON.parse(m.farey(5)).length, 11],
  ['totients 6', Array.from(m.totients(6)).join(','), '0,1,1,2,2,4,2'],
  ['dimension 127', m.dimension('127', 3, 2, 3).toFixed(4), '1.7712'],
];

let failed = 0;
for (const [label, got, want] of checks) {
  const ok = got === want;
  if (!ok) failed += 1;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${label.padEnd(26)} ${String(got)}${ok ? '' : ` (want ${String(want)})`}`);
}
console.log(failed ? `${failed} failed` : `${checks.length} checks green`);
process.exit(failed ? 1 : 0);
