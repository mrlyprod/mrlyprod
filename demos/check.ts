import init, * as m from './pkg/mrlyweb.js';

const bytes = await Bun.file(new URL('./pkg/mrlyweb_bg.wasm', import.meta.url)).arrayBuffer();
await init({ module_or_path: bytes });

const blinker = new Uint8Array(25);
for (const site of [7, 12, 17]) blinker[site] = 1;
const grid = m.two_grid('7', 3, 3, 0, 2);
const faces = m.three_faces('23', 3, 3, 2);
const race = new m.Race('127', 3, 4, 3, 300, 1);

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
