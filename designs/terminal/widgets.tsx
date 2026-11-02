import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";

/* WIDGETS */

export function Widgets({ cards, editing, gallery, onEdit, onAdd, onRemove, bind }: Props["widgets"]) {
  return (
    <aside className="terminal widgets" aria-label="Widgets" {...bind.column()}>
      {cards.map((card) => (
        <section
          key={card.id}
          className={classes("frame", "widget", card.size, card.held ? "held" : undefined)}
          {...bind.card(card.id)()}
        >
          <div className="title">
            <span className="name">{card.name}</span>
            <span className="actions">
              <button
                type="button"
                className="knob"
                aria-label={`Remove ${card.name}`}
                hidden={!editing}
                onClick={() => onRemove(card.id)}
              >
                [x]
              </button>
            </span>
          </div>
          <div className="body">{card.body}</div>
        </section>
      ))}
      {editing ? (
        <div className="frame gallery" role="dialog" aria-label="Add widget">
          <div className="title">
            <span className="name">Add widget</span>
          </div>
          <ul className="list">
            {gallery.map((one) => (
              <li key={one.id}>
                <button type="button" onClick={() => onAdd(one.id)}>
                  + {one.name}
                </button>
              </li>
            ))}
          </ul>
          <div className="buttons">
            <button type="button" className="terminal control button primary" onClick={() => onEdit(false)}>
              Done
            </button>
          </div>
        </div>
      ) : (
        <div className="buttons">
          <button type="button" className="terminal control button" onClick={() => onEdit(true)}>
            Edit
          </button>
        </div>
      )}
    </aside>
  );
}
