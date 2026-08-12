import type { Args, Call } from "./types"

export const call = (verb: string, args: Args = {}): Call => ({ verb, args })

export const set = (app: string, key: string, value: unknown): Call => ({
  verb: `${app}.set`,
  args: { key, value },
})

export const setter = (app: string) => (key: string, value: unknown): Call => set(app, key, value)
