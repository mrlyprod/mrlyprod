import { call } from "../builders.ts"
import type { Call, Manifest, Node, Send } from "../types.ts"
import { bang } from "./math/bang.tsx"
import { billiards } from "./physics/billiards.tsx"
import { calculator } from "./tools/calculator.tsx"
import { calendar } from "./tools/calendar.tsx"
import { captcha } from "./puzzles/captcha.tsx"
import { chess } from "./puzzles/chess.tsx"
import { clock } from "./tools/clock.tsx"
import { colors } from "./design/colors.tsx"
import { emoji } from "./design/emoji.tsx"
import { crush } from "./games/crush.tsx"
import { dice } from "./tools/dice.tsx"
import { escape } from "./games/escape.tsx"
import { extras } from "./company/extras.tsx"
import { files } from "./system/files.tsx"
import { font } from "./design/font.tsx"
import { hash } from "./tools/hash.tsx"
import { menu } from "./system/menu.tsx"
import { iden } from "./system/iden.tsx"
import { lasers } from "./physics/lasers.tsx"
import { julia } from "./toys/julia.tsx"
import { life } from "./math/life.tsx"
import { log } from "./system/log.tsx"
import { mandelbrot } from "./toys/mandelbrot.tsx"
import { matrix } from "./toys/matrix.tsx"
import { memory } from "./puzzles/memory.tsx"
import { mines } from "./puzzles/mines.tsx"
import { moire } from "./math/moire.tsx"
import { notes } from "./creativity/notes.tsx"
import { pages } from "./company/pages.tsx"
import { photos } from "./creativity/photos.tsx"
import { piano } from "./creativity/piano.tsx"
import { pixel } from "./design/pixel.tsx"
import { quiz } from "./puzzles/quiz.tsx"
import { settings } from "./system/settings.tsx"
import { six } from "./math/six.tsx"
import { sleep } from "./toys/sleep.tsx"
import { snake } from "./games/snake.tsx"
import { solids } from "./toys/solids.tsx"
import { tennis } from "./games/tennis.tsx"
import { text } from "./design/text.tsx"
import { three } from "./math/three.tsx"
import { tile } from "./math/tile.tsx"
import { timer } from "./tools/timer.tsx"
import { ttt } from "./puzzles/ttt.tsx"
import { twenty48 } from "./puzzles/twenty48.tsx"
import { two } from "./math/two.tsx"
import { ui } from "./system/ui.tsx"
import { waves } from "./physics/waves.tsx"

export type View = (state: unknown, send: Send) => Node

export const views: Record<string, View> = {
  menu,
  iden,
  calculator,
  notes,
  solids,
  ui,
  life,
  clock,
  timer,
  calendar,
  dice,
  pages,
  photos,
  snake,
  julia,
  mandelbrot,
  matrix,
  sleep,
  ttt,
  memory,
  mines,
  twenty48,
  crush,
  tennis,
  text,
  escape,
  quiz,
  captcha,
  pixel,
  settings,
  chess,
  font,
  two,
  three,
  bang,
  tile,
  six,
  waves,
  billiards,
  lasers,
  moire,
  hash,
  colors,
  emoji,
  piano,
  extras,
  log,
  files,
}

export type KeySet = Partial<Record<"up" | "down" | "left" | "right", Call>>

const turns = (verb: string): KeySet => ({
  up: call(verb, { dir: "up" }),
  down: call(verb, { dir: "down" }),
  left: call(verb, { dir: "left" }),
  right: call(verb, { dir: "right" }),
})

export const keys: Record<string, KeySet> = {
  snake: turns("snake.turn"),
  twenty48: turns("twenty48.slide"),
  escape: turns("escape.turn"),
  tennis: {
    left: call("tennis.move", { dir: "left" }),
    right: call("tennis.move", { dir: "right" }),
  },
  crush: {
    up: call("crush.crush"),
    down: call("crush.drop"),
    left: call("crush.move", { dir: "left" }),
    right: call("crush.move", { dir: "right" }),
  },
}

export function fallback(app: string, state: unknown, manifest?: Manifest): Node {
  return {
    kind: "Card",
    key: `${app}-raw`,
    children: [
      { kind: "Symbol", key: "raw-emoji", as: "emoji", value: manifest?.emoji ?? "❓" },
      { kind: "Text", key: "raw-title", text: manifest?.title ?? app, role: "title" },
      { kind: "Text", key: "raw-state", text: JSON.stringify(state, null, 2), role: "note" },
    ],
  }
}
