import type { CSSProperties } from "react";
import type { Props } from "../../net/shell/design.ts";
import { BLOCKS, hash, paint } from "./brick.ts";

/* LOCK */

const spell = (word: string) => [...word].map((letter, n) => ({ letter, n, key: word.slice(0, n + 1) }));

export function Lock({ word, state, bind }: Props["lock"]) {
  return (
    <div className="lock" {...BLOCKS} data-state={state} role="img" aria-label={word} {...bind()}>
      {spell(word).map(({ letter, n, key }) => (
        <span
          key={key}
          className="letter brick"
          style={{ ...paint(hash(key)), "--n": n } as CSSProperties}
          aria-hidden="true"
        >
          <span className="lip studs" />
          <span className="print">{letter}</span>
        </span>
      ))}
    </div>
  );
}
