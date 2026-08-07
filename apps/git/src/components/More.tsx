import { useState } from "react"
import { Button, Popover, Symbol } from "mrlyui"

// COPY

async function grab(href: string): Promise<void> {
  const body = await fetch(href).then(res => res.text())
  await navigator.clipboard.writeText(body)
}

// MORE

export function More({ href }: { href: string }) {
  const [done, setDone] = useState(false)
  const copy = () => {
    void grab(href)
      .then(() => {
        setDone(true)
        setTimeout(() => setDone(false), 1500)
      })
      .catch(() => {})
  }
  return (
    <Popover
      align="right"
      trigger={({ toggle }) => (
        <Button onClick={toggle}>
          <Symbol name="more_vert" />
        </Button>
      )}
    >
      <div className="menu" role="menu">
        <a className="menu-item" role="menuitem" href={href}>
          <span className="menu-item-icon">
            <Symbol name="raw_on" size="var(--font-size)" />
          </span>
          <span className="menu-item-label">raw</span>
        </a>
        <button className="menu-item" type="button" role="menuitem" onClick={copy}>
          <span className="menu-item-icon">
            <Symbol name="content_copy" size="var(--font-size)" />
          </span>
          <span className="menu-item-label">{done ? "copied" : "copy"}</span>
        </button>
      </div>
    </Popover>
  )
}
