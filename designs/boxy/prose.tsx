import { Figure } from "@mrlyprod/ui/kit/host.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BOXY, hash, tint } from "./cell.ts";

export function Prose(props: Props["prose"]) {
  const { kind, id, title, level, lead, by, dates, figure, html, contents, cards, children, after } = props;
  const Head = level === 2 ? "h2" : "h1";
  if (kind === "lost" || kind === "error")
    return (
      <div className="prose" {...BOXY} data-kind={kind}>
        <div className="frame">
          <Head id={id} className="cell">
            {title}
          </Head>
        </div>
        {lead && <p className="card">{lead}</p>}
        {children}
        {after}
      </div>
    );
  const post = kind === "post";
  return (
    <article className="prose" {...BOXY} data-kind={kind}>
      {(post || figure) && <Figure className="opener" name={figure ?? ""} label={title} />}
      <Head id={id}>{title}</Head>
      {(lead || post || children) && (
        <div className="card lede">
          {post && by && <p className="by">{by}</p>}
          {post && dates && dates.length > 0 && <p className="by">{dates.join(" / ")}</p>}
          {lead && <p className="lead">{lead}</p>}
          {children}
        </div>
      )}
      {contents.length > 0 && (
        <nav className="contents" aria-label="Contents">
          {contents.map((one) => (
            <a
              key={one.id}
              className="cell"
              href={`#${one.id}`}
              data-level={one.level}
              aria-current={one.on ? "location" : undefined}
            >
              {one.text}
            </a>
          ))}
        </nav>
      )}
      {html !== undefined && <div className="card body" dangerouslySetInnerHTML={{ __html: html }} />}
      {cards && cards.length > 0 && (
        <div className="cards">
          {cards.map((card) => (
            <a key={card.href} className="tile" href={card.href} style={tint(hash(card.href))}>
              {card.figure ? (
                <Figure className="art cell" name={card.figure} />
              ) : (
                <span className="art cell" aria-hidden="true">
                  {card.name.slice(0, 1).toLowerCase()}
                </span>
              )}
              <span className="name cell">{card.name.toLowerCase()}</span>
              {card.text && <span className="text cell">{card.text}</span>}
            </a>
          ))}
        </div>
      )}
      {after}
    </article>
  );
}
