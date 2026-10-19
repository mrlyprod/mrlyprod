import { lock } from '../../shell/lock.js';
import { calm } from '../../shell/prefs.js';
import { load, STILL } from '../../shell/savers.js';

const QUICK = 250;
const TYPE = 'image/webp';
const QUALITY = 0.85;

export const shots = new Map();

/* SHOOT */

function nap(ms, job) {
  return new Promise((done) => {
    const timer = setTimeout(done, ms);
    job.wake = () => {
      clearTimeout(timer);
      done();
    };
  });
}

async function shoot(id, box, job) {
  const canvas = document.createElement('canvas');
  box.append(canvas);
  let stop = null;
  try {
    stop = await lock(canvas, id);
    const shown = await load(id);
    if (job.gone) return null;
    await nap(stop.scene?.still || calm() ? QUICK : (shown?.still ?? STILL), job);
    if (job.gone) return null;
    stop.pause?.();
    return { url: canvas.toDataURL(TYPE, QUALITY), as: stop.as };
  } catch (error) {
    console.error(error);
    return job.gone ? null : { url: '', as: '' };
  } finally {
    stop?.();
    canvas.remove();
  }
}

/* QUEUE */

export function shooter(ids, done) {
  const seen = new Set();
  const job = { gone: false, wake: null };
  const box = document.createElement('div');
  box.className = 'savers-shot';
  box.setAttribute('aria-hidden', 'true');
  document.body.append(box);
  let busy = false;
  const next = () => ids.find((id) => seen.has(id) && !shots.has(id)) ?? ids.find((id) => !shots.has(id));
  const pump = async () => {
    if (busy || job.gone || document.hidden) return;
    const id = next();
    if (!id) return;
    busy = true;
    const shot = await shoot(id, box, job);
    busy = false;
    if (!shot) return;
    shots.set(id, shot);
    done(id);
    pump();
  };
  const eye = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (entry.isIntersecting) seen.add(entry.target.dataset.id);
      else seen.delete(entry.target.dataset.id);
    }
    pump();
  });
  document.addEventListener('visibilitychange', pump);
  return {
    watch: (node) => eye.observe(node),
    stop() {
      job.gone = true;
      job.wake?.();
      eye.disconnect();
      document.removeEventListener('visibilitychange', pump);
      box.remove();
    },
  };
}
