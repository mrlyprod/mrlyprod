import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Window({ bind }: Props["window"]) {
  return (
    <section className="window raised" aria-label="Page">
      <div className="title">
        <Glyph art="page" size={14} />
        <span className="name" {...bind.title()} />
        <span className="actions">
          <a className="raised knob close" href={href(bind.close)} {...bind.close()} aria-label="Close">
            <Glyph art="close" />
          </a>
        </span>
      </div>
      <nav className="menubar" aria-label="App">
        <span className="menus" role="menubar" {...bind.menus()} />
      </nav>
      <div className="panes">
        <main className="sunken well" {...bind.main()} />
        <aside className="raised pane right" {...bind.pane()} aria-label="Page tools" hidden>
          <div className="title">
            <span className="name">Tools</span>
          </div>
        </aside>
      </div>
    </section>
  );
}
