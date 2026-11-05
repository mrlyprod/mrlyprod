import { fold, markup } from "@mrlyprod/ui/kit/font/fold.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Fold({ word }: { word: string }) {
  const model = fold(word);
  return (
    <svg
      className="glyphs fold"
      viewBox={`0 0 ${model.cols} 5`}
      style={{ "--rows": 5, "--cols": model.cols } as object}
      aria-hidden="true"
      dangerouslySetInnerHTML={{ __html: markup(model) }}
    />
  );
}

export function Lock({ word, state, bind, ask }: Props["lock"]) {
  if (!ask) return null;
  return (
    <div className="lock" data-state={state} role="dialog" aria-label={word} {...bind()}>
      <Fold word={word} />
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
