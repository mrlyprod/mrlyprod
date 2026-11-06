import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Window({ bind }: Props["window"]) {
  return (
    <section className="window" aria-label="Page">
      {"\n"}
      <div className="panes">
        {"\n"}
        <main {...bind.main()} />
        {"\n"}
        <aside className="pane right" {...bind.pane()} aria-label="Page tools" hidden />
        {"\n"}
      </div>
      {"\n"}
      <a className="home" href={href(bind.close)} {...bind.close()} aria-label="Close" title="Close" />
    </section>
  );
}
