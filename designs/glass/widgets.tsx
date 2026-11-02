import type { ReactNode } from "react";
import type { Props } from "../../net/shell/design.ts";
import { Glyph } from "./glyph.tsx";

/* TYPES */

type Card = { id: string; name: string; size: "wide" | "small"; body: ReactNode };

type CardProps = {
  one: Card;
  held?: boolean;
  lens?: boolean;
  bind?: Props["widgets"]["bind"]["card"];
  onRemove?: () => void;
};

const none = () => () => ({});

/* CARD */

function Widget({ one, held, lens = true, bind = none, onRemove }: CardProps) {
  return (
    <article
      className={lens ? "widget" : "widget frost"}
      {...bind(one.id)()}
      data-size={one.size}
      data-lens={lens ? "widget" : undefined}
      data-held={held ? "" : undefined}
      inert={!lens}
      aria-label={one.name}
    >
      {onRemove && (
        <button
          type="button"
          className="remove"
          aria-label={`Remove ${one.name}`}
          title={`Remove ${one.name}`}
          onClick={onRemove}
        >
          <Glyph art="minus" />
        </button>
      )}
      {one.body}
    </article>
  );
}

/* WIDGETS */

export function Widgets({ cards, editing, gallery, menu, onEdit, onAdd, onRemove, bind }: Props["widgets"]) {
  return (
    <>
      <aside className="widgets" aria-label="Widgets" data-editing={editing ? "" : undefined} {...bind.root()}>
        <div className="column" {...bind.column()}>
          {cards.map((one) => (
            <Widget
              key={one.id}
              one={one}
              held={one.held}
              bind={bind.card}
              onRemove={editing ? () => onRemove(one.id) : undefined}
            />
          ))}
        </div>
        {!editing && (
          <button type="button" className="edit" {...bind.edit()} onClick={() => onEdit(true)}>
            Edit Widgets
          </button>
        )}
      </aside>
      {editing && (
        <section className="catalog" {...bind.gallery()} data-lens="window" aria-label="Widget gallery">
          <header>
            <h2>Widgets</h2>
            <button type="button" className="primary" {...bind.done()} onClick={() => onEdit(false)}>
              Done
            </button>
          </header>
          <div className="row" {...bind.shelf()}>
            {gallery.map((one) => (
              <div key={one.id} className="pick" {...bind.pick(one.id)()} data-size={one.size}>
                <Widget one={one} lens={false} />
                <span>{one.name}</span>
                <button type="button" disabled={one.added} onClick={() => onAdd(one.id)}>
                  {one.added ? "Added" : "Add"}
                </button>
              </div>
            ))}
          </div>
        </section>
      )}
      {menu && (
        <div className="pop menu" {...menu.list()} data-lens="menu">
          <button type="button" {...menu.edit()}>
            Edit Widgets
          </button>
          <a className="button" {...menu.wallpaper()}>
            Change Wallpaper
          </a>
        </div>
      )}
    </>
  );
}
