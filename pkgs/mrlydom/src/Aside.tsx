import type { ReactNode, Ref } from "react"
import { Brand } from "./Brand"
import { Fold } from "./Fold"
import { ICONS, useFont, useTheme } from "./theme"

export type Section = { id: string; text: string }

export type Link = readonly [string, string, string]

type Props = {
  toc: Section[]
  places: Link[]
  socials: Link[]
  shut?: boolean
  ref?: Ref<HTMLElement>
  children?: ReactNode
}

export function Aside({ toc, places, socials, shut = false, ref, children }: Props) {
  const [theme, cycle] = useTheme()
  const font = useFont()
  return (
    <aside
      className={shut ? "pane right shut" : "pane right"}
      id="aux"
      popover="auto"
      aria-label="contents"
      ref={ref}
    >
      {toc.length > 1 && (
        <Fold icon="toc" label="contents" open>
          <ol className="toc">
            {toc.map(section => (
              <li key={section.id}>
                <a href={`#${section.id}`}>{section.text}</a>
              </li>
            ))}
          </ol>
        </Fold>
      )}
      {children}
      <Fold icon="settings" label="settings" grid>
        <button id="theme-toggle" type="button" onClick={cycle}>
          <span className="ic" aria-hidden="true">
            {ICONS[theme] ?? ""}
          </span>
          theme
        </button>
        <button id="font-toggle" type="button" onClick={font}>
          <span className="ic" aria-hidden="true">
            custom_typography
          </span>
          font
        </button>
      </Fold>
      <Fold icon="explore" label="places" grid>
        {places.map(([name, href, icon]) => (
          <a key={name} href={href}>
            <Brand name={icon} />
            {name}
          </a>
        ))}
      </Fold>
      <Fold icon="groups" label="socials" grid>
        {socials.map(([name, href, icon]) => (
          <a key={name} href={href} target="_blank" rel="noopener">
            <Brand name={icon} />
            {name}
          </a>
        ))}
      </Fold>
    </aside>
  )
}
