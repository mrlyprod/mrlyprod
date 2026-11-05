import type { Ask, Props } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";
import { Sprite } from "./sprite.tsx";

function Prompt({ ask }: { ask: Ask }) {
  return (
    <form className="ask" {...ask.bind.form()}>
      {ask.wrong && (
        <p className="wrong" role="alert">
          WRONG PASSWORD
        </p>
      )}
      <label className="entry">
        <span className="prompt">ENTER PASSWORD</span>
        <input className="field" spellCheck={false} {...ask.bind.field()} />
      </label>
    </form>
  );
}

export function Lock({ word, state, bind, ask }: Props["lock"]) {
  return (
    <div className="pixel lock" data-state={state} hidden={state === "leaving"} {...bind()}>
      <div className="cabinet">
        <p className="score" aria-hidden="true">
          <span>1UP</span>
          <b>00</b>
          <span>HIGH SCORE</span>
          <b>{String(word.length * 1000).padStart(6, "0")}</b>
        </p>
        <h1 className="word" aria-label={word}>
          {[...word].map((c, i) => (
            <span key={`${c}${i}`} aria-hidden="true" style={{ "--n": i } as CSSProperties}>
              {c}
            </span>
          ))}
        </h1>
        <div className="cast" aria-hidden="true">
          <Sprite art="pac" size={36} />
          <span className="dots" />
          <Sprite art="ghost" size={36} />
          <Sprite art="ghost" size={36} />
          <Sprite art="ghost" size={36} />
        </div>
        {ask ? (
          <Prompt ask={ask} />
        ) : (
          <>
            <p className="coin">INSERT COIN</p>
            <p className="press">PRESS ANY KEY</p>
          </>
        )}
        <p className="credit">CREDIT 0</p>
      </div>
    </div>
  );
}
