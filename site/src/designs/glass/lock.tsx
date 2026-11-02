import { fold, markup } from "../../lib/font/fold.ts";
import type { Props } from "../../shell/design.ts";

/* FOLD */

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

/* LOCK */

export function Lock(_: Props["lock"]) {
  return null;
}
