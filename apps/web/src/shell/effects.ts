import { buzz, play, start, stop } from "../sound.ts"
import type { Args, Effect, Send } from "../types.ts"

export function perform(effect: Effect, emit: Send): void {
  switch (effect.kind) {
    case "notify": {
      const data = effect.data as { title?: string; body?: string }
      buzz([30, 50, 30])
      if ("Notification" in window && Notification.permission === "granted") {
        new Notification(data.title ?? "", { body: data.body ?? "" })
      }
      break
    }
    case "sound": {
      const data = effect.data as { op?: string; id?: string; freq?: number; wave?: string; ms?: number; gain?: number }
      if (data.op === "note" && data.freq !== undefined) play(data.freq, data.wave, data.ms, data.gain)
      else if (data.op === "start" && data.id !== undefined && data.freq !== undefined) start(data.id, data.freq, data.wave, data.gain)
      else if (data.op === "stop" && data.id !== undefined) stop(data.id)
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
