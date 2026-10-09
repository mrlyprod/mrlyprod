import { download } from '../export.js';

const TYPES = ['video/webm;codecs=vp9,opus', 'video/webm;codecs=vp8,opus', 'video/webm', 'video/mp4'];

const FRAMES = { '9:16': [1080, 1920], '16:9': [1920, 1080] };

const SMALL = { '9:16': [720, 1280], '16:9': [1280, 720] };

const CAP = 180;

const CHUNK = 1000;

export function frame(ratio, coarse) {
  const sizes = coarse ? SMALL : FRAMES;
  return [...(sizes[ratio] ?? sizes['9:16'])];
}

export function mime() {
  const ok = globalThis.MediaRecorder?.isTypeSupported;
  return TYPES.find((type) => ok?.(type)) ?? '';
}

const coarse = () => typeof matchMedia === 'function' && matchMedia('(pointer: coarse)').matches;

const webgl = (canvas) => (canvas.getContext('2d') ? null : (canvas.getContext('webgl2') ?? canvas.getContext('webgl')));

function source(canvas, [w, h]) {
  const gl = webgl(canvas);
  if (!gl || gl.getContextAttributes?.()?.preserveDrawingBuffer) return { from: canvas, copy: () => {} };
  const from = document.createElement('canvas');
  from.width = w;
  from.height = h;
  const pen = from.getContext('2d');
  return { from, copy: () => pen.drawImage(canvas, 0, 0, w, h) };
}

function capture(from, fps) {
  const still = from.captureStream(0);
  const track = still.getVideoTracks()[0];
  if (typeof track?.requestFrame === 'function') return { stream: still, nudge: () => track.requestFrame() };
  for (const one of still.getTracks()) one.stop();
  return { stream: from.captureStream(fps), nudge: () => {} };
}

export function record(handle, { ratio = '9:16', fps = 30, name = 'mrly', audio, size = frame(ratio, coarse()) } = {}) {
  if (typeof MediaRecorder !== 'function') throw new Error('this browser has no MediaRecorder');
  const step = 1000 / fps;
  const { from, copy } = source(handle.canvas, size);
  const { stream, nudge } = capture(from, fps);
  for (const track of audio?.getAudioTracks?.() ?? []) stream.addTrack(track);
  const type = mime();
  const rate = Math.min(...size) >= 1080 ? 16e6 : 8e6;
  const recorder = new MediaRecorder(stream, type ? { mimeType: type, videoBitsPerSecond: rate } : { videoBitsPerSecond: rate });
  const chunks = [];
  let drawn = 0;
  let behind = 0;
  let ended = false;
  let finish;
  const done = new Promise((ok) => (finish = ok));
  const secs = () => (drawn * step) / 1000;
  const hide = () => {
    if (document.hidden && recorder.state === 'recording') recorder.pause();
    else if (!document.hidden && recorder.state === 'paused') recorder.resume();
  };
  const stop = () => {
    if (ended) return done;
    ended = true;
    document.removeEventListener('visibilitychange', hide);
    setTimeout(() => {
      handle.fix(null);
      if (recorder.state !== 'inactive') recorder.stop();
    }, 4 * step);
    return done;
  };
  recorder.ondataavailable = (e) => {
    if (e.data?.size) chunks.push(e.data);
  };
  recorder.onstop = () => {
    for (const track of stream.getVideoTracks()) track.stop();
    const kind = recorder.mimeType || type || 'video/webm';
    const blob = new Blob(chunks, { type: kind });
    finish(download(blob, `${name}-${ratio.replace(':', 'x')}.${/mp4/.test(kind) ? 'mp4' : 'webm'}`));
  };
  const after = ({ late }) => {
    if (ended) return;
    copy();
    nudge();
    drawn += 1;
    behind = late;
    if (secs() >= CAP) stop();
  };
  document.addEventListener('visibilitychange', hide);
  recorder.start(CHUNK);
  handle.fix({ frame: size, step, after });
  return { stop, secs, late: () => behind, done };
}
