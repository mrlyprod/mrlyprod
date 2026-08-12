import { useState } from "react"
import { Button, Cluster, Slider, Symbol } from "mrlyui"
import { call } from "../builders"
import { useSend } from "../send"
import { useTime } from "../time"

export function Transport() {
  const { at, span } = useTime()
  const send = useSend()
  const [draft, setDraft] = useState<number | null>(null)
  const seek = (to: number) => send(call("journal.seek", { to }))
  return (
    <>
      <Cluster>
        <Button disabled={at === 0} onClick={() => seek(0)}>
          <Symbol name="keyboard_double_arrow_left" />
        </Button>
        <Button disabled={at === 0} onClick={() => seek(at - 1)}>
          <Symbol name="chevron_left" />
        </Button>
        <Button disabled={at === span} onClick={() => seek(at + 1)}>
          <Symbol name="chevron_right" />
        </Button>
        <Button disabled={at === span} onClick={() => seek(span)}>
          <Symbol name="keyboard_double_arrow_right" />
        </Button>
      </Cluster>
      <Slider
        min={0}
        max={span}
        step={1}
        value={draft ?? at}
        onChange={setDraft}
        onCommit={to => {
          setDraft(null)
          if (to !== at) seek(to)
        }}
        format={v => `${v} / ${span}`}
      />
    </>
  )
}
