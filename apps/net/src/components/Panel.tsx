import { HELP, LEAD } from "../lib/site"

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
      <p className="lead">Order yours.</p>
      <p className="act">
        <a className="build" href={HELP}>
          Get in touch
        </a>
      </p>
      <p className="fine">{LEAD}</p>
    </div>
  )
}
