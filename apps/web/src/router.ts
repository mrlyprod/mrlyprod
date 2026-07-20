import type { Call, Observation } from "./types.ts"

type Handler = (call: Call) => void
type Observer = (call: Call, obs: Observation) => void

export type Router = {
  on: (verb: string, handler: Handler) => void
  after: (verb: string, observer: Observer) => void
  handle: (call: Call) => boolean
  observe: (call: Call, obs: Observation) => void
}

export function router(): Router {
  const handlers = new Map<string, Handler>()
  const observers = new Map<string, Observer>()
  const find = (verb: string): Handler | undefined => {
    const exact = handlers.get(verb)
    if (exact !== undefined) return exact
    for (const [key, handler] of handlers) {
      if (key.endsWith(".") && verb.startsWith(key)) return handler
    }
    return undefined
  }
  return {
    on: (verb, handler) => handlers.set(verb, handler),
    after: (verb, observer) => observers.set(verb, observer),
    handle: call => {
      const handler = find(call.verb)
      if (handler === undefined) return false
      handler(call)
      return true
    },
    observe: (call, obs) => observers.get(call.verb)?.(call, obs),
  }
}
