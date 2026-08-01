import { call, raster } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Call, Node, Raster, Send } from "../../types.ts"

type State = { display: string; glyph?: Raster }

export function calculator(state: unknown, _send: Send): Node {
  const s = state as State
  const verb = (name: string) => call(`calculator.${name}`)
  const key = (k: string, glyph: string, action: Call) => (
    <label key={k} mode="icon" symbol={{ as: "glyph", value: glyph }} text={glyph} call={action} />
  )
  const digit = (d: number) => key(`d${d}`, String(d), call("calculator.digit", { d }))
  const op = (name: string, glyph: string) => key(name, glyph, call("calculator.op", { op: name }))
  return (
    <stack key="calculator">
      <card key="panel">
        <stack key="pad">
          {s.glyph !== undefined ? (
            raster("readout", "calculator", s.glyph)
          ) : (
            <label key="readout" mode="icon" symbol={{ as: "glyph", value: s.display }} text={s.display} call={verb("copy")} />
          )}
          <grid key="keys" cols={4}>
            {key("clear", "AC", verb("clear"))}
            {key("negate", "+/-", verb("negate"))}
            {key("percent", "%", verb("percent"))}
            {op("div", "÷")}
            {digit(7)}
            {digit(8)}
            {digit(9)}
            {op("mul", "×")}
            {digit(4)}
            {digit(5)}
            {digit(6)}
            {op("sub", "−")}
            {digit(1)}
            {digit(2)}
            {digit(3)}
            {op("add", "+")}
          </grid>
          <grid key="base" cols={2}>
            {digit(0)}
            <grid key="tail" cols={2}>
              {key("dot", ".", verb("dot"))}
              {key("equals", "=", verb("equals"))}
            </grid>
          </grid>
        </stack>
      </card>
    </stack>
  )
}
