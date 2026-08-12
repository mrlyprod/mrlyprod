import { useCallback } from "react"
import type { ReactNode } from "react"
import {
  Brand,
  Button,
  Chrome,
  Fold,
  Footer,
  Frame,
  Header,
  Panes,
  Setting,
  Stack,
  Symbol,
  THEME_ICONS,
  Toggle,
  go,
  useFont,
  useHead,
  usePanes,
  useTheme,
} from "mrlyui"
import { Panel } from "./Panel"
import type { Anchor } from "../lib/md"
import type { Ctx } from "../lib/repo"
import { routes } from "../lib/repo"
import { BASE, CLONE, PLACES, ROOT, SOCIALS } from "../lib/site"
import { href } from "../lib/tree"

type Props = {
  ctx: Ctx
  route: string
  current?: string
  title: string
  desc: string
  toc?: Anchor[]
  children: ReactNode
}

// SIDE

function Side({ toc }: { toc: Anchor[] }) {
  const [theme, cycle] = useTheme()
  const [font, swap] = useFont()
  return (
    <Stack tight>
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
      <Fold icon="settings" label="settings" open>
        <Setting label="theme">
          <Button onClick={cycle}>
            <Symbol name={THEME_ICONS[theme]} />
            {theme === "" ? "auto" : theme}
          </Button>
        </Setting>
        <Setting label="font">
          <Toggle value={font} onChange={swap} />
        </Setting>
      </Fold>
      <Fold icon="explore" label="places" grid>
        {PLACES.map(([name, at, icon]) => (
          <a key={name} href={at}>
            <Brand name={icon} />
            {name}
          </a>
        ))}
      </Fold>
      <Fold icon="groups" label="socials" grid>
        {SOCIALS.map(([name, at, icon]) => (
          <a key={name} href={at} target="_blank" rel="noopener">
            <Brand name={icon} />
            {name}
          </a>
        ))}
      </Fold>
    </Stack>
  )
}

// SHELL

export function Shell({ ctx, route, current, title, desc, toc = [], children }: Props) {
  useHead({ route, title, desc, root: ROOT, base: BASE })
  const panes = usePanes()
  const stray = useCallback(() => {
    const all = routes(ctx)
      .map(href)
      .filter(at => at !== location.pathname)
    go(all[Math.floor(Math.random() * all.length)] ?? "/")
  }, [ctx])
  return (
    <Chrome>
      <Header
        menu={panes.left.open}
        iden={panes.right.open}
        onMenu={panes.left.toggle}
        onMark={stray}
        onIden={panes.right.toggle}
        panes={panes}
      />
      <Panes
        panes={panes}
        left={<Panel ctx={ctx} current={current} />}
        right={<Side toc={toc} />}
        leftTitle="explorer"
        rightTitle="contents"
      >
        <Frame wide>
          <Stack>{children}</Stack>
          <Footer>
            <code className="clone">{CLONE}</code>
          </Footer>
        </Frame>
      </Panes>
    </Chrome>
  )
}
