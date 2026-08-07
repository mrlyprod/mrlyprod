import { sound } from "mrlyui"
import type { Args, Effect, Send } from "./types"

export function perform(effect: Effect, emit: Send): void {
  switch (effect.kind) {
    case "notify": {
      const data = effect.data as { title?: string; body?: string }
      sound.buzz([30, 50, 30])
      if ("Notification" in window && Notification.permission === "granted") {
        new Notification(data.title ?? "", { body: data.body ?? "" })
      }
      break
    }
    case "sound": {
      const data = effect.data as { op?: string; id?: string; freq?: number; wave?: string; ms?: number; gain?: number }
      const hz = data.freq === undefined ? undefined : data.freq / 1000
      const level = data.gain === undefined ? undefined : data.gain / 100
      if (data.op === "note" && hz !== undefined) sound.play(hz, data.wave, data.ms, level)
      else if (data.op === "start" && data.id !== undefined && hz !== undefined) sound.start(data.id, hz, data.wave, level)
      else if (data.op === "stop" && data.id !== undefined) sound.stop(data.id)
      break
    }
    case "copy": {
      const data = effect.data as { text?: string }
      if (data.text !== undefined) void navigator.clipboard.writeText(data.text)
      break
    }
    case "fetch": {
      const data = effect.data as { url?: string; as?: string }
      const ret = effect.call
      if (ret === undefined || data.url === undefined) break
      const land = (args: Args) => emit({ ...ret, args: { ...ret.args, ...args } })
      void (async () => {
        try {
          const res = await fetch(data.url as string)
          if (!res.ok) {
            land({ error: `http ${res.status}` })
          } else if (data.as === "json") {
            land({ data: (await res.json()) as unknown })
          } else if (data.as === "text") {
            land({ data: await res.text() })
          } else {
            const bytes = new Uint8Array(await res.arrayBuffer())
            let bin = ""
            for (const b of bytes) bin += String.fromCharCode(b)
            land({ data: btoa(bin), mime: res.headers.get("content-type") ?? "application/octet-stream" })
          }
        } catch (err) {
          land({ error: String(err) })
        }
      })()
      break
    }
    default:
      console.warn(`unperformed effect: ${effect.kind}`)
  }
}
