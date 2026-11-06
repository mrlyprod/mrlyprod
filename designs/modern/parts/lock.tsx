import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Lock({ word, state, bind, ask }: Props["lock"]) {
  if (!ask)
    return (
      <div className="hello" data-state={state} role="img" aria-label={word} {...bind()}>
        <span className="word">{word}</span>
      </div>
    );
  return (
    <div className="lock" data-state={state} role="dialog" aria-label={word} {...bind()}>
      <span className="word">{word}</span>
      <form className="ask" {...ask.bind.form()}>
        <input placeholder="Enter Password" {...ask.bind.field()} />
        {ask.wrong && (
          <p className="wrong" role="alert">
            Wrong password.
          </p>
        )}
      </form>
    </div>
  );
}
