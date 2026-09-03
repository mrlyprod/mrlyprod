import init, * as m from './pkg/mrlyweb.js';

const bytes = await Bun.file(new URL('./pkg/mrlyweb_bg.wasm', import.meta.url)).arrayBuffer();
await init({ module_or_path: bytes });

const blinker = new Uint8Array(25);
for (const site of [7, 12, 17]) blinker[site] = 1;
const grid = m.two_grid('7', 3, 3, 0, 2);
const sum = (types: Uint8Array) => types.reduce((a: number, b: number) => a + b, 0);
const line = Uint8Array.from([0, 1, 1, 0, 1, 0, 0]);
const cone110 = m.eca_history(Uint8Array.from([0, 0, 1, 0, 0]), 110, 3, false);
const card110 = JSON.parse(m.eca_card(110));
const card90 = JSON.parse(m.eca_card(90));
const maskMoore = m.life_mask(2, '7', 3, 1);
const maskDeep = m.life_mask(2, '7', 3, 2);
const maskDiag = m.life_mask(2, '9', 3, 1);
const maskLine = m.life_mask(1, '1', 3, 1);
const faces = m.three_faces('23', 3, 3, 2);
const race = new m.Race('127', 3, 4, 3, 300, 1);
const cut = JSON.parse(m.diagonal_profile('126', 2, 4, 2));
const art = m.diagonal_svg('126', 2, 3, 2, [10, 11], 4);
const slice = JSON.parse(m.slice_census('23', 3, 1, 2));
const deep = JSON.parse(m.slice_census('23', 3, 2, 2));
const series = JSON.parse(m.slice_series('23', 6));
const walsh = JSON.parse(m.walsh_spectrum('23', 6));
const skew = JSON.parse(m.walsh_spectrum('11', 6));
const inked = (read: { law: { fills: number }[] }) => read.law.map((row) => row.fills).join(',');
const counted = (code: string) => Array.from({ length: 6 }, (_, i) => JSON.parse(m.slice_census(code, 2 * i + 1, 1, 2)).fills).join(',');
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
const terms = Array.from({ length: 8 }, (_, i) => JSON.parse(m.visible_read(i + 1, 2)).lit).join(',');
const litWindow = JSON.parse(m.visible_read(100, 2));
const cube = JSON.parse(m.visible_read(1000, 3));
const quartic = JSON.parse(m.visible_read(1000, 4));
const lattice = m.visible_pixels(100, 200, true);
const litDots = lattice.rgba.reduce((count: number, byte: number, at: number) => count + (at % 4 === 0 && byte === 92 && lattice.rgba[at + 1] === 200 && lattice.rgba[at + 2] === 255 ? 1 : 0), 0);
const approach = m.visible_walk(100, 2, 8);
const forms = JSON.parse(m.formulas_read(1000));
const halved = JSON.parse(m.formulas_read(500));
const basel = m.formulas_walk('basel', 1000, 4);
const comet = m.formulas_walk('goldbach', 500, 2);
const sparks = ['wallis', 'leibniz', 'basel', 'gamma', 'e', 'primes', 'goldbach', 'mertens'].map((kind) => m.formulas_walk(kind, 400, 160).length).join(',');
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

const morseWord = JSON.parse(m.morse_word(64));
const morseGallery = JSON.parse(m.morse_gallery(6));
const morseParity = m.morse_lift('parity', 6);
const morseAnd = m.morse_lift('and', 6);
const morseXor = m.morse_lift('xor', 6);
const morseSame = (a: Uint8Array, b: Uint8Array) => a.length === b.length && a.every((byte: number, at: number) => byte === b[at]);
const morseSign = JSON.parse(m.morse_filter('9', 2, 2, 3, 'sign'));
const morseFlat = JSON.parse(m.morse_filter('7', 2, 2, 3, 'design'));
const morseWide = JSON.parse(m.morse_filter('495', 3, 3, 2, 'design'));

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

const tiled = (dimension: number, code: string, number: number, level: number, base: number, projection: string, reps: number[], crop: boolean) =>
  JSON.parse(m.tile_census(code, number, level, base, dimension, projection, reps, crop));
const tileWide = tiled(2, '495', 3, 2, 3, '', [5, 5], false);
const tileTall = tiled(2, '495', 3, 2, 3, '', [3, 9], false);
const tileBlock = tiled(3, '23', 3, 1, 2, '', [5, 5, 5], false);
const tileSlab = tiled(3, '23', 3, 1, 2, '', [3, 9, 3], false);
const tileMesh = tiled(6, '23', 3, 1, 2, 'cut', [5, 5], false);
const tileStrip = tiled(6, '23', 3, 1, 2, 'cut', [3, 9], false);
const tileTrim = tiled(6, '23', 3, 1, 2, 'cut', [5, 5], true);
const tileNarrow = tiled(6, '23', 3, 1, 2, 'cut', [3, 9], true);
const tileSheet = m.tile_grid('495', 3, 2, 3, 5, 5);
const tileArt = m.tile_svg('23', 3, 1, 2, 'cut', 5, 5, true, 6);
const towerCut = JSON.parse(m.magic_hex_census(['23', '23'], [3, 3], [2, 2], 'cut'));
const towerIso = JSON.parse(m.magic_hex_census(['23', '23'], [3, 3], [2, 2], 'iso'));

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
  ['eca_next 110', Array.from(m.eca_next(Uint8Array.from([0, 0, 1, 0, 0]), 110, false)).join(','), '0,1,1,0,0'],
  ['eca_history 110 rows', `${cone110.width},${cone110.height}`, '5,4'],
  ['eca_seed 110 fill', sum(m.eca_seed(110, 31).types), 326],
  ['eca_seed 90 fill', sum(m.eca_seed(90, 8).types), 29],
  ['eca_card 110 class', `${card110.b3_rep},${card110.wolfram_rep},${card110.npn_rep},${card110.genus}`, '61,110,25,comp'],
  ['eca_card 90 totalistic', `${card90.outer_totalistic.birth},${card90.outer_totalistic.survive},${card90.surjective}`, '1,1,true'],
  ['eca_soup seeded', String(m.eca_soup(64, 0.5, 7).join(',') === m.eca_soup(64, 0.5, 7).join(',')), 'true'],
  ['life_mask 7 level 2', `${maskDeep.width},${sum(maskDeep.types)}`, '9,64'],
  ['life_mask_index moore', m.life_mask_index(maskDeep.types, 9, 9), 1],
  ['life_mask_index diagonal', m.life_mask_index(maskDiag.types, 3, 3), 2],
  ['life_next_masked line', Array.from(m.life_next_masked(line, 7, 1, [1], [0, 1], maskLine.types, 3, 1, false)).join(','), '1,1,1,0,1,1,0'],
  ['life_run_masked blinker', JSON.parse(m.life_run_masked(blinker, 5, 5, [3], [2, 3], maskMoore.types, 3, 3, false, 16)).loop, 2],
  ['moire heatmap bytes', m.moire('heatmap', 9, 32, 'fire', 64, false).rgba.length, 4096],
  ['hex_svg polygons', m.hex_svg('23', 3, 1, 2, 'iso', 10).includes('<polygon'), true],
  ['slice_census 23 mesh', `${slice.triangles},${slice.fills},${slice.vertices},${slice.euler}`, '54,42,37,1'],
  ['slice_census 23 pieces', `${slice.components},${slice.holes}`, '1,1'],
  ['slice_census 23 closed', slice.closed.vertices, '37'],
  ['slice_census 232 fills', JSON.parse(m.slice_census('232', 3, 1, 2)).fills, 12],
  ['slice_census 23 level 2', `${deep.fills},${deep.holes}`, '306,7'],
  ['slice_series 23 components', series.map((row: {components: number}) => row.components).join(','), '1,1,7,1,19,1'],
  ['slice_series 23 holes', series.map((row: {holes: number}) => row.holes).join(','), '0,1,0,7,0,19'],
  ['walsh_spectrum 23 spectrum', walsh.spectrum.join(','), '0,-4,-4,0,-4,0,0,4'],
  ['walsh_spectrum 23 levels', walsh.levels.map((row: {sixteenths: number}) => row.sixteenths).join(','), '8,12,0,-4'],
  ['walsh_spectrum 23 weights', walsh.weights.join(','), '1,3,0,0'],
  ['walsh_spectrum 23 ink law', inked(walsh), '6,42,72,204,210,486'],
  ['walsh_spectrum 23 sign', walsh.law.map((row: {s: number}) => row.s).join(','), '-1,1,-1,1,-1,1'],
  ['walsh_spectrum 11 spectrum', skew.spectrum.join(','), '2,2,-2,-2,-6,2,-2,-2'],
  ['walsh_spectrum 11 levels', skew.levels.map((row: {sixteenths: number}) => row.sixteenths).join(','), '6,6,2,2'],
  ['walsh_spectrum 11 weights', skew.weights.join(','), '1,1,1,0'],
  ['walsh_spectrum 11 ink law', inked(skew), '6,20,76,100,230,240'],
  ['walsh_spectrum law counts the mesh', `${inked(walsh)} ${inked(skew)}`, `${counted('23')} ${counted('11')}`],
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
  ['visible A018805 1..8', terms, '1,3,7,11,19,23,35,43'],
  ['visible window 100', `${litWindow.lit} ${litWindow.total} ${litWindow.density.toFixed(10)}`, '6087 10000 0.6087000000'],
  ['visible limit 6/pi^2', litWindow.limit.toFixed(9), '0.607927102'],
  ['visible recovers pi 100', `${litWindow.name} ${litWindow.constant.toFixed(8)}`, 'pi 3.13959750'],
  ['visible cube is apery', `${cube.name} ${cube.lit} ${cube.constant.toFixed(8)}`, 'zeta(3) 832046137 1.20185643'],
  ['visible quartic is pi', `${quartic.name} ${quartic.constant.toFixed(8)}`, 'pi 3.14154967'],
  ['visible pixels are the count', `${lattice.width} ${litDots}`, `200 ${4 * 6087}`],
  ['visible walk lands on n', `${approach.length} ${approach[14]} ${approach[15].toFixed(8)}`, '16 100 3.13959750'],
  ['formulas constants', `${forms.constants.pi.toFixed(9)} ${forms.constants.e.toFixed(9)} ${forms.constants.gamma.toFixed(9)}`, '3.141592654 2.718281828 0.577215665'],
  ['formulas partials 1000', `${forms.cards.wallis.value.toFixed(9)} ${forms.cards.leibniz.value.toFixed(9)} ${forms.cards.basel.value.toFixed(9)}`, '1.570403873 0.785148163 1.643934567'],
  ['formulas gamma and e 1000', `${forms.cards.gamma.value.toFixed(9)} ${forms.cards.e.value.toFixed(9)}`, '0.577715582 2.716923932'],
  ['formulas prime count 1000', `${forms.cards.primes.value} ${forms.cards.primes.li.toFixed(6)} ${forms.cards.primes.ratio.toFixed(6)}`, '168 177.609658 144.764827'],
  ['formulas goldbach 2000', `${forms.cards.goldbach.even} ${forms.cards.goldbach.value} ${forms.cards.goldbach.floor}`, '2000 37 1'],
  ['formulas goldbach 1000', `${halved.cards.goldbach.even} ${halved.cards.goldbach.value} ${halved.cards.primes.value}`, '1000 28 95'],
  ['formulas mertens 1000 500', `${forms.cards.mertens.value} ${halved.cards.mertens.value}`, '2 -6'],
  ['formulas basel walk', `${basel.length} ${basel[0]} ${basel[1].toFixed(6)} ${basel[9]} ${basel[10].toFixed(6)}`, '12 2 1.250000 1000 1.643935'],
  ['formulas goldbach walk', `${comet[0]} ${comet[3]} ${comet[4]} ${comet[5].toFixed(9)}`, '4 1000 28 0.035714286'],
  ['formulas walks eight kinds', sparks, '480,480,480,480,480,480,480,480'],
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
  ['morse word agrees', `${morseWord.agree} ${morseWord.ones} ${morseWord.longest}`, 'true 32 2'],
  ['morse runs', `${morseWord.singles} ${morseWord.doubles} ${morseWord.cube_free}`, '22 21 true'],
  ['morse boundary doubles', morseWord.doubling_agree, true],
  ['morse stage 3', Array.from(m.morse_stage(3)).join(','), '0,1,1,0,1,0,0,1'],
  ['morse parity folds', `${morseGallery[0].folds} ${morseGallery[0].tile.join('')} ${morseGallery[0].design}`, 'true 0110 9'],
  ['morse walsh folds', `${morseGallery[1].folds} ${morseGallery[1].tile.join('')} ${morseGallery[1].design}`, 'true 0001 7'],
  ['morse xor is parity', `${morseGallery[2].twin} ${morseSame(morseXor.types, morseParity.types)}`, 'parity true'],
  ['morse sum no fold', `${morseGallery[3].folds} ${morseGallery[3].faults} ${morseGallery[3].first.join(',')}`, 'false 1376 1,3'],
  ['morse signs are the lifts', `${morseSame(m.morse_signs('9', 2, 2, 6).types, morseParity.types)} ${morseSame(m.morse_signs('7', 2, 2, 6).types, morseAnd.types)}`, 'true true'],
  ['morse lift ones', morseParity.types.reduce((a: number, b: number) => a + b, 0), 2048],
  ['morse filter sign closed', `${morseSign.closed_exact} ${morseSign.morse_exact} ${morseSign.morse_faults} ${morseSign.lit}`, 'true false 128 128'],
  ['morse filter half a coin', morseSign.morse_faults * 2, morseSign.cells],
  ['morse filter design closed', `${morseFlat.closed_exact} ${morseFlat.side} ${morseFlat.lit} ${morseFlat.morse_faults}`, 'true 16 27 127'],
  ['morse filter base three', `${morseWide.closed_exact} ${morseWide.side} ${morseWide.morse_faults}`, 'true 27 null'],
  ['tile plane 5x5', `${tileWide.sheet.join('x')} ${tileWide.fills} ${tileWide.exposed} ${tileWide.buried}`, '45x45 1600 1280 720'],
  ['tile plane 5x5 walk', `${tileWide.vertices} ${tileWide.edges} ${tileWide.euler}`, '2016 3840 -224'],
  ['tile plane 3x9', `${tileTall.sheet.join('x')} ${tileTall.fills} ${tileTall.exposed} ${tileTall.euler}`, '27x81 1728 1404 -242'],
  ['tile plane fills multiply', `${tileWide.fills} ${tileTall.fills}`, `${25 * Number(tileWide.tile_fills)} ${27 * Number(tileTall.tile_fills)}`],
  ['tile cube 5x5x5', `${tileBlock.sheet.join('x')} ${tileBlock.fills} ${tileBlock.exposed} ${tileBlock.buried}`, '15x15x15 2500 4200 4800'],
  ['tile cube 5x5x5 walk', `${tileBlock.faces} ${tileBlock.euler}`, '9600 -324'],
  ['tile cube 3x9x3', `${tileSlab.sheet.join('x')} ${tileSlab.fills} ${tileSlab.exposed} ${tileSlab.euler}`, '9x27x9 1620 2952 -224'],
  ['tile hex 5x5', `${tileMesh.sheet.join('x')} ${tileMesh.triangles} ${tileMesh.fills} ${tileMesh.exposed}`, '47x33 1350 1050 414'],
  ['tile hex 3x9', `${tileStrip.sheet.join('x')} ${tileStrip.triangles} ${tileStrip.fills} ${tileStrip.exposed}`, '29x57 1458 1134 462'],
  ['tile hex 5x5 cropped', `${tileTrim.sheet.join('x')} ${tileTrim.triangles} ${tileTrim.fills} ${tileTrim.exposed}`, '43x27 1161 891 357'],
  ['tile hex 3x9 cropped', `${tileNarrow.sheet.join('x')} ${tileNarrow.triangles} ${tileNarrow.fills}`, '25x51 1275 969'],
  ['tile hex is a disc', [tileMesh, tileStrip, tileTrim, tileNarrow].map((r) => r.euler).join(','), '1,1,1,1'],
  ['tile grid draws the census', `${tileSheet.width} ${tileSheet.types.reduce((a: number, b: number) => a + b, 0)}`, `45 ${tileWide.fills}`],
  ['tile cells draws the census', m.tile_cells('23', 3, 1, 2, 5, 5, 5).length / 3, Number(tileBlock.fills)],
  ['tile svg draws the census', tileArt.match(/<polygon/g)?.length, tileTrim.triangles],
  ['magic perimeter word', m.magic_perimeter(['7', '9'], [3, 5], [2, 2]), '368'],
  ['magic hex cut is the slice', `${towerCut.triangles} ${towerCut.fills} ${towerCut.exposed}`, `${deep.triangles} ${deep.fills} 162`],
  ['magic hex iso skin', `${towerIso.grid.join('x')} ${towerIso.fills} ${towerIso.voids} ${towerIso.exposed}`, '18x35 486 0 88'],
];

// BLEND

const plotTerms = m.ledger_terms('23', 3, 2, 'surface', 'level', 8, '500000');
const plotRule = JSON.parse(m.blend_recurrence(plotTerms));
const plotCoefficients = JSON.stringify(plotRule.coefficients);
const plotSeries = JSON.parse(m.blend_series('23', 3, 2, 'surface', 'level', 8, '500000', 4));
const plotSide = JSON.parse(m.blend_series('7', 2, 2, 'fills', 'side', 12, '500000', 5));
const plotFamily = JSON.parse(m.blend_family(2, 2, 'fills', 'level', 3, '500000'));
const plotFills = m.ledger_terms('7', 2, 2, 'fills', 'level', 8, '500000');
const plotFaces = m.ledger_terms('7', 2, 2, 'surface', 'level', 8, '500000');
const plotMix = JSON.parse(m.blend_mix(plotFaces, plotFills, 'hadamard', 0, 3));
const plotSums = JSON.parse(m.blend_mix(plotFills, [], 'sigma', 0, 2));
const plotThin = JSON.parse(m.blend_mix(plotFills, [], 'decimate', 2, 2));

checks.push(
  ['blend_recurrence sponge', `${plotRule.order} ${plotCoefficients} ${plotRule.recurrence}`, '2 [[28,1],[-160,1]] a(n) = 28 a(n-1) - 160 a(n-2)'],
  ['blend_recurrence primes', m.blend_recurrence(['2', '3', '5', '7', '11', '13', '17', '19']), 'null'],
  ['blend_characteristic sponge', m.blend_characteristic(plotCoefficients), '[[1,1],[-28,1],[160,1]]'],
  ['blend_growth sponge', m.blend_growth(plotCoefficients).toFixed(9), '20.000000000'],
  ['blend_series sponge row', `${plotSeries.name} ${plotSeries.oeis} ${plotSeries.closed}`, 'mrly_bang_d3_23.surface.level A332705 a(L) = 28 a(L-1) - 160 a(L-2)'],
  ['blend_series sponge rule', `${plotSeries.order} ${plotSeries.polynomial} ${plotSeries.growth_from}`, '2 x^2 - 28 x + 160 the recurrence root'],
  ['blend_series sponge growth', `${plotSeries.growth.toFixed(9)} ${plotSeries.exponent.toFixed(9)}`, '20.000000000 1.301029996'],
  ['blend_series sponge views', `${plotSeries.ratios[0]} ${plotSeries.differences[1][0]} ${plotSeries.differences.length} ${plotSeries.log10[0].toFixed(7)}`, '14.6667 984 4 1.8573325'],
  ['blend_series carpet side', `${plotSide.closed} ${plotSide.order} ${plotSide.polynomial} ${plotSide.differences[3][0]}`, '3k^2 - 2k 3 x^3 - 3 x^2 + 3 x - 1 0'],
  ['blend_family plane', `${plotFamily.length} ${plotFamily[4].code} ${plotFamily[4].terms.join(',')}`, '6 7 8,64,512'],
  ['blend_family base three', JSON.parse(m.blend_family(2, 3, 'fills', 'level', 2, '500000')).length, 26],
  ['blend_mix hadamard terms', `${plotMix.terms[0]} ${plotMix.terms[3]}`, '128 14483456'],
  ['blend_mix hadamard rule', `${plotMix.order} ${plotMix.polynomial} ${plotMix.growth.toFixed(9)}`, '2 x^2 - 88 x + 1536 64.000000000'],
  ['blend_mix sigma and thin', `${plotSums.terms[2]} ${plotSums.order} ${plotThin.terms.join(',')}`, '584 2 8,512,32768,2097152'],
  ['blend_ops', m.blend_ops().join(','), 'add,sub,hadamard,cauchy,shift,decimate,delta,sigma,scale'],
  ['moire_correlation 3 5', m.moire_correlation(3, 5), 0],
  ['moire_correlation 3 9', m.moire_correlation(3, 9).toFixed(12), '0.219264504827'],
);

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

// CARRY

const carryAnchor = JSON.parse(m.carry_block(3, 3, 6));
const carryFive = JSON.parse(m.carry_block(5, 3, 4));
const carrySigns = JSON.parse(m.carry_signs(10));
const carryTail = JSON.parse(m.carry_ratios(3, 50)).at(-1);
const carryLadder = (base: number, dimension: number, levels: number) => JSON.parse(m.carry_block(base, dimension, levels)).terms.join(',');
const carryTraces = Array.from({ length: 6 }, (_, i) => JSON.parse(m.carry_block(3, i + 2, 1)).trace).join(',');
const carryOrders = Array.from({ length: 5 }, (_, i) => {
  const dimension = i + 2;
  const order = Math.ceil(dimension / 2);
  const read = JSON.parse(m.carry_block(3, dimension, 2 * order + 1));
  return `${read.order}${read.found}${read.fits ? '+' : '-'}`;
}).join(' ');
const carryCounts = Array.from({ length: 5 }, (_, i) => m.diagonal_count('23', 3, i + 1, 2, (3 * (3 ** (i + 1) - 1)) / 2)).join(',');
const carryLane = (key: 'three' | 'five') => carrySigns.map((row: { three: { sign: number }; five: { sign: number } }) => row[key].sign).join(',');

checks.push(
  ['carry_cap both bases', `${m.carry_cap(3)},${m.carry_cap(5)}`, '15,11'],
  ['carry_block 3 3 block', JSON.stringify(carryAnchor.block), '[[6,6],[1,3]]'],
  ['carry_block 3 3 digits', carryAnchor.digits.join(','), '1,3,3,6,3,3,1'],
  ['carry_block 3 3 reading', `${carryAnchor.polynomial} ${carryAnchor.trace} ${carryAnchor.determinant} ${carryAnchor.fill}`, 'x^2 - 9 x + 12 9 12 20'],
  ['carry_block 3 3 root', `${carryAnchor.read.root.toFixed(9)} ${((9 + Math.sqrt(33)) / 2).toFixed(9)}`, '7.372281323 7.372281323'],
  ['carry_block 3 3 exponent', `${carryAnchor.read.log_root.toFixed(6)} ${carryAnchor.read.log_fill.toFixed(6)} ${carryAnchor.read.sign}`, '1.818410 1.726833 1'],
  ['carry_block 3 3 ladder', carryAnchor.terms.join(','), '1,6,42,306,2250,16578,122202'],
  ['carry_block 3 3 ratios', carryAnchor.ratios.join(','), '6,7,7.2857,7.3529,7.368,7.3713'],
  ['carry_block 5 3 block', `${JSON.stringify(carryFive.block)} ${carryFive.polynomial} ${carryFive.fill}`, '[[18,30],[3,7]] x^2 - 25 x + 36 112'],
  ['carry_block 5 3 ladder', carryFive.terms.join(','), '1,18,414,9702,227646'],
  ['carry traces D=2..7', carryTraces, '2,9,11,60,47,336'],
  ['carry ladder D=4', carryLadder(3, 4, 6), '1,6,132,1848,29040,441408,6772128'],
  ['carry ladder D=5', carryLadder(3, 5, 4), '1,30,1000,35700,1321600'],
  ['carry ladder D=6', carryLadder(3, 6, 4), '1,20,4030,242300,24642700'],
  ['carry order is ceil D/2', carryOrders, '11+ 22+ 22+ 33+ 33+'],
  ['carry ladder is the cut', carryCounts, '6,42,306,2250,16578'],
  ['carry cut is the hexagon', `${slice.fills},${deep.fills},${series.slice(0, 2).map((row: { fills: number }) => row.fills).join(',')}`, '42,306,6,42'],
  ['carry sign law base 3', carryLane('three'), '-1,1,-1,1,-1,1,-1,1,-1'],
  ['carry sign law base 5', carryLane('five'), '-1,1,-1,1,-1,1,-1,1,-1'],
  ['carry open odd class', carrySigns.map((row: { open: boolean }) => (row.open ? 1 : 0)).join(''), '000001000'],
  ['carry spectral ratio 50', `${carryTail.dimension} ${carryTail.ratio.toFixed(9)} ${(13 / 12).toFixed(9)}`, '50 1.083333333 1.083333333'],
);

// MANIFEST

import { existsSync, readdirSync } from 'node:fs';

const desk = import.meta.dir;
const manifest = await Bun.file(`${desk}/pages.json`).json();
const shelfKeys = new Set(manifest.shelves.map((shelf: { key: string }) => shelf.key));
const rows: { name: string; blurb: string; category: string; research: string | null; paper: string | null }[] = manifest.pages;
const ignored = new Set(['dist', 'lib', 'node_modules', 'pkg', 'scripts']);
const folders = readdirSync(`${desk}/demos`, { withFileTypes: true })
  .filter((entry) => entry.isDirectory() && !ignored.has(entry.name) && existsSync(`${desk}/demos/${entry.name}/index.html`))
  .map((entry) => entry.name)
  .sort();
const named = rows.map((row) => row.name).sort();
const strays = named.filter((name) => !folders.includes(name)).concat(folders.filter((name) => !named.includes(name)));
const shelfless = rows.filter((row) => !shelfKeys.has(row.category)).map((row) => row.name);
const unpaged = rows.filter((row) => row.research && !existsSync(`${desk}/../../research/${row.research}.md`)).map((row) => row.name);
const readme = await Bun.file(`${desk}/README.md`).text();
const listed = readme.slice(readme.indexOf('## PAGES')).split('\n')
  .map((line) => /^- \[([a-z]+)\]\(demos\/\1\/\) - (.+)$/.exec(line))
  .filter((hit) => hit !== null)
  .map((hit) => `${hit![1]} ${hit![2]}`);
const wanted = rows.map((row) => `${row.name} ${row.blurb}`);
const drift = listed.find((line, at) => line !== wanted[at]) ?? (listed.length === wanted.length ? 'none' : `${listed.length} of ${wanted.length} lines`);
const shelf = `${desk}/../../carlomitchener/research`;

checks.push(
  ['manifest is the folders', `${named.length} ${strays.join(',') || 'none'}`, `${folders.length} none`],
  ['manifest shelves exist', shelfless.join(',') || 'none', 'none'],
  ['manifest research exists', unpaged.join(',') || 'none', 'none'],
  ['readme is the manifest', drift.slice(0, 40), 'none'],
);

if (existsSync(shelf)) {
  const lost = rows.filter((row) => row.paper && !existsSync(`${shelf}/${row.paper}`)).map((row) => row.name);
  checks.push(['manifest papers exist', lost.join(',') || 'none', 'none']);
} else {
  console.log('note  no shelf checkout beside this one, paper lanes unchecked');
}

let failed = 0;
for (const [label, got, want] of checks) {
  const ok = got === want;
  if (!ok) failed += 1;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${label.padEnd(26)} ${String(got)}${ok ? '' : ` (want ${String(want)})`}`);
}
console.log(failed ? `${failed} failed` : `${checks.length} checks green`);
process.exit(failed ? 1 : 0);
