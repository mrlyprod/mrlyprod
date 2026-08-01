import { write } from "mrlydom"
import { BUILD, HELP, LEAD, PRESET } from "../lib/site"

export function Panel({ preset }: { preset?: string }) {
  if (preset === undefined) {
    return (
      <div className="buy">
        <p className="lead">Coming soon.</p>
        <p className="fine">
          <a href={HELP}>Want it sooner? Say hello.</a>
        </p>
      </div>
    )
  }
  return (
    <div className="buy">
      <p className="lead">Design yours.</p>
      <p className="act">
        <a className="build" href={`/${BUILD}`} data-preset={preset} onClick={() => write(PRESET, preset)}>
          Open the designer
        </a>
      </p>
      <p className="fine">{LEAD}</p>
      <p className="fine">
        <a href={HELP}>Questions? Say hello.</a>
      </p>
    </div>
  )
}
