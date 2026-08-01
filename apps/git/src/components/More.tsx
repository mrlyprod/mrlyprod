export function More({ href }: { href: string }) {
  return (
    <details className="more">
      <summary aria-label="more">
        <span className="ic" aria-hidden="true">
          more_vert
        </span>
      </summary>
      <div className="sheet">
        <a href={href}>
          <span className="ic" aria-hidden="true">
            raw_on
          </span>
          raw
        </a>
        <button className="copy" type="button" data-raw={href}>
          <span className="ic" aria-hidden="true">
            content_copy
          </span>
          <span className="word">copy</span>
        </button>
      </div>
    </details>
  )
}
