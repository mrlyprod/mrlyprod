import { Figure } from "@mrlyprod/ui/kit/host.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Prose({ kind, id, title, level, lead, by, figure, html, cards, children, after }: Props["prose"]) {
  const post = kind === "post";
  const hub = kind === "hub";
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
      {html !== undefined && <div className={hub ? "hub" : undefined} dangerouslySetInnerHTML={{ __html: html }} />}
      {cards && cards.length > 0 && (
        <div className="kids">
          {cards.map((card) => (
            <a key={card.href} href={card.href}>
              {card.figure ? (
                <Figure className="art" name={card.figure} />
              ) : (
                <span className="art" aria-hidden="true">
                  {card.name.slice(0, 1)}
                </span>
              )}
              <span className="name">{card.name}</span>
              {card.text && <span className="text">{card.text}</span>}
            </a>
          ))}
        </div>
      )}
      {after}
    </>
  );
}
