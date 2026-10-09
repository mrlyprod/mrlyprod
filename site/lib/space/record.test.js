import { afterEach, beforeEach, expect, test } from 'bun:test';
import { frame, mime, record } from './record.js';

const real = { createObjectURL: URL.createObjectURL, revokeObjectURL: URL.revokeObjectURL, setTimeout: globalThis.setTimeout };
let seen;
let supported;

class Recorder {
  static isTypeSupported = (type) => supported.includes(type);
  constructor(stream, options) {
    this.stream = stream;
    this.options = options;
    this.state = 'inactive';
    this.mimeType = options.mimeType ?? '';
  }
  start(ms) {
    this.state = 'recording';
    seen.push(['start', ms]);
  }
  stop() {
    this.state = 'inactive';
    this.ondataavailable({ data: new Blob(['frames'], { type: this.mimeType }) });
    this.onstop();
  }
}

const stream = (video) => {
  const tracks = [video];
  return { getVideoTracks: () => [video], getTracks: () => tracks, addTrack: (one) => tracks.push(one) };
};

const canvas = ({ frames = true, gl = null } = {}) => ({
  getContext: (kind) => (kind === '2d' ? (gl ? null : {}) : gl),
  captureStream: (fps) => {
    seen.push(['capture', fps]);
    return stream({ stop: () => {}, requestFrame: frames ? () => seen.push('frame') : undefined });
  },
});

const handle = (at) => {
  const one = { canvas: at, fix: (held) => ((one.held = held), seen.push(['fix', held && held.frame])) };
  return one;
};

const draws = (one, n) => {
  for (let i = 0; i < n; i++) one.held?.after({ t: i, late: 0 });
};

beforeEach(() => {
  seen = [];
  supported = ['video/webm;codecs=vp9,opus', 'video/mp4'];
  globalThis.MediaRecorder = Recorder;
  const pen = { drawImage: () => seen.push('copy') };
  globalThis.document = Object.assign(new EventTarget(), {
    hidden: false,
    createElement: (tag) => (tag === 'a' ? { click() { seen.push(['save', this.download]); } } : { ...canvas(), getContext: () => pen }),
  });
  URL.createObjectURL = () => 'blob:x';
  URL.revokeObjectURL = () => {};
  globalThis.setTimeout = (fn) => fn();
});

afterEach(() => {
  Object.assign(URL, { createObjectURL: real.createObjectURL, revokeObjectURL: real.revokeObjectURL });
  globalThis.setTimeout = real.setTimeout;
  delete globalThis.MediaRecorder;
  delete globalThis.document;
});

test('frame is 1080 by 1920 for 9:16 and 1920 by 1080 for 16:9, and 720p on a coarse pointer', () => {
  expect([frame('9:16'), frame('16:9'), frame('9:16', true), frame('16:9', true)]).toEqual([[1080, 1920], [1920, 1080], [720, 1280], [1280, 720]]);
});

test('mime takes the first type the recorder supports, webm before mp4', () => {
  const first = mime();
  supported = ['video/mp4'];
  expect([first, mime()]).toEqual(['video/webm;codecs=vp9,opus', 'video/mp4']);
});

test('a take fixes the frame and the step, then asks one requestFrame per draw', () => {
  const one = handle(canvas());
  record(one, { ratio: '9:16', name: 'lightspeed-7' });
  draws(one, 3);
  expect([one.held.step, seen]).toEqual([1000 / 30, [['capture', 0], ['start', 1000], ['fix', [1080, 1920]], 'frame', 'frame', 'frame']]);
});

test('stop returns the stage and saves <name>-9x16.webm', async () => {
  const one = handle(canvas());
  const take = record(one, { ratio: '9:16', name: 'lightspeed-7' });
  draws(one, 2);
  const file = await take.stop();
  expect([file, seen.slice(-2)]).toEqual(['lightspeed-7-9x16.webm', [['fix', null], ['save', 'lightspeed-7-9x16.webm']]]);
});

test('stop waits four frame periods, then returns the stage and ends the recorder', async () => {
  const waits = [];
  globalThis.setTimeout = (fn, ms) => waits.push([fn, ms]);
  const one = handle(canvas());
  const take = record(one, { fps: 30 });
  take.stop();
  expect([waits.length, waits[0][1], seen.filter((entry) => entry[0] === 'fix').length]).toEqual([1, 4 * (1000 / 30), 1]);
  waits[0][0]();
  expect([await take.done, seen.filter((entry) => entry[0] === 'fix').length]).toEqual(['mrly-9x16.webm', 2]);
});

test('a take stops itself at 180 s of frames', async () => {
  const one = handle(canvas());
  const take = record(one, { ratio: '16:9', name: 'a' });
  const after = one.held.after;
  for (let i = 0; i < 5401; i++) after({ t: i, late: 0 });
  expect([await take.done, take.secs(), seen.filter((entry) => entry === 'frame').length]).toEqual(['a-16x9.webm', 180, 5400]);
});

test('a recorder that writes mp4 names the file .mp4', async () => {
  supported = ['video/mp4'];
  const take = record(handle(canvas()), { ratio: '16:9', name: 'lightspeed-7' });
  expect(await take.stop()).toBe('lightspeed-7-16x9.mp4');
});

test('without requestFrame the take captures at fps', () => {
  const one = handle(canvas({ frames: false }));
  record(one, { fps: 30 });
  draws(one, 2);
  expect(seen.filter((entry) => entry[0] === 'capture')).toEqual([['capture', 0], ['capture', 30]]);
});

test('a WebGL canvas that drops its buffer is copied into a 2D canvas after each draw, before the frame', () => {
  const one = handle(canvas({ gl: { getContextAttributes: () => ({ preserveDrawingBuffer: false }) } }));
  record(one, {});
  draws(one, 2);
  expect(seen.filter((entry) => typeof entry === 'string')).toEqual(['copy', 'frame', 'copy', 'frame']);
});

test('a take may name its own frame size', () => {
  const one = handle(canvas());
  record(one, { ratio: '16:9', size: [1280, 720] });
  expect(one.held.frame).toEqual([1280, 720]);
});
