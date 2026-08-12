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
import { Menu } from "./Menu"
import type { Site } from "../lib/data"
import type { Anchor } from "../lib/md"
import { BASE, PLACES, ROOT, SOCIALS } from "../lib/site"

type Props = {
  site: Site
  route: string
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
            {toc.map(anchor => (
              <li key={anchor.id}>
                <a href={`#${anchor.id}`}>{anchor.text}</a>
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

export function Shell({ site, route, title, desc, toc = [], children }: Props) {
  useHead({ route, title, desc, root: ROOT, base: BASE })
  const panes = usePanes()
  const home = useCallback(() => {
    go("/")
  }, [])
  return (
    <Chrome>
      <Header
        menu={panes.left.open}
        iden={panes.right.open}
        onMenu={panes.left.toggle}
        onMark={home}
        onIden={panes.right.toggle}
        panes={panes}
      />
      <Panes
        panes={panes}
        left={<Menu site={site} route={route} />}
        right={<Side toc={toc} />}
        leftTitle="menu"
        rightTitle="contents"
      >
        <Frame>
          <Stack>{children}</Stack>
          <Footer />
        </Frame>
      </Panes>
    </Chrome>
  )
}
