import { expect, test } from 'bun:test';
import { gl } from './fake.js';
import { fill, program, target } from './gl2.js';

test('draws reports the shaders, uniforms, framebuffer and viewport each draw saw', () => {
  const fake = gl();
  const prog = program(fake, 'uniform vec2 uRes;\nvoid main() {}');
  const to = target(fake, 8, 8);
  fake.bindFramebuffer(fake.FRAMEBUFFER, to.fb);
  fake.viewport(0, 0, 8, 8);
  prog.set({ uRes: [8, 8] });
  fill(fake);
  fake.bindFramebuffer(fake.FRAMEBUFFER, null);
  fake.viewport(0, 0, 4, 4);
  prog.set({ uRes: [4, 4] });
  fill(fake);
  const [first, second] = fake.draws();
  expect([first.fs.includes('uRes'), first.uniforms, first.fb === to.fb, first.viewport, first.mode, first.count, second.uniforms, second.fb, second.viewport]).toEqual([true, { uRes: [8, 8] }, true, [0, 0, 8, 8], fake.TRIANGLES, 3, { uRes: [4, 4] }, null, [0, 0, 4, 4]]);
});

test('a gl constant that does not exist throws, so a typo cannot pass', () => {
  const fake = gl();
  expect(fake.FUNC_ADD).toBe(0x8006);
  expect(() => fake.FUNC_ADDD).toThrow('no constant FUNC_ADDD');
});
