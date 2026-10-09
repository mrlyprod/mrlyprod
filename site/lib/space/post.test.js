import { expect, test } from 'bun:test';
import { gl } from './fake.js';
import { post } from './post.js';

const VIEW = { w: 1280, h: 720, dpr: 1, t: 0 };

const formats = (fake) => fake.log.filter(([key]) => key === 'texImage2D').map((call) => call[3]);

test('targets are RGBA16F with a float colour buffer and RGBA8 without one', () => {
  const high = gl();
  post(high, VIEW);
  const low = gl({ ext: [] });
  post(low, VIEW);
  expect(new Set(formats(high).slice(1))).toEqual(new Set([high.RGBA16F]));
  expect(new Set(formats(low))).toEqual(new Set([low.RGBA8]));
});

test('a flash is exposure through the tone curve, never a white fill', () => {
  const fake = gl();
  const film = post(fake, VIEW);
  film.begin(1);
  film.end({ ev: 5 });
  const gains = fake.log.filter(([key, at]) => key === 'uniform1f' && at.name === 'uGain').map((call) => call[2]);
  expect(gains).toEqual([32, 32]);
  expect(fake.log.filter(([key]) => key === 'clearColor').map((call) => call.slice(1))).toEqual([[0, 0, 0, 1]]);
});

test('a phone blooms 4 mips from quarter res, a desk 5 from half res', () => {
  const sizes = (view) => {
    const fake = gl();
    post(fake, view).begin(1);
    return fake.log.filter(([key]) => key === 'texImage2D').slice(-5).map((call) => call[4]);
  };
  expect(sizes({ ...VIEW, w: 1080, h: 1920, dpr: 3 })).toEqual([1080, 270, 135, 68, 34]);
  expect(sizes(VIEW)).toEqual([640, 320, 160, 80, 40]);
});

test('a flare smears the 1/8 mip in one horizontal pass the finish reads once, no pass without it', () => {
  const passes = (flare) => {
    const fake = gl();
    const film = post(fake, VIEW);
    film.begin(1);
    film.end({ flare });
    return fake.draws().filter(({ fs }) => fs.includes('w += k;')).map(({ viewport }) => viewport);
  };
  expect([passes(0), passes(1)]).toEqual([[], [[0, 0, 160, 90]]]);
});
