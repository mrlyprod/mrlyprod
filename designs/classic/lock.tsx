import type { Ask, Props } from "@mrlyprod/ui/shell/design.ts";

function Dialog({ ask }: { ask: Ask }) {
  return (
    <form className="ask" {...ask.bind.form()}>
      <label className="row">
        <span className="label">Password:</span>
        <input className="sunken field" {...ask.bind.field()} />
      </label>
      {ask.wrong && (
        <p className="wrong" role="alert">
          Wrong password.
        </p>
      )}
      <button type="submit" className="raised button" disabled={ask.busy}>
        OK
      </button>
    </form>
  );
}

export function Lock({ word, state, bind, ask }: Props["lock"]) {
  return (
    <div className="classic lock" data-state={state} hidden={state === "leaving"} {...bind()}>
      {ask ? (
        <div className="raised box" role="dialog" aria-label={word}>
          <b className="word">{word}</b>
          <Dialog ask={ask} />
        </div>
      ) : (
        <div className="raised box">
          <b className="word">{word}</b>
          <span className="sunken progress" aria-hidden="true">
            <span className="fill" />
          </span>
        </div>
      )}
    </div>
  );
}
