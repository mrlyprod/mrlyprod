import { Figure } from "@mrlyprod/ui/kit/host.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { cx } from "./kit.ts";
import { Sprite } from "./sprite.tsx";

type Card = NonNullable<Props["prose"]["cards"]>[number];

function Listing({ cards }: { cards: Card[] }) {
  return (
    <div className="listing">
      {cards.map((card) => (
        <a key={card.href} className="card" href={card.href}>
          {card.figure ? (
            <span className="art">
              <Figure name={card.figure} />
            </span>
          ) : (
            <span className="art blank" aria-hidden="true">
              <Sprite art="ghost" size={32} />
            </span>
          )}
          <span className="name">{card.name}</span>
          {card.text && <span className="text">{card.text}</span>}
        </a>
      ))}
    </div>
  );
}

export function Prose(props: Props["prose"]) {
  const { kind, id, title, level, lead, by, dates = [], figure, html, contents, cards, children, after } = props;
  const Head = level === 2 ? "h2" : "h1";
  if (kind === "lost" || kind === "error")
    return (
      <div className={cx("pixel", "prose", kind)}>
        <div className="over frame" role="alert">
          <Sprite art="ghost" size={48} />
          <p className="game">{kind === "lost" ? "GAME OVER" : "TILT"}</p>
          <Head id={id}>{title}</Head>
          {lead && <p className="lead">{lead}</p>}
          {children}
        </div>
        {after}
      </div>
    );
  return (
    <article className={cx("pixel", "prose", kind)}>
      <Head id={id}>{title}</Head>
      {(by || dates.length > 0) && <p className="by">{[by, ...dates].filter(Boolean).join(" \u00b7 ")}</p>}
      {contents.length > 0 && (
        <ul className="contents">
          {contents.map((one) => (
            <li key={one.id} className={`level${one.level}`}>
              <a href={`#${one.id}`} aria-current={one.on ? "location" : undefined}>
                {one.text}
              </a>
            </li>
          ))}
        </ul>
      )}
      {figure && <Figure className="opener" name={figure} label={title} />}
      {(lead || children) && (
        <div className="intro">
          {lead && <p className="lead">{lead}</p>}
          {children}
        </div>
      )}
      {html !== undefined && <div className="body" dangerouslySetInnerHTML={{ __html: html }} />}
      {cards && cards.length > 0 && <Listing cards={cards} />}
      {after}
    </article>
  );
}
