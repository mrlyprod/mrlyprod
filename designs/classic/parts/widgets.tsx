import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Widgets({ cards, editing, gallery, onEdit, onAdd, onRemove, bind }: Props["widgets"]) {
  return (
    <aside className="widgets" aria-label="Widgets" {...bind.column()}>
      {cards.map((card) => (
        <section
          key={card.id}
          className={classes("raised", "widget", card.size, card.held ? "held" : undefined)}
          {...bind.card(card.id)()}
        >
          <div className="title">
            <span className="name">{card.name}</span>
            <span className="actions">
              <button
                type="button"
                className="raised knob"
                aria-label={`Remove ${card.name}`}
                hidden={!editing}
                onClick={() => onRemove(card.id)}
              >
                <Glyph art="close" />
              </button>
            </span>
          </div>
          <div className="body">{card.body}</div>
        </section>
      ))}
      {editing ? (
        <div className="raised gallery" role="dialog" aria-label="Add widget">
          <div className="title">
            <span className="name">Add widget</span>
          </div>
          <ul className="sunken list">
            {gallery.map((one) => (
              <li key={one.id}>
                <button type="button" onClick={() => onAdd(one.id)}>
                  {one.name}
                </button>
              </li>
            ))}
          </ul>
          <div className="buttons">
            <button type="button" className="raised primary" onClick={() => onEdit(false)}>
              Done
            </button>
          </div>
        </div>
      ) : (
        <div className="buttons">
          <button type="button" className="raised" onClick={() => onEdit(true)}>
            Edit
          </button>
        </div>
      )}
    </aside>
  );
}
