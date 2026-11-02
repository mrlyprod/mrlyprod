import { Figure } from "../../net/sdk/host.tsx";
import type { Props } from "../../net/shell/design.ts";
import { BRICKS, hash, paint } from "./brick.ts";

/* PROSE */

export function Prose(props: Props["prose"]) {
  const { kind, id, title, level, lead, by, dates, figure, html, contents, cards, children, after } = props;
  const Head = level === 2 ? "h2" : "h1";
  if (kind === "lost" || kind === "error")
    return (
      <div className="prose" {...BRICKS} data-kind={kind}>
        <div className="heap">
          <div className="big brick" style={paint(kind === "error" ? "red" : "black")}>
            <Head id={id} className="tile">
              {title}
            </Head>
          </div>
          <span className="tumble brick" style={paint("yellow")} aria-hidden="true" />
        </div>
        {lead && <p className="lead tile">{lead}</p>}
        {children}
        {after}
      </div>
    );
  const post = kind === "post";
  return (
    <article className="prose plate" {...BRICKS} style={paint("white")} data-kind={kind}>
      <div className="lip studs" aria-hidden="true" />
      {contents.length > 0 && (
        <nav className="contents" aria-label="Contents">
          {contents.map((one) => (
            <a
              key={one.id}
              className="brick"
              style={paint(one.on ? "blue" : "white")}
              href={`#${one.id}`}
              data-level={one.level}
              aria-current={one.on ? "location" : undefined}
            >
              {one.text}
            </a>
          ))}
        </nav>
      )}
      <div className="sheet tile">
        {(post || figure) && <Figure className="opener" name={figure ?? ""} label={title} />}
        <Head id={id}>{title}</Head>
        {post && by && <p className="by">{by}</p>}
        {post && dates && dates.length > 0 && <p className="by">{dates.join(" / ")}</p>}
        {(lead || children) && (
          <div className="intro" style={paint("yellow")}>
            {lead && <p className="lead">{lead}</p>}
            {children}
          </div>
        )}
        {html !== undefined && <div className="body" dangerouslySetInnerHTML={{ __html: html }} />}
        {cards && cards.length > 0 && (
          <div className="cards">
            {cards.map((card) => (
              <a key={card.href} className="card" href={card.href} style={paint(hash(card.href))}>
                {card.figure && <Figure className="face" name={card.figure} />}
                <span className="name">{card.name}</span>
                {card.text && <span className="text">{card.text}</span>}
              </a>
            ))}
          </div>
        )}
        {after}
      </div>
    </article>
  );
}
