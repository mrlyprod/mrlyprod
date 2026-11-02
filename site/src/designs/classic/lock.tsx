import type { Props } from "../../shell/design.ts";

/* LOCK */

export function Lock({ word, state, bind }: Props["lock"]) {
  return (
    <div className="classic lock" data-state={state} hidden={state === "leaving"} {...bind()}>
      <div className="raised box">
        <b className="word">{word}</b>
        <span className="sunken progress" aria-hidden="true">
          <span className="fill" />
        </span>
      </div>
    </div>
  );
}
