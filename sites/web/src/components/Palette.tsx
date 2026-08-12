import { ColorPicker } from "mrlyui"
import { call } from "../builders"
import { hex, names } from "../palette"
import { reading } from "../reads"
import { useSend } from "../send"

export function Palette({ app, colors }: { app: string; colors: string[] }) {
  const send = useSend()
  const lib = (reading("colors/library") as string[] | null) ?? []
  const pool = lib.length > 0 ? lib : names()
  const worn = new Set(colors.map(one => one.toLowerCase()))
  const swatches = pool.map(one => ({ name: one, color: hex(one) }))
  const value = pool.filter(one => worn.has(hex(one).toLowerCase()))

  const pick = (one: string | null) => {
    if (one === null) return
    const swatch = hex(one)
    const next = worn.has(swatch.toLowerCase())
      ? colors.filter(c => c.toLowerCase() !== swatch.toLowerCase())
      : [...colors, swatch]
    send(call(`${app}.set`, { key: "palette", value: next }))
  }

  return <ColorPicker swatches={swatches} value={value} onChange={pick} />
}
