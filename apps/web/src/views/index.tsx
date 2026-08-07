import type { ComponentType } from "react"
import {
  Box,
  Caption,
  Emoji,
  Title,
} from "mrlyui"
import { useApp } from "../send"
import { Bang } from "./math/bang"
import { Calculator } from "./tools/calculator"
import { Calendar } from "./tools/calendar"
import { Captcha } from "./puzzles/captcha"
import { Chess } from "./puzzles/chess"
import { Clock } from "./tools/clock"
import { Colors } from "./design/colors"
import { Crush } from "./games/crush"
import { Dice } from "./tools/dice"
import { Emojis } from "./design/emoji"
import { Escape } from "./games/escape"
import { Files } from "./system/files"
import { Font } from "./design/font"
import { Hash } from "./tools/hash"
import { Iden } from "./system/iden"
import { Julia } from "./toys/julia"
import { Life } from "./math/life"
import { Log } from "./system/log"
import { Mandelbrot } from "./toys/mandelbrot"
import { Matrix } from "./toys/matrix"
import { Memory } from "./puzzles/memory"
import { Menu } from "./system/menu"
import { Mines } from "./puzzles/mines"
import { Moire } from "./math/moire"
import { Notes } from "./creativity/notes"
import { Photos } from "./creativity/photos"
import { Piano } from "./creativity/piano"
import { Pixel } from "./design/pixel"
import { Quiz } from "./puzzles/quiz"
import { Settings } from "./system/settings"
import { Six } from "./math/six"
import { Sleep } from "./toys/sleep"
import { Snake } from "./games/snake"
import { Solids } from "./toys/solids"
import { Tennis } from "./games/tennis"
import { Three } from "./math/three"
import { Tile } from "./math/tile"
import { Timer } from "./tools/timer"
import { Ttt } from "./puzzles/ttt"
import { Twenty48 } from "./puzzles/twenty48"
import { Two } from "./math/two"
import { Ui } from "./system/ui"

export type View = ComponentType<{ state: unknown }>

export const views: Record<string, View> = {
  menu: Menu,
  iden: Iden,
  calculator: Calculator,
  notes: Notes,
  solids: Solids,
  ui: Ui,
  life: Life,
  clock: Clock,
  timer: Timer,
  calendar: Calendar,
  dice: Dice,
  photos: Photos,
  snake: Snake,
  julia: Julia,
  mandelbrot: Mandelbrot,
  matrix: Matrix,
  sleep: Sleep,
  ttt: Ttt,
  memory: Memory,
  mines: Mines,
  twenty48: Twenty48,
  crush: Crush,
  tennis: Tennis,
  escape: Escape,
  quiz: Quiz,
  captcha: Captcha,
  pixel: Pixel,
  settings: Settings,
  chess: Chess,
  font: Font,
  two: Two,
  three: Three,
  bang: Bang,
  tile: Tile,
  six: Six,
  moire: Moire,
  hash: Hash,
  colors: Colors,
  emoji: Emojis,
  piano: Piano,
  log: Log,
  files: Files,
}

export function Fallback({ app, state }: { app: string; state: unknown }) {
  const manifest = useApp(app)
  return (
    <Box>
      <Emoji glyph={manifest?.emoji ?? "❓"} />
      <Title>{manifest?.title ?? app}</Title>
      <Caption>{JSON.stringify(state, null, 2)}</Caption>
    </Box>
  )
}
