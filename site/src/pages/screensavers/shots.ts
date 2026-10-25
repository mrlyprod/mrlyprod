import type { Lock } from "../../shell/lock.ts";
import { lock } from "../../shell/lock.ts";
import { calm } from "../../shell/prefs.ts";
import { load, STILL } from "../../shell/savers.ts";

/* TYPES */

export type Shot = { url: string; as: string };

export type Job = { gone: boolean; wake: (() => void) | null };

export type Shooter = { watch: (node: HTMLElement) => void; stop: () => void };

const QUICK = 250;
const TYPE = "image/webp";
const QUALITY = 0.85;

export const shots = new Map<string, Shot>();

/* SHOOT */

function nap(ms: number, job: Job) {
  return new Promise<void>((done) => {
    const timer = setTimeout(done, ms);
    job.wake = () => {
      clearTimeout(timer);
      done();
    };
  });
}

async function shoot(id: string, box: HTMLElement, job: Job): Promise<Shot | null> {
  const canvas = document.createElement("canvas");
  box.append(canvas);
  let stop: Lock | null = null;
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
    return job.gone ? null : { url: "", as: "" };
  } finally {
    stop?.();
    canvas.remove();
  }
}

/* QUEUE */

export function shooter(ids: string[], done: (id: string) => void): Shooter {
  const seen = new Set<string | undefined>();
  const job: Job = { gone: false, wake: null };
  const box = document.createElement("div");
  box.className = "savers-shot";
  box.setAttribute("aria-hidden", "true");
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
      const { id } = (entry.target as HTMLElement).dataset;
      if (entry.isIntersecting) seen.add(id);
      else seen.delete(id);
    }
    pump();
  });
  document.addEventListener("visibilitychange", pump);
  return {
    watch: (node: HTMLElement) => eye.observe(node),
    stop() {
      job.gone = true;
      job.wake?.();
      eye.disconnect();
      document.removeEventListener("visibilitychange", pump);
      box.remove();
    },
  };
}
