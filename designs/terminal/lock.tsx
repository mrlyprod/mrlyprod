import type { Ask, Props } from "@mrlyprod/ui/shell/design.ts";
import { banner } from "./banner.ts";

function Prompt({ ask }: { ask: Ask }) {
  return (
    <form className="ask" {...ask.bind.form()}>
      {ask.wrong && (
        <p className="line wrong" role="alert">
          wrong password
        </p>
      )}
      <label className="line prompt">
        <span aria-hidden="true">password: </span>
        <input className="field" {...ask.bind.field()} />
      </label>
    </form>
  );
}

export function Lock({ word, state, bind, ask }: Props["lock"]) {
  return (
    <div className="terminal lock" data-state={state} hidden={state === "leaving"} {...bind()}>
      <div className="boot">
        <pre className="word" role="img" aria-label={word}>
          {banner(word)}
        </pre>
        <p className="line one">{`${word.toLowerCase()} 1.0 tty0`}</p>
        {ask ? (
          <Prompt ask={ask} />
        ) : (
          <>
            <p className="line two">
              loading <span className="progress" aria-hidden="true" />
            </p>
            <p className="line three">
              ready
              <span className="cursor" aria-hidden="true" />
            </p>
          </>
        )}
      </div>
    </div>
  );
}
