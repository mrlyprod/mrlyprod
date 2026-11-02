import type { CSSProperties } from "react";
import type { Ask, Props } from "../../net/shell/design.ts";
import { BRICKS, hash, paint } from "./brick.ts";

/* LOCK */

const spell = (word: string) => [...word].map((letter, n) => ({ letter, n, key: word.slice(0, n + 1) }));

function Letters({ word }: { word: string }) {
  return spell(word).map(({ letter, n, key }) => (
    <span
      key={key}
      className="letter brick"
      style={{ ...paint(hash(key)), "--n": n } as CSSProperties}
      aria-hidden="true"
    >
      <span className="lip studs" />
      <span className="print">{letter}</span>
    </span>
  ));
}

function Slot({ ask }: { ask: Ask }) {
  return (
    <form className="ask" {...ask.bind.form()}>
      <input className="text tile" placeholder="Password" {...ask.bind.field()} />
      {ask.wrong && (
        <p className="wrong" role="alert">
          Wrong password.
        </p>
      )}
    </form>
  );
}

export function Lock({ word, state, bind, ask }: Props["lock"]) {
  if (!ask)
    return (
      <div className="lock" {...BRICKS} data-state={state} role="img" aria-label={word} {...bind()}>
        <Letters word={word} />
      </div>
    );
  return (
    <div className="lock asking" {...BRICKS} data-state={state} role="dialog" aria-label={word} {...bind()}>
      <div className="row">
        <Letters word={word} />
      </div>
      <Slot ask={ask} />
    </div>
  );
}
