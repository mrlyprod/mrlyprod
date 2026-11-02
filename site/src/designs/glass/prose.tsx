import { Figure } from "../../lib/host.tsx";
import type { Props } from "../../shell/design.ts";

/* PROSE */

export function Prose({ kind, id, title, lead, by, figure, html, children, after }: Props["prose"]) {
  const post = kind === "post";
  return (
    <>
      {(post || figure) && <Figure className="opener" name={figure!} label={title} />}
      <div className={post ? "plate" : "lede"}>
        <h1 id={id}>{title}</h1>
        {post && <p className="by">{by}</p>}
        {lead && <p className="lead">{lead}</p>}
        {children}
      </div>
      {html !== undefined && <div dangerouslySetInnerHTML={{ __html: html }} />}
      {after}
    </>
  );
}
