import { h } from "../jsx.ts"
import type { Call, Designs, Node } from "../types.ts"

type Props = {
  data: Designs
  turn: (key: string, value: unknown) => Call
  onpick: (value: unknown) => Call
  close: Call
}

export function TilePicker({ data, turn, onpick, close }: Props): Node {
  const cfg = data.config
  const vocab = data.vocab
  return (
    <overlay key="sheet-tile" close={close}>
      <stack key="tile-sheet">
        <card key="pick">
          <grid key="groups" cols={vocab.groups.length}>
            {vocab.groups.map(name => (
              <button key={`g-${name}`} call={turn("group", name)} bg={name === cfg.group ? "var(--accent-color)" : undefined}>{name}</button>
            ))}
          </grid>
          <grid key="catalogs" cols={vocab.catalogs.length}>
            {vocab.catalogs.map(name => (
              <button key={`k-${name}`} call={turn("catalog", name)} bg={name === cfg.catalog ? "var(--accent-color)" : undefined}>{name}</button>
            ))}
          </grid>
          <grid key="page" cols={3}>
            <button key="prev" call={turn("page", Math.max(0, cfg.page - 1))}>←</button>
            <text key="counter" role="note">{`${cfg.page + 1} / ${vocab.pages}`}</text>
            <button key="next" call={turn("page", Math.min(vocab.pages - 1, cfg.page + 1))}>→</button>
          </grid>
        </card>
        <card key="thumbs">
          <grid key="thumb-grid" cols={4}>
            {data.designs.map((d, i) => (
              <cell key={`d-${i}`} call={onpick(d.value)}>
                <canvas key="thumb" handle={`sheet-tile-${i}`} rows={d.frame.rows} palette={d.frame.palette} />
              </cell>
            ))}
          </grid>
        </card>
      </stack>
    </overlay>
  )
}
