import type { Ask, Props } from "@mrlyprod/ui/shell/design.ts";
import { BOXY } from "./cell.ts";
import { Word } from "./word.tsx";

function Field({ ask }: { ask: Ask }) {
  return (
    <form className="ask" {...ask.bind.form()}>
      <input className="cell" placeholder="password" {...ask.bind.field()} />
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
      <div className="lock" {...BOXY} data-state={state} role="img" aria-label={word} {...bind()}>
        <Word text={word} className="word big" />
      </div>
    );
  return (
    <div className="lock" {...BOXY} data-state={state} role="dialog" aria-label={word} {...bind()}>
      <Word text={word} className="word big" />
      <Field ask={ask} />
    </div>
  );
}
