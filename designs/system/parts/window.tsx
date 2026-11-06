import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { tint } from "../own/cell.ts";

export function Window({ bind }: Props["window"]) {
  return (
    <section className="window" data-hue="gray" style={tint("gray")} aria-label="Page">
      <div className="title">
        <a className="close cell" href={href(bind.close)} {...bind.close()} aria-label="Close">
          x
        </a>
        <span className="name cell" aria-hidden="true" {...bind.title()} />
      </div>
      <div className="panes">
        <main {...bind.main()} />
        <aside className="pane right" {...bind.pane()} aria-label="Page tools" hidden />
      </div>
    </section>
  );
}
