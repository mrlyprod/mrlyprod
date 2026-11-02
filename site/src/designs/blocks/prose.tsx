import { Figure } from "../../lib/host.tsx";
import type { Props } from "../../shell/design.ts";
import { BLOCKS, hash, paint } from "./brick.ts";

/* PROSE */

export function Prose(props: Props["prose"]) {
  const { kind, id, title, lead, by, dates, figure, html, contents, cards, children, after } = props;
  if (kind === "lost" || kind === "error")
    return (
      <div className="prose" {...BLOCKS} data-kind={kind}>
        <div className="heap">
          <div className="big brick" style={paint(kind === "error" ? "red" : "black")}>
            <h1 id={id} className="tile">
              {title}
            </h1>
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
    <article className="prose plate" {...BLOCKS} style={paint("white")} data-kind={kind}>
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
        <h1 id={id}>{title}</h1>
        {post && by && <p className="by">{by}</p>}
        {post && dates && dates.length > 0 && <p className="by">{dates.join(" / ")}</p>}
        {lead && <p className="lead">{lead}</p>}
        {children}
        {html !== undefined && <div className="body" dangerouslySetInnerHTML={{ __html: html }} />}
        {cards && cards.length > 0 && (
          <div className="cards">
            {cards.map((card) => (
              <a key={card.href} className="card piece" href={card.href} style={paint(hash(card.href))}>
                <span className="brick">
                  <span className="lip studs" aria-hidden="true" />
                  <span className="face">{card.figure && <Figure name={card.figure} />}</span>
                </span>
                <span className="name tile">{card.name}</span>
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
