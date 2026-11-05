import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BRICKS, hash, paint } from "./brick.ts";

export function Widgets({ cards, editing, gallery, onEdit, onAdd, onRemove, bind }: Props["widgets"]) {
  const shown = new Set(cards.map(({ id }) => id));
  return (
    <section className="widgets" {...BRICKS} data-editing={editing} aria-label="Widgets" {...bind.column()}>
      {cards.map(({ id, name, size, body, held }) => (
        <article
          key={id}
          className="widget plate"
          {...bind.card(id)()}
          data-size={size}
          data-held={held}
          style={paint(hash(id))}
        >
          <header className="lip studs">
            <h2 className="head tile">{name}</h2>
            {editing && (
              <button
                type="button"
                className="remove brick"
                style={paint("red")}
                aria-label={`Remove ${name}`}
                onClick={() => onRemove(id)}
              >
                <Icon name="close" size={20} />
              </button>
            )}
          </header>
          <div className="body tile">{body}</div>
        </article>
      ))}
      {editing && (
        <section className="gallery plate" style={paint("white")} aria-label="Add a widget">
          {gallery
            .filter(({ id }) => !shown.has(id))
            .map(({ id, name, size }) => (
              <button
                type="button"
                key={id}
                className="brick"
                data-size={size}
                style={paint(hash(id))}
                onClick={() => onAdd(id)}
              >
                <span className="tile">{name}</span>
              </button>
            ))}
        </section>
      )}
      <button
        type="button"
        className="edit brick"
        style={paint("white")}
        aria-pressed={editing}
        onClick={() => onEdit(!editing)}
      >
        {editing ? "Done" : "Edit"}
      </button>
    </section>
  );
}
