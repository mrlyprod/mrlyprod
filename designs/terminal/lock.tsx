import type { Props } from "../../net/shell/design.ts";
import { banner } from "./banner.ts";

/* LOCK */

export function Lock({ word, state, bind }: Props["lock"]) {
  return (
    <div className="terminal lock" data-state={state} hidden={state === "leaving"} {...bind()}>
      <div className="boot">
        <pre className="word" role="img" aria-label={word}>
          {banner(word)}
        </pre>
        <p className="line one">{`${word.toLowerCase()} 1.0 tty0`}</p>
        <p className="line two">
          loading <span className="progress" aria-hidden="true" />
        </p>
        <p className="line three">
          ready
          <span className="cursor" aria-hidden="true" />
        </p>
      </div>
    </div>
  );
}
