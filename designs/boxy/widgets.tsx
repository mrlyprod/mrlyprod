import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BOXY, hash, tint } from "./cell.ts";

export function Widgets({ cards, editing, gallery, onEdit, onAdd, onRemove, bind }: Props["widgets"]) {
  const shown = new Set(cards.map(({ id }) => id));
  return (
    <section className="widgets" {...BOXY} aria-label="Widgets" {...bind.column()}>
      {cards.map(({ id, name, size, body, held }) => (
        <article
          key={id}
          className="widget"
          {...bind.card(id)()}
          data-size={size}
          data-held={held}
          style={tint(hash(id))}
        >
          <header className="head">
            {editing && (
              <button type="button" className="cell" aria-label={`Remove ${name}`} onClick={() => onRemove(id)}>
                x
              </button>
            )}
            <h2 className="cell">{name}</h2>
          </header>
          <div className="body cell">{body}</div>
        </article>
      ))}
      {editing && (
        <section className="gallery" aria-label="Add a widget">
          {gallery
            .filter(({ id }) => !shown.has(id))
            .map(({ id, name, size }) => (
              <button
                type="button"
                key={id}
                className="cell"
                data-size={size}
                style={tint(hash(id))}
                onClick={() => onAdd(id)}
              >
                + {name}
              </button>
            ))}
        </section>
      )}
      <button type="button" className="edit cell" aria-pressed={editing} onClick={() => onEdit(!editing)}>
        {editing ? "done" : "edit"}
      </button>
    </section>
  );
}
