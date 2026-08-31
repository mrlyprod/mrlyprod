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
const stack = JSON.parse(m.farey_novelty(5));
const split = JSON.parse(m.slice_partition(3));
const shape = JSON.parse(m.volume_shape(7, 64));
const sieve = new m.Sieve(30);
let sweeps = 0;
while (!sieve.done()) {
  sieve.step();
  sweeps += 1;
}
const hundred = new m.Sieve(100);
hundred.finish();
const stones = JSON.parse(m.factor('360'));
const count = JSON.parse(m.prime_chart(10000, 400));
const trial = JSON.parse(m.carpet_witness(169));
const clear = JSON.parse(m.carpet_witness(197));
const euler = JSON.parse(m.spiral_polynomial('square', 201, 4, -2, 41));
const spoke = JSON.parse(m.spiral_polynomial('hex', 41, 3, 3, 1));
const sheet = m.spiral_pixels('square', 61, 4, -2, 41, 'prime', false, 180);
const pixel = (px: number, py: number) => Array.from(sheet.rgba.slice((py * 180 + px) * 4, (py * 180 + px) * 4 + 3)).join(' ');
const hit = JSON.parse(m.spiral_at('square', 61, 81.5, 92.5, 180));
const corner = JSON.parse(m.spiral_at('hex', 21, 195, 100, 200));
const centres = m.spiral_centers('square', 21, 420);
const gauss = JSON.parse(m.ring_census('gaussian', 2));
const flake = JSON.parse(m.ring_census('eisenstein', 2));
const window = m.ring_pixels('gaussian', 2, 'class', true, 100);
const dot = (px: number, py: number) => Array.from(window.rgba.slice((py * 100 + px) * 4, (py * 100 + px) * 4 + 3)).join(' ');
const r2 = m.ring_weights('gaussian', 25);
const struck = JSON.parse(m.ring_at('gaussian', 40, 403, 374, 768));
const mirror = JSON.parse(m.ring_at('eisenstein', 5, 140, 127, 220));

const carpet = JSON.parse(m.graph_census('flat', '7', 3, 1, 2, 'core'));
const knots = m.graph_nodes('flat', '7', 3, 1, 2, 'core');
const sponge = JSON.parse(m.graph_census('cube', '23', 3, 1, 2, 'core'));
const hexnet = JSON.parse(m.graph_census('hex', '23', 3, 1, 2, 'core'));
const rim = JSON.parse(m.graph_census('hex', '23', 3, 1, 2, 'edge'));
const ring = m.graph_nodes('flat', '15', 2, 1, 2, 'core');
const loop = m.graph_branches('flat', '15', 2, 1, 2, 'core');
const relax = new m.Layout(ring.subarray(2), loop, 2, 1);
const rest = relax.step(500);
const settled = relax.positions();
const gaps = Array.from({ length: loop.length / 2 }, (_, k) => Math.hypot(settled[2 * loop[2 * k]] - settled[2 * loop[2 * k + 1]], settled[2 * loop[2 * k] + 1] - settled[2 * loop[2 * k + 1] + 1]));
const roots = m.zeta_zeros(5);
const gammas = m.zeta_zeros(100);
const root = m.zeta_at(14.134725);
const walk = m.zeta_line(0, 50, 600);
const seam = m.zeta_seam(250, 500);
const stair = m.psi_stair(100);
const smooth = m.psi_formula(10, new Float64Array(0), 3);
const folded = m.psi_formula(100, gammas, 2);
const ledgerRows = m.ledger_build('closed', 4);
const octagon = JSON.parse(m.ledger_search('8, 21, 40, 65', '', 2, 2, 0, 25));
const known = JSON.parse(m.ledger_identify('6, 42, 306, 2250'));
const grown = JSON.parse(m.ledger_grow('convolved', 4, 100));
const hollow = JSON.parse(m.ledger_row('9', 2, 2, 'voids', 'side', 3, '500000'));
const gasket = m.ledger_profile('126', 3, 2, 2, 4);
const stack7 = JSON.parse(m.farey_novelty(7));
const cut5 = JSON.parse(m.diagonal_profile('126', 2, 5, 2));
const first = (terms: string) => { const f = JSON.parse(m.ledger_identify(terms)); return `${f[0].id} ${f[0].shift}`; };
const pinned = JSON.parse(m.census_window());
const heads = JSON.parse(m.census_walk(7692));
const closedTier = JSON.parse(m.census_report());
const writes16 = JSON.parse(m.census_writers(16, 0, 1));
const outside = JSON.parse(m.census_writers(1001, 0, 1));
const wordCodes = ['7', '14', '9'];
const wordSides = [3, 7, 5];
const wordBases = [2, 2, 2];
const wordCensus = JSON.parse(m.magic_census(wordCodes, wordSides, 2, wordBases));
const wordOnce = JSON.parse(m.magic_census(['7', '9'], [3, 5], 2, [2, 2]));
const wordTwice = JSON.parse(m.magic_census(['7', '9', '7', '9'], [3, 5, 3, 5], 2, [2, 2, 2, 2]));
const orderAhead = JSON.parse(m.magic_census(['3', '6'], [2, 2], 2, [2, 2]));
const orderBehind = JSON.parse(m.magic_census(['6', '3'], [2, 2], 2, [2, 2]));
const carpetLadder = JSON.parse(m.magic_census(['7', '7', '7', '7', '7'], [3, 5, 7, 9, 11], 2, [2, 2, 2, 2, 2]));
const carpetLaw = carpetLadder.letters.every((letter: { number: number; fill: string }) => Number(letter.fill) === letter.number ** 2 - ((letter.number - 1) / 2) ** 2);
const ladder = JSON.parse(m.magic_staircase(5));
const morse = JSON.parse(m.magic_rates(['3', '7'], [2, 2], [2, 2], 'thue-morse', 64));
const evenly = JSON.parse(m.magic_rates(['3', '7'], [2, 2], [2, 2], 'periodic', 64));
const collideAhead = m.magic_grid(['9', '273'], [2, 3], [2, 3]);
const collideBehind = m.magic_grid(['273', '9'], [3, 2], [3, 2]);
const collideSame = collideAhead.width === collideBehind.width && collideAhead.types.every((byte: number, at: number) => byte === collideBehind.types[at]);
const mengerWord = JSON.parse(m.magic_census(['23', '23', '23'], [3, 3, 3], 3, [2, 2, 2]));

const shapes2 = JSON.parse(m.crop_shapes(2));
const cropTouch = m.crop_grid('7', 3, 2, 2, 'ball', 1, 2, false, 'touching');
const cropIn = m.crop_grid('7', 3, 2, 2, 'ball', 1, 2, false, 'inside');
const cropFine = m.crop_grid('7', 3, 1, 2, 'ball', 1, 2, false, 'refined1');
const cropCubes = m.crop_cells('23', 3, 1, 2, 'ball', 1, 2, false, 'touching');
const cropMesh = m.crop_faces('23', 3, 1, 2, 'ball', 1, 2, false, 'touching');
const cropTally = JSON.parse(m.crop_census('7', 3, 2, 2, 2, 'ball', 1, 2, false));
const cropSweep = JSON.parse(m.crop_series('7', 3, 2, 2, 2, 'ball', 1, 2, false, 'level', 2));
const cropRadii = JSON.parse(m.crop_series('7', 3, 1, 2, 2, 'ball', 1, 2, false, 'radius', 4));
const cropArt = m.crop_svg('7', 3, 1, 2, 'ball', 1, 2, false, 4);
const cropHole = m.crop_svg('7', 3, 1, 2, 'diamond', 1, 2, true, 4);
const cropField = m.field_crop(square, 8, 2, 'ball', 1, 2, false);

const checks: [string, unknown, unknown][] = [
  ['two_grid 7 side', grid.width, 27],
  ['crop_shapes 2', shapes2.join(','), 'ball,box,diamond,triangle,octagon'],
  ['crop_grid touching side', cropTouch.width, 9],
  ['crop_grid orders fills', cropIn.types.reduce((a: number, b: number) => a + b, 0) <= cropTouch.types.reduce((a: number, b: number) => a + b, 0), true],
  ['crop_grid refined side', cropFine.width, 9],
  ['crop_cells sponge ball', cropCubes.length / 3, 20],
  ['crop_faces sponge ball', `${cropMesh[0] / 36},${cropMesh.length}`, `72,${2 + cropMesh[0]}`],
  ['crop_census carpet cells', cropTally.cells_out + cropTally.cells_cut + cropTally.cells_in, 81],
  ['crop_census carpet fills', cropTally.filled_out + cropTally.filled_cut + cropTally.filled_in, 64],
  ['crop_census exposed', `${cropTally.exposed_before},${Number(cropTally.exposed_after) > 0}`, `${JSON.parse(m.two_census('7', 3, 2, 0, 2)).perimeter},true`],
  ['crop_series level rows', cropSweep.map((row: { x: number }) => row.x).join(','), '0,1,2'],
  ['crop_series radius rows', `${cropRadii.length},${cropRadii[3].x}`, '4,1'],
  ['crop_svg clip circle', `${cropArt.includes('<clipPath')},${cropArt.includes('<circle')}`, 'true,true'],
  ['crop_svg anti mask', `${cropHole.includes('<mask')},${cropHole.includes('<polygon')}`, 'true,true'],
  ['crop_field kept centres', cropField.reduce((a: number, v: number) => a + (Number.isNaN(v) ? 0 : 1), 0), 52],
  ['two_grid 7 fills', grid.types.reduce((a, b) => a + b, 0), 512],
  ['fill sponge level 3', m.fills('23', 3, 3, 3, 2), '8000'],
  ['void sponge level 3', m.voids('23', 3, 3, 3, 2), '11683'],
  ['universe counts D=1..4', m.counting_sequence(4).join(','), '3,6,22,402'],
  ['base 3 counts D=1..2', m.baseq_sequence(3, 2).join(','), '4,26'],
  ['classes_sequence 4', m.classes_sequence(4).join(','), '4,12,64,700'],
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
  ['volume 23 surface at 2', m.volume_surface(solid, 9, 2), 648],
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
  ['diagonal 126 central pair', cut.central.join(','), '22,23'],
  ['race side', race.side(), 81],
  ['farey 5 nodes', JSON.parse(m.farey(5)).length, 11],
  ['totients 6', Array.from(m.totients(6)).join(','), '0,1,1,2,2,4,2'],
  ['dimension 127', m.dimension('127', 3, 2, 3).toFixed(4), '1.7712'],
  ['random_code 3 2 seed 7', m.random_code(3, 2, 7), '160'],
  ['random_codes 3 2 seed 7', m.random_codes(3, 2, 7, 3).join(','), '160,134,72'],
  ['random_between seed 7', Array.from(m.random_between(7, [0, 0, 1], [3, 1800, 36])).join(','), '0,1023,17'],
  ['level_cap side and cells', `${m.level_cap(3, 1, 128)},${m.level_cap(2, 1, 512)},${m.level_cap(3, 3, 60000)}`, '4,9,3'],
  ['fill_cap triangle', m.fill_cap('7', 2, 2, 2, 1100), 6],
  ['grid_total 3 2 4', m.grid_total(3, 2, 4), '6561'],
  ['odd_scales 9', Array.from(m.odd_scales(9)).join(','), '1,3,5,7,9'],
  ['farey_novelty 5', `${stack.lit},${stack.novel},${stack.match},${stack.primes.join(' ')}`, '11,11,true,2 3 5'],
  ['slice_partition 3', `${split.carpet},${split.net},${split.exact}`, '42,12,true'],
  ['volume_shape 7 64', `${shape.layers},${shape.voxels}`, '4,262144'],
  ['radial_share square', m.radial_share(m.harmonics(square, 8, 64, 8)).toFixed(1), '95.3'],
  ['full_turn 6', m.full_turn(6), 60],
  ['frame_step 900 at 60', m.frame_step(900, 60), 90],
  ['diagonal_digits 126 at 20', m.diagonal_digits('126', 2, 4, 2, 20), '101'],
  ['diagonal_total 126 both', m.diagonal_total('126', 2, 3, 2, [10, 11]), '54'],
  ['sieve 30 sweeps and count', `${sweeps},${sieve.count()}`, '3,10'],
  ['sieve 100 count', hundred.count(), 25],
  ['sieve 150 grid', `${hundred.grid(15).width},${new m.Sieve(150).grid(15).height}`, '15,10'],
  ['factor 360', stones.factors.map(([p, e]: number[]) => `${p}^${e}`).join(' '), '2^3 3^2 5^1'],
  ['factor 6 rectangles', JSON.parse(m.factor('6')).rectangles.map((r: number[]) => r.join('x')).join(' '), '1x6 2x3'],
  ['prime_chart pi 10000', count.pi.at(-1), 1229],
  ['prime_chart pi 100000', JSON.parse(m.prime_chart(100000, 100)).pi.at(-1), 9592],
  ['carpet_witness 169', `${trial.max.toFixed(7)},${trial.at}`, '0.0517383,13'],
  ['carpet_witness 197', `${clear.max},${clear.prime},${clear.row.length}`, '0,true,97'],
  ['spiral_xy square 10 25', `${m.spiral_xy('square', 10).join(',')} ${m.spiral_xy('square', 25).join(',')}`, '2,-1,2 2,-2,2'],
  ['spiral_xy hex 8 19 20', [8, 19, 20].map((n) => m.spiral_xy('hex', n).join(',')).join(' '), '1,1,2 0,2,2 1,2,3'],
  ['spiral_polynomial euler', `${euler.top},${euler.primes},${euler.count},${euler.hits},${euler.streak}`, '40401,4236,101,80,21'],
  ['spiral_polynomial euler shares', `${euler.density.toFixed(6)} ${euler.share.toFixed(6)}`, '0.104849 0.792079'],
  ['spiral_polynomial euler cells', `${euler.values[20]} ${euler.cells[100].join(',')}`, '1601 60,100'],
  ['spiral_polynomial hex spoke', `${spoke.top} ${spoke.cells[20].join(',')}`, '1261 0,20'],
  ['spiral_pixels square 61', `${sheet.width}x${sheet.height} ${pixel(90, 90)} ${pixel(92, 90)} ${pixel(81, 92)}`, '180x180 7 9 11 255 209 102 255 138 92'],
  ['spiral_at square 61', `${hit.n},${hit.prime},${hit.factors.map((f: number[]) => f.join('^')).join(' ')}`, '41,true,41^1'],
  ['spiral_at hex corner', `${corner.n},${corner.x},${corner.y}`, '281,10,0'],
  ['spiral_centers square 21', `${centres.length} ${centres[0]},${centres[1]} ${centres[2]},${centres[3]}`, '882 210,210 230,210'],
  ['prime_from 90', m.prime_from(90), 97],
  ['ring_census gaussian 2', `${gauss.points},${gauss.primes},${gauss.split},${gauss.inert},${gauss.ramified},${gauss.units},${gauss.symmetry}`, '25,12,8,0,4,4,8'],
  ['ring_census gaussian 3', `${JSON.parse(m.ring_census('gaussian', 3)).primes}`, '24'],
  ['ring_census eisenstein 2', `${flake.points},${flake.primes},${flake.split},${flake.inert},${flake.ramified},${flake.units},${flake.symmetry}`, '19,12,0,6,6,6,12'],
  ['ring_weights gaussian 25', `${r2[3]},${r2[5]},${r2[25]}`, '0,8,12'],
  ['ring_weights eisenstein 7', Array.from(m.ring_weights('eisenstein', 7)).join(','), '1,6,0,6,6,0,0,12'],
  ['ring_peak gaussian eisenstein 60', `${Array.from(m.ring_peak('gaussian', 60)).join(',')} ${Array.from(m.ring_peak('eisenstein', 60)).join(',')}`, '25,12 49,18'],
  ['ring_fates gaussian 7', Array.from(m.ring_fates('gaussian', 7)).join(','), '0,0,3,2,0,1,0,2'],
  ['ring_fates eisenstein 7', Array.from(m.ring_fates('eisenstein', 7)).join(','), '0,0,2,3,0,2,0,1'],
  ['ring_pixels gaussian 2', `${window.width}x${window.height} ${dot(70, 30)} ${dot(90, 30)} ${dot(90, 50)} ${dot(50, 50)}`, '100x100 255 122 182 92 200 255 31 38 46 7 9 11'],
  ['ring_at gaussian 40', `${struck.a},${struck.b},${struck.norm},${struck.class},${struck.associates.length},${struck.conjugate[1]}`, '2,1,5,split,4,-1'],
  ['ring_at eisenstein 5', `${mirror.a},${mirror.b},${mirror.norm},${mirror.class},${mirror.associates.length},${mirror.conjugate.slice(0, 2).join(',')}`, '1,-1,3,ramified,6,2,1'],
  ['graph_census carpet 7', `${carpet.nodes},${carpet.branches},${carpet.components}`, '8,8,1'],
  ['graph_nodes carpet 7', `${knots[0]},${knots[1]},${knots.length}`, '2,8,18'],
  ['graph_branches carpet 7', m.graph_branches('flat', '7', 3, 1, 2, 'core').length, 16],
  ['graph_roles carpet 7', Array.from(m.graph_roles('flat', '7', 3, 1, 2, 'core')).join(','), '2,2,2,2,2,2,2,2'],
  ['graph_nodes sponge 23', `${m.graph_nodes('cube', '23', 3, 1, 2, 'core').slice(0, 2).join(',')}`, '3,20'],
  ['graph_census sponge 23', `${sponge.nodes},${sponge.branches},${sponge.junctions},${sponge.euler}`, '20,24,8,-4'],
  ['graph_branches unit cube', m.graph_branches('cube', '255', 1, 1, 2, 'edge').length / 2, 12],
  ['graph_census hex 23 core', `${hexnet.nodes},${hexnet.branches},${(hexnet.length / hexnet.branches).toFixed(4)}`, '42,48,0.5774'],
  ['graph_census hex 23 edge', `${rim.nodes},${rim.length}`, '36,78'],
  ['graph_size carpet 495', m.graph_size('flat', '495', 3, 2, 3, 'core'), '64'],
  ['graph_size sponge tunnel', m.graph_size('cube', '23', 3, 2, 2, 'tunnel'), '329'],
  ['graph_cap 7 23 hex', `${m.graph_cap('flat', '7', 3, 2, 'core', 20000)},${m.graph_cap('cube', '23', 3, 2, 'core', 2000)},${m.graph_cap('hex', '23', 3, 2, 'edge', 2000)}`, '4,2,2'],
  ['layout ring rests', `${rest < 1e-3} ${Math.max(...gaps) - Math.min(...gaps) < 1e-3} ${relax.ticks()}`, 'true true 500'],
  ['zeta_zeros 5', Array.from(roots, (z) => z.toFixed(6)).join(' '), '14.134725 21.022040 25.010858 30.424876 32.935062'],
  ['zeta_zeros 100 last', gammas[99].toFixed(6), '236.524230'],
  ['zeta_count 100 200', `${m.zeta_count(100)} ${m.zeta_count(200)}`, '29 79'],
  ['zeta_at first zero', `${Math.hypot(root[0], root[1]) < 1e-5} ${Math.abs(root[2]) < 1e-5}`, 'true true'],
  ['zeta_at 0', `${m.zeta_at(0)[0].toFixed(7)} ${m.zeta_at(0)[3]}`, '-1.4603545 0'],
  ['zeta_line 0 50 600', `${walk.length} ${walk[0]} ${walk[2400]} ${walk[1].toFixed(7)}`, '2404 0 50 -1.4603545'],
  ['zeta_seam 250', `${seam[0]} ${seam[1] < 5e-5}`, '20 true'],
  ['psi_stair 10 100', `${stair[9].toFixed(4)} ${stair[99].toFixed(4)}`, '7.8320 94.0453'],
  ['psi_formula 10 no zeros', `${smooth.length} ${smooth[5].toFixed(4)}`, '6 8.1671'],
  ['psi_formula 100 zeros', `${Math.abs(folded[3] - stair[99]) < 1} ${Math.abs(m.psi_gap(100, gammas) - (stair[99] - folded[3])) < 1e-12}`, 'true true'],
  ['ledger_measures', m.ledger_measures().length, 12],
  ['ledger_designs 2 2', m.ledger_designs(2, 2).join(','), '0,1,3,6,7,15'],
  ['ledger_designs 2 3', m.ledger_designs(2, 3).length, 26],
  ['ledger_terms carpet fills', m.ledger_terms('7', 2, 2, 'fills', 'level', 3, '500000').join(','), '8,64,512'],
  ['ledger_terms sponge surface', m.ledger_terms('23', 3, 2, 'surface', 'level', 3, '500000').join(','), '72,1056,18048'],
  ['ledger_terms carpet side', m.ledger_terms('7', 2, 2, 'fills', 'side', 4, '500000').join(','), '8,21,40,65'],
  ['ledger_terms tree side', m.ledger_terms('3', 2, 2, 'fills', 'side', 3, '500000').join(','), '6,15,28'],
  ['ledger_terms sponge euler capped', m.ledger_terms('23', 3, 2, 'euler', 'level', 8, '1000').join(','), '-4,-80'],
  ['ledger_identify slice', `${known[0].id} ${known[0].shift}`, 'A299916 1'],
  ['ledger_closed carpet side', m.ledger_closed('7', 2, 2, 'fills', 'side'), '3k^2 - 2k'],
  ['ledger_closed sponge surface', m.ledger_closed('23', 3, 2, 'surface', 'level'), 'a(L) = 28 a(L-1) - 160 a(L-2)'],
  ['ledger_records', JSON.parse(m.ledger_records()).length, 60],
  ['ledger_build closed', ledgerRows, 7692],
  ['ledger_search octagonal', `${octagon.total} ${octagon.rows[0].name} ${octagon.rows[0].oeis} ${octagon.rows[0].shift} ${octagon.rows[0].tag} ${octagon.rows[0].closed}`, '1 mrly_bang_d2_7.fills.side A000567 0 Proved 3k^2 - 2k'],
  ['ledger_search surfaces', JSON.parse(m.ledger_search('', 'surface', 3, 2, 0, 5)).total, 44],
  ['ledger_grow convolved 100', `${grown.rows} ${grown.done} ${grown.total}`, '7792 100 5044'],
  ['ledger_row void side', `${hollow.name} ${hollow.terms.join(',')} ${hollow.closed} ${hollow.number}`, 'mrly_bang_d2_9.voids.side 4,12,24 2k^2 - 2k 3'],
  ['ledger_profile gasket', `${gasket.length} ${gasket.slice(15, 31).every((c: string) => c === '81')}`, '46 true'],
  ['ledger_profile strip', m.ledger_profile('1', 1, 2, 3, 2).join(''), '101000101'],
  ['farey_novelty 7', `${stack7.lit} ${stack7.novel}`, '19 19'],
  ['diagonal_profile gasket 5', `${cut5.max} ${cut5.constant}`, '243 true'],
  ['slice_census vertices 9', JSON.parse(m.slice_census('23', 9, 1, 2)).vertices, 271],
  ['slice_census unit hexagon', JSON.parse(m.slice_census('23', 1, 1, 2)).fills, 6],
  ['two_census void side 5', JSON.parse(m.two_census('9', 5, 1, 0, 2)).fills, 13],
  ['baseq_sequence 5 2', m.baseq_sequence(5, 2).join(','), '8,172112'],
  ['ledger_identify gasket', first('3, 9, 27, 81'), 'A000244 1'],
  ['ledger_identify farey', first('2, 3, 5, 7, 11, 13'), 'A005728 1'],
  ['ledger_identify vertices', first('7, 37, 91, 169'), 'A154105 0'],
  ['ledger_identify classes', first('4, 12, 64, 700'), 'A129824 1'],
  ['ledger_identify tile', first('20, 81, 208'), 'A103532 1'],
  ['census_window registry', `${pinned.registry} ${pinned.cap} ${pinned.cells} ${pinned.ceiling} ${pinned.depths.join('/')}`, '18066 48 100000 1000 8/16/32/48'],
  ['census_window tiers', pinned.tiers.map((t: { tier: string; keys: number }) => `${t.tier} ${t.keys}`).join(', '), 'closed 7692, convolved 5044, side 2665, level 2665'],
  ['census_walk closed heads', `${heads.depth} ${heads.done} ${heads.total} ${heads.never} ${heads.once} ${heads.multiple}`, '8 7692 18066 396 102 502'],
  ['census_report closed heads', `${closedTier.written} ${closedTier.first_miss} ${closedTier.incidences} ${closedTier.low}`, '604 83 30865 452'],
  ['census_report stops', `${closedTier.ceiling_stopped} ${closedTier.cap_stopped} ${closedTier.blank}`, '5048 2644 54'],
  ['census_counts window', `${m.census_counts().length} ${m.census_counts()[15]}`, '1000 633'],
  ['census_writers 16 heads', `${writes16.rows} ${writes16.shown[0].name} ${writes16.shown[0].closed} ${writes16.shown[0].index}`, '633 mrly_bang_d1_1.fills.level 2^L 3'],
  ['census_writers outside', `${outside.inside} ${outside.rows}`, 'false 0'],
  ['census_champions heads', JSON.parse(m.census_champions(2)).map((c: { value: number; rows: number }) => `${c.value} at ${c.rows}`).join(', '), '16 at 633, 12 at 579'],
  ['census_misses heads', JSON.parse(m.census_misses(3)).join(','), '83,86,107'],
  ['magic side', wordCensus.side, '105'],
  ['magic fill', wordCensus.fill, '3432'],
  ['magic dim', wordCensus.dimension.toFixed(9), '1.749241044'],
  ['magic pieces', `${wordCensus.components} ${wordCensus.counted}`, '2496 drawn'],
  ['word count agrees', m.word_count(wordCodes, wordSides, 2, wordBases), wordCensus.fill],
  ['word profile heights', m.word_profile(wordCodes, wordSides, 2, wordBases).length, 209],
  ['word member 25', m.word_member(wordCodes, wordSides, 2, wordBases, '25'), true],
  ['word members head', m.word_members(['3', '6'], [2, 2], 2, [2, 2]).join(','), '1,2,5,6'],
  ['block reduction side', wordTwice.side, (BigInt(wordOnce.side) ** 2n).toString()],
  ['block reduction fill', wordTwice.fill, (BigInt(wordOnce.fill) ** 2n).toString()],
  ['block reduction dim', wordTwice.dimension === wordOnce.dimension, true],
  ['block reduction flag', `${wordOnce.periodic} ${wordTwice.periodic}`, 'false true'],
  ['order fill', `${orderAhead.fill} ${orderBehind.fill}`, '4 4'],
  ['order components', `${orderAhead.components} ${orderBehind.components} ${orderAhead.counted}`, '4 2 closed'],
  ['carpet fill law', carpetLaw, true],
  ['staircase dim 1', ladder.rows[0].dimension.toFixed(9), '1.892789261'],
  ['staircase dim 2', ladder.rows[1].dimension.toFixed(9), '1.892315261'],
  ['staircase dips', ladder.rows[1].dimension < ladder.rows[0].dimension, true],
  ['staircase constant', ladder.constant.toFixed(9), ladder.rows[0].dimension.toFixed(9)],
  ['thue exponent limit', morse.limit.toFixed(15), '1.292481250360578'],
  ['thue fill rate exact', morse.rows[63][1], morse.limit],
  ['thue rate climbs', `${morse.rows[63][0] > morse.rows[31][0]} ${morse.rows[63][0] < morse.limit}`, 'true true'],
  ['thue order blind', `${evenly.limit === morse.limit} ${morse.phi}`, 'true 0'],
  ['thue alphabet', morse.alphabet, true],
  ['magic cap 2d', m.magic_cap([3, 7, 5, 3], 2, 243), 3],
  ['magic cap 3d', m.magic_cap([3, 3, 3, 3, 3], 3, 128), 4],
  ['menger word cubes', `${mengerWord.fill} ${m.magic_cells(['23', '23', '23'], [3, 3, 3], [2, 2, 2]).length / 3}`, '8000 8000'],
  ['menger word surface', m.magic_surface(['23', '23', '23'], [3, 3, 3], [2, 2, 2]), '18048'],
  ['menger word constant', mengerWord.constant, true],
  ['magic name', m.magic_name(wordCodes, wordSides), 'mrly_word_d2_c7n3_c14n7_c9n5'],
  ['magic name round trip', JSON.parse(m.magic_parse(m.magic_name(wordCodes, wordSides))).codes.join(','), wordCodes.join(',')],
  ['code collision same tile', collideSame, true],
  ['code collision side', `${collideAhead.width} ${collideAhead.types.reduce((a: number, b: number) => a + b, 0)}`, '6 6'],
];

if (Bun.argv.includes('--deep')) {
  const fold = (a: { width: number; types: Uint8Array }, b: { width: number; types: Uint8Array }) => {
    const width = a.width * b.width;
    const types = new Uint8Array(width * width);
    for (let r = 0; r < width; r += 1) {
      for (let c = 0; c < width; c += 1) {
        const outer = a.types[Math.floor(r / b.width) * a.width + Math.floor(c / b.width)];
        types[r * width + c] = outer && b.types[(r % b.width) * b.width + (c % b.width)] ? 1 : 0;
      }
    }
    return { width, types };
  };
  const kron = wordCodes
    .map((code, at) => m.two_grid(code, wordSides[at], 1, 0, wordBases[at]))
    .reduce((left, right) => fold(left, right));
  const drawn = m.magic_grid(wordCodes, wordSides, wordBases);
  checks.push(
    ['magic kronecker fold', `${drawn.width} ${drawn.types.every((byte: number, at: number) => byte === kron.types[at])}`, `${kron.width} true`],
  );
  console.log('walking the whole registry to the pinned 48-term window, minutes not seconds');
  for (;;) {
    const state = JSON.parse(m.census_walk(500));
    if (state.complete) break;
  }
  const deep = JSON.parse(m.census_report());
  const champion = JSON.parse(m.census_writers(16, 0, 1));
  checks.push(
    ['census pinned window', `${deep.depth} ${deep.rows} ${deep.never} ${deep.once} ${deep.multiple}`, '48 18066 41 31 928'],
    ['census pinned written', `${deep.written} ${deep.share.toFixed(4)} ${deep.first_miss} ${deep.run}`, '959 0.9590 269 268'],
    ['census pinned tally', `${deep.incidences} ${deep.low} ${deep.bands[2].missed} ${deep.bands[2].density.toFixed(6)}`, '193419 29144 41 0.045556'],
    ['census pinned ladder', deep.depths.map((row: { written: number }) => row.written).join(','), '783,929,955,959'],
    ['census pinned champions', JSON.parse(m.census_champions(5)).map((c: { value: number; rows: number }) => `${c.value} at ${c.rows}`).join(', '), '16 at 2858, 9 at 2811, 4 at 2559, 12 at 2303, 36 at 2270'],
    ['census pinned misses', JSON.parse(m.census_misses(6)).join(','), '269,362,422,443,446,487'],
    ['census pinned writers', `${champion.rows} ${champion.tiers.map((t: { rows: number }) => t.rows).join('/')} ${JSON.parse(m.census_writers(269, 0, 1)).rows}`, '2858 666/1529/530/133 0'],
  );
}

let failed = 0;
for (const [label, got, want] of checks) {
  const ok = got === want;
  if (!ok) failed += 1;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${label.padEnd(26)} ${String(got)}${ok ? '' : ` (want ${String(want)})`}`);
}
console.log(failed ? `${failed} failed` : `${checks.length} checks green`);
process.exit(failed ? 1 : 0);
