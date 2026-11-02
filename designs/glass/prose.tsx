import { Figure } from "../../net/sdk/host.tsx";
import type { Props } from "../../net/shell/design.ts";

/* PROSE */

export function Prose({ kind, id, title, level, lead, by, figure, html, children, after }: Props["prose"]) {
  const post = kind === "post";
  const Head = level === 2 ? "h2" : "h1";
  return (
    <>
      {(post || figure) && <Figure className="opener" name={figure!} label={title} />}
      <div className={post ? "plate" : "lede"}>
        <Head id={id}>{title}</Head>
        {post && <p className="by">{by}</p>}
        {lead && <p className="lead">{lead}</p>}
        {children}
      </div>
      {html !== undefined && <div dangerouslySetInnerHTML={{ __html: html }} />}
      {after}
    </>
  );
}
