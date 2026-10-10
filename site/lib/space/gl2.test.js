import { expect, test } from 'bun:test';
import { gl } from './fake.js';
import { bake, fill, governor, hdr, program, quads, target, texture, tier } from './gl2.js';

test('tier reads the short side in css px and the store area, and a fixed frame is never a phone', () => {
  const sizes = [[1170, 2532, 3], [1080, 1920, 2], [2560, 1440, 2], [1920, 1080, 1], [3840, 2160, 2], [2048, 2048, 1]];
  expect(sizes.map(([w, h, dpr]) => tier({ w, h, dpr }))).toEqual(['phone', 'phone', 'desk', 'desk', 'big', 'desk']);
  expect([tier({ w: 1080, h: 1920, dpr: 2, fixed: true }), tier({ w: 1280, h: 720, dpr: 720 / 540, fixed: true })]).toEqual(['desk', 'desk']);
});

test('the governor drops a notch after 1 s over 22 ms, climbs after 3 s under 14 ms, and pins when fixed', () => {
  const view = { t: 0 };
  const g = governor(view, [1, 0.75, 0.5]);
  const run = (step, ms) => {
    const seen = [];
    for (let t = 0; t < ms; t += step) {
      view.t += step;
      seen.push(g.tick());
    }
    return [...new Set(seen)];
  };
  expect(run(16, 5000)).toEqual([1]);
  expect(run(30, 1500)).toEqual([1, 0.75]);
  expect(run(30, 3000)).toEqual([0.75, 0.5]);
  expect(run(10, 2900)).toEqual([0.5]);
  expect(run(10, 3200)).toEqual([0.5, 0.75]);
  view.fixed = true;
  expect(run(100, 3000)).toEqual([1]);
});

test('program throws the info log of a shader that does not compile', () => {
  const fake = gl({ bad: 'vec5' });
  expect(() => program(fake, '#version 300 es\nout vec4 o;\nvoid main() { o = vec5(1.0); }')).toThrow("ERROR: 0:1: 'vec5' : syntax error");
});

test('set picks the setter by the active uniform type and binds samplers to their own units', () => {
  const fake = gl();
  const frag = 'uniform float uA;\nuniform vec3 uB;\nuniform mat3 uC;\nuniform sampler2D uD;\nuniform samplerCube uE;\nvoid main() {}';
  const prog = program(fake, frag);
  fake.log.length = 0;
  prog.set({ uA: 2, uB: [1, 2, 3], uC: [1, 0, 0, 0, 1, 0, 0, 0, 1], uD: { tex: 'a' }, uE: 'b', uZ: 9 });
  const verbs = fake.log.filter(([key]) => key.startsWith('uniform') || key === 'bindTexture').map(([key, ...args]) => [key, ...args.slice(0, 2)]);
  expect(verbs).toEqual([
    ['uniform1f', { name: 'uA' }, 2],
    ['uniform3fv', { name: 'uB' }, [1, 2, 3]],
    ['uniformMatrix3fv', { name: 'uC' }, false],
    ['bindTexture', fake.TEXTURE_2D, 'a'],
    ['uniform1i', { name: 'uD' }, 0],
    ['bindTexture', fake.TEXTURE_CUBE_MAP, 'b'],
    ['uniform1i', { name: 'uE' }, 1],
  ]);
});

const CUBE = '#version 300 es\nuniform int uFace;\nin vec3 d;\nout vec4 o;\nvoid main() { o = vec4(d, 1.0); }';

test('bake draws the six cube faces in order, each at the size of the face', () => {
  const fake = gl();
  bake(fake, 64, CUBE);
  const faces = fake.log.filter(([key]) => key === 'framebufferTexture2D').map(([, , , face]) => face - fake.TEXTURE_CUBE_MAP_POSITIVE_X);
  expect([faces, fake.draws().map((one) => one.uniforms.uFace), fake.draws().map((one) => one.viewport)]).toEqual([[0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5], Array(6).fill([0, 0, 64, 64])]);
});

test('bake leaves the viewport as it found it', () => {
  const fake = gl();
  fake.viewport(0, 0, 1280, 720);
  bake(fake, 64, CUBE);
  expect(Array.from(fake.getParameter(fake.VIEWPORT))).toEqual([0, 0, 1280, 720]);
});

test('a quad batch draws all its instances in one instanced call, each attribute with divisor 1', () => {
  const fake = gl();
  const prog = program(fake, '#version 300 es\nvoid main() {}', '#version 300 es\nin vec2 aAt;\nin float aSize;\nvoid main() {}');
  const batch = quads(fake, { aAt: 2, aSize: 1 }, 10);
  fake.log.length = 0;
  batch.draw(prog, 4);
  const divisors = fake.log.filter(([key]) => key === 'vertexAttribDivisor').map(([, , divisor]) => divisor);
  expect([fake.draws().map((one) => [one.mode, one.count, one.instances]), divisors]).toEqual([[[fake.TRIANGLE_STRIP, 4, 4]], [1, 1]]);
});

test('a target with extra adds a second float attachment and draws to both', () => {
  const fake = gl();
  hdr(fake);
  fake.log.length = 0;
  const one = target(fake, 8, 4, { hdr: true, extra: 1 });
  const slots = fake.log.filter(([key]) => key === 'framebufferTexture2D').map(([, , slot, , tex]) => [slot, tex]);
  const formats = fake.log.filter(([key]) => key === 'texImage2D').map(([, , , inner]) => inner);
  expect([slots, formats, fake.log.find(([key]) => key === 'drawBuffers')[1]]).toEqual([[[fake.COLOR_ATTACHMENT0, one.tex], [fake.COLOR_ATTACHMENT1, one.aux]], [fake.RGBA16F, fake.RGBA16F], [fake.COLOR_ATTACHMENT0, fake.COLOR_ATTACHMENT1]]);
});

test('a texture with a depth is 3D and one without is 2D', () => {
  const fake = gl();
  texture(fake, { w: 4, h: 4, d: 4, format: 'r8', data: new Uint8Array(64) });
  texture(fake, { w: 4, h: 4 });
  expect(fake.log.filter(([key]) => key.startsWith('texImage')).map(([key]) => key)).toEqual(['texImage3D', 'texImage2D']);
});

test('a 32-bit float texture is sampled nearest with no mips, whatever filter it asks for', () => {
  const fake = gl();
  texture(fake, { w: 4, h: 4, format: 'rg32f', filter: 'mip' });
  const filters = fake.log.filter(([key, , name]) => key === 'texParameteri' && [fake.TEXTURE_MIN_FILTER, fake.TEXTURE_MAG_FILTER].includes(name)).map(([, , , value]) => value);
  expect([filters, fake.log.some(([key]) => key === 'generateMipmap')]).toEqual([[fake.NEAREST, fake.NEAREST], false]);
});

test('an hdr target on a context that cannot render floats is RGBA8', () => {
  const fake = gl({ ext: [] });
  target(fake, 8, 8, { hdr: true });
  const image = fake.log.find(([key]) => key === 'texImage2D');
  expect([image[3], image[8]]).toEqual([fake.RGBA8, fake.UNSIGNED_BYTE]);
});

test('an hdr target on a context with only the half-float extension is RGBA16F', () => {
  const fake = gl({ ext: ['EXT_color_buffer_half_float'] });
  target(fake, 8, 8, { hdr: true });
  const image = fake.log.filter(([key]) => key === 'texImage2D').at(-1);
  expect([image[3], image[8]]).toEqual([fake.RGBA16F, fake.HALF_FLOAT]);
});

test('use binds again every sampler a program was given, so another program cannot leave its own texture there', () => {
  const fake = gl();
  const a = program(fake, 'uniform sampler2D uA;\nvoid main() {}');
  const b = program(fake, 'uniform sampler2D uB;\nvoid main() {}');
  const [one, two] = [fake.createTexture(), fake.createTexture()];
  a.set({ uA: one });
  b.set({ uB: two });
  fake.log.length = 0;
  a.use();
  expect(fake.log.filter(([key]) => key === 'bindTexture')).toEqual([['bindTexture', fake.TEXTURE_2D, one]]);
});

test('a texture upload binds on unit 15 and leaves the units of the programs alone', () => {
  const fake = gl();
  const prog = program(fake, 'uniform sampler2D uA;\nvoid main() {}');
  const one = fake.createTexture();
  prog.set({ uA: one });
  const fresh = texture(fake, { w: 2, h: 2 });
  fill(fake);
  expect(fake.draws()[0].units).toEqual({ 0: one.id, 15: fresh.id });
});
