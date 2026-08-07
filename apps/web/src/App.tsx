import { useCallback, useEffect, useRef, useSyncExternalStore } from "react"
import {
  Box,
  Button,
  Footer,
  Frame,
  Header,
  Letters,
  Mark,
  Splash,
  Stack,
  Start,
  Toaster,
  sound,
  toast,
  useAttract,
} from "mrlyui"
import { Asker } from "./ask"
import type { Face } from "./boot"
import { hold } from "./components/Shot"
import { perform } from "./effects"
import { Kernel, Wire } from "./send"
import { useSplash } from "./splash"
import type { Manifest, Observation, Send } from "./types"
import { Fallback, views } from "./views/index"

const BEAT = 125

const ARROWS: Record<string, "up" | "down" | "left" | "right"> = {
  ArrowUp: "up",
  ArrowDown: "down",
  ArrowLeft: "left",
  ArrowRight: "right",
  w: "up",
  a: "left",
  s: "down",
  d: "right",
  W: "up",
  A: "left",
  S: "down",
  D: "right",
}

function arcade(apps: Manifest[], route: string): boolean {
  const category = apps.find(app => app.route === route)?.category
  return category === "games" || category === "puzzles"
}

function useBeat(obs: Observation, send: Send, paused: boolean): void {
  const pulse = obs.view?.beat
  const held = useRef(pulse)
  held.current = pulse
  useEffect(() => {
    if (paused || pulse === undefined) return
    const timer = setInterval(() => {
      const now = held.current
      if (now !== undefined) send(now, true)
    }, BEAT)
    return () => clearInterval(timer)
  }, [pulse === undefined, paused, send])
}

function useKeys(obs: Observation, apps: Manifest[], send: Send, paused: boolean): void {
  const app = obs.view?.app
  useEffect(() => {
    if (paused || app === undefined) return
    const keys = apps.find(one => one.route === app)?.keys
    if (keys === undefined) return
    const hear = (event: KeyboardEvent) => {
      if (event.ctrlKey || event.metaKey || event.altKey) return
      const tag = document.activeElement?.tagName ?? ""
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return
      const dir = ARROWS[event.key]
      if (dir === undefined) return
      const hit = keys[dir]
      if (hit === undefined) return
      event.preventDefault()
      send(hit)
    }
    window.addEventListener("keydown", hear)
    return () => window.removeEventListener("keydown", hear)
  }, [app, apps, paused, send])
}

function useNotices(obs: Observation): void {
  const seen = useRef(-1)
  const count = (obs.notices ?? []).length
  useEffect(() => {
    if (seen.current >= 0 && count > seen.current) {
      const notices = obs.notices ?? []
      const last = notices[notices.length - 1]
      if (last !== undefined) toast(last.body === "" ? last.title : `${last.title} · ${last.body}`)
    }
    seen.current = count
  }, [count, obs])
}

function useEffects(obs: Observation, send: Send): void {
  useEffect(() => {
    for (const effect of obs.effects ?? []) perform(effect, send)
    if (obs.last !== null && !obs.last.ok) sound.buzz(30)
  }, [obs, send])
}

export function App({ face }: { face: Face }) {
  const obs = useSyncExternalStore(face.subscribe, face.snapshot, face.snapshot)
  const shown = useSplash()
  const apps = face.registry.apps
  const slot = obs.view
  const route = slot?.app ?? ""

  const { idle, wake } = useAttract(!shown && arcade(apps, route), face.stray)

  const send = useCallback<Send>(
    (made, beat) => {
      if (beat !== true) wake()
      face.send(made, beat)
    },
    [face, wake],
  )

  useBeat(obs, send, shown || idle)
  useKeys(obs, apps, send, shown)
  useNotices(obs)
  useEffects(obs, send)

  useEffect(() => {
    window.scrollTo(0, 0)
  }, [route])

  useEffect(() => {
    const unlock = () => sound.unlock()
    window.addEventListener("pointerdown", unlock)
    return () => window.removeEventListener("pointerdown", unlock)
  }, [])

  const title = (apps.find(app => app.route === route)?.title ?? route).toUpperCase()
  const resets = slot?.actions.some(verb => verb.verb === `${route}.reset`) ?? false
  const Draw = views[route]

  return (
    <Wire.Provider value={send}>
      <Kernel.Provider value={face.registry}>
        <Header
          onMenu={() => send({ verb: "nav.open", args: { app: "menu" } })}
          onMark={() => send({ verb: "splash.on", args: {} })}
          onIden={() => send({ verb: "nav.open", args: { app: "iden" } })}
        />
        <Frame>
          <Stack>
            <Box>
              <Button
                wide
                onClick={() =>
                  send(resets ? { verb: `${route}.reset`, args: {} } : { verb: "nav.open", args: { app: route } })
                }
              >
                <Letters text={title} label={title} />
              </Button>
            </Box>
            <div ref={hold}>
              {slot === null || slot === undefined ? null : Draw === undefined ? (
                <Fallback app={slot.app} state={slot.state} />
              ) : (
                <Draw state={slot.state} />
              )}
            </div>
          </Stack>
          <Footer>
            <Mark />
          </Footer>
        </Frame>
        {idle ? (
          <Start
            onClick={() => {
              wake()
              if (route !== "") send({ verb: `${route}.reset`, args: {} })
            }}
          />
        ) : null}
        {shown ? <Splash onDismiss={() => send({ verb: "splash.off", args: {} })} /> : null}
        <Asker />
        <Toaster />
      </Kernel.Provider>
    </Wire.Provider>
  )
}
