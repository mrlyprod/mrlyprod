const FADE = 1 / 64

const ROOT = 43

const MAJOR = [0, 2, 4, 5, 7, 9, 11]

const NAMES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]

const prefs = { sound: true, haptics: true, note: "random", wave: "sine", duration: 150 }

let ctx: AudioContext | null = null
let bus: DynamicsCompressorNode | null = null
const live = new Map<string, { osc: OscillatorNode; gain: GainNode }>()

const freq = (midi: number): number => 440 * 2 ** ((midi - 69) / 12)

const shape = (wave: string): OscillatorType =>
  wave === "triangle" || wave === "square" || wave === "sawtooth" ? wave : "sine"

export function pref(key: string, value: unknown): void {
  ;(prefs as Record<string, unknown>)[key] = value
  if (key === "sound" && value === false) silence()
}

export function unlock(): void {
  if (typeof AudioContext === "undefined") return
  if (ctx === null) {
    ctx = new AudioContext()
    bus = ctx.createDynamicsCompressor()
    bus.threshold.value = -6
    bus.knee.value = 12
    bus.ratio.value = 12
    bus.attack.value = 0.003
    bus.release.value = 0.1
    bus.connect(ctx.destination)
    document.addEventListener("visibilitychange", () => {
      if (document.hidden) silence()
    })
  }
  if (ctx.state === "suspended") void ctx.resume()
}

function voice(pitch: number, wave: string, gain: number): { osc: OscillatorNode; gain: GainNode } | null {
  if (ctx === null || bus === null || !prefs.sound) return null
  const osc = ctx.createOscillator()
  osc.type = shape(wave)
  osc.frequency.value = pitch
  const env = ctx.createGain()
  env.gain.setValueAtTime(0, ctx.currentTime)
  env.gain.linearRampToValueAtTime(gain, ctx.currentTime + FADE)
  osc.connect(env)
  env.connect(bus)
  osc.start()
  return { osc, gain: env }
}

export function play(pitch: number, wave = prefs.wave, ms = prefs.duration, gain = 0.3): void {
  if (ctx === null) return
  const v = voice(pitch, wave, gain)
  if (v === null) return
  const end = ctx.currentTime + Math.max(ms / 1000, FADE * 2)
  v.gain.gain.setValueAtTime(gain, end - FADE)
  v.gain.gain.linearRampToValueAtTime(0, end)
  v.osc.stop(end)
}

export function start(id: string, pitch: number, wave = prefs.wave, gain = 0.3): void {
  if (ctx === null) return
  stop(id)
  const v = voice(pitch, wave, gain)
  if (v !== null) live.set(id, v)
}

export function stop(id: string): void {
  const v = live.get(id)
  if (v === undefined || ctx === null) return
  live.delete(id)
  v.gain.gain.cancelScheduledValues(ctx.currentTime)
  v.gain.gain.setValueAtTime(v.gain.gain.value, ctx.currentTime)
  v.gain.gain.linearRampToValueAtTime(0, ctx.currentTime + FADE)
  v.osc.stop(ctx.currentTime + FADE)
}

export function silence(): void {
  for (const id of [...live.keys()]) stop(id)
}

export function tick(): void {
  buzz(10)
  if (!prefs.sound) return
  const midi =
    prefs.note === "random"
      ? ROOT + 12 * Math.floor(Math.random() * 2) + (MAJOR[Math.floor(Math.random() * 7)] ?? 0)
      : 48 + Math.max(0, NAMES.indexOf(prefs.note))
  play(freq(midi), prefs.wave, prefs.duration)
}

export function buzz(pattern: number | number[] = 10): void {
  if (!prefs.haptics) return
  if (typeof navigator !== "undefined" && "vibrate" in navigator) navigator.vibrate(pattern)
}
