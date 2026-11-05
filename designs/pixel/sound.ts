const VOICES = 6;

const SCALE = [523.25, 587.33, 659.25, 783.99, 880, 1046.5, 1174.66, 1318.51];

export type Arcade = {
  wake: (e: Event) => void;
  press: (x: number, big: boolean) => void;
  key: (key: string) => void;
  swipe: () => void;
  stop: () => void;
};

export function arcade(on: () => boolean): Arcade {
  let ctx: AudioContext | null = null;
  let master: GainNode | null = null;
  let voices = 0;
  let swiped = 0;
  const blip = (from: number, to: number, time: number, type: OscillatorType, at = 0, loud = 1) => {
    if (!ctx || !master || !on() || voices >= VOICES || ctx.state !== "running") return;
    voices++;
    const t = ctx.currentTime + at;
    const tone = ctx.createOscillator();
    const gain = ctx.createGain();
    tone.type = type;
    tone.frequency.setValueAtTime(from, t);
    tone.frequency.exponentialRampToValueAtTime(to, t + time);
    gain.gain.setValueAtTime(loud, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + time);
    tone.connect(gain).connect(master);
    tone.start(t);
    tone.stop(t + time + 0.02);
    tone.onended = () => {
      voices--;
      tone.disconnect();
      gain.disconnect();
    };
  };
  return {
    wake(e) {
      if (!on() || !e.isTrusted || !navigator.userActivation?.isActive || typeof AudioContext !== "function") return;
      if (!ctx) {
        ctx = new AudioContext({ latencyHint: "interactive" });
        master = ctx.createGain();
        master.gain.value = 0.05;
        master.connect(ctx.destination);
      }
      if (ctx.state === "suspended") ctx.resume().catch(() => {});
    },
    press(x, big) {
      const f = 660 + (x / Math.max(1, innerWidth)) * 660;
      blip(f, f * 2, 0.07, "square");
      if (big) blip(f * 1.5, f * 3, 0.09, "square", 0.06, 0.8);
    },
    key(key) {
      if (key === "Enter") {
        blip(988, 988, 0.08, "square");
        blip(1319, 1319, 0.3, "square", 0.08);
        return;
      }
      if (key === "Backspace" || key === "Delete") return blip(440, 110, 0.12, "triangle");
      const code = key.length === 1 ? key.toLowerCase().charCodeAt(0) : key.length;
      blip(SCALE[code % SCALE.length]!, SCALE[code % SCALE.length]! * 1.01, 0.05, "triangle");
    },
    swipe() {
      const now = performance.now();
      if (now - swiped < 70) return;
      swiped = now;
      blip(1800, 600, 0.05, "sawtooth", 0, 0.35);
    },
    stop() {
      ctx?.close().catch(() => {});
      ctx = null;
      master = null;
    },
  };
}
