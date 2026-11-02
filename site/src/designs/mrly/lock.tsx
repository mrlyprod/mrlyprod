import type { Props } from "../../shell/design.ts";
import { MRLY } from "./cell.ts";
import { Word } from "./word.tsx";

/* LOCK */

export function Lock({ word, state, bind }: Props["lock"]) {
  return (
    <div className="lock" {...MRLY} data-state={state} role="img" aria-label={word} {...bind()}>
      <Word text={word} className="word big" />
    </div>
  );
}
