import { Button, ColorPicker, Grid } from "mrlyui"
import { call } from "../builders"
import { Cells } from "../eyes/Cells"
import { hex } from "../palette"
import { reading } from "../reads"
import { useSend } from "../send"
import type { Cells as Deck } from "../types"

type Slab = { id: number; name: string; value: unknown; cells: Deck }

export function Library({ kind, host, name, current }: {
  kind: "colors" | "emoji" | "font" | "tile"
  host: string
  name: string
  current?: unknown
}) {
  const send = useSend()
  const lib = reading(`${kind}/library`)
  if (!Array.isArray(lib) || lib.length === 0) return null
  const set = (value: unknown) => send(call(`${host}.set`, { key: name, value }))

  if (kind === "colors") {
    const swatches = (lib as string[]).map(one => ({ name: one, color: hex(one) }))
    return (
      <ColorPicker
        big
        swatches={swatches}
        value={typeof current === "string" ? current : null}
        onChange={one => set(one)}
      />
    )
  }

  if (kind === "emoji" || kind === "font") {
    return (
      <Grid cols={8}>
        {(lib as string[]).map(value => (
          <Button key={value} onClick={() => set(value)}>
            {value}
          </Button>
        ))}
      </Grid>
    )
  }

  return (
    <Grid cols={3}>
      {(lib as Slab[]).map(entry => (
        <Button key={entry.id} onClick={() => set(entry.value)}>
          <Cells app="tile" cells={entry.cells} handle={`lib-${host}-${name}-${entry.id}`} />
        </Button>
      ))}
    </Grid>
  )
}
