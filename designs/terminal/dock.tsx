import type { Props } from "../../net/shell/design.ts";
import { Tile } from "./tile.tsx";

/* DOCK */

export function Dock({ launch, groups, bind }: Props["dock"]) {
  return (
    <nav className="terminal dock" aria-label="Windows" {...bind.root()}>
      <span className="session" aria-hidden="true">
        [mrly]
      </span>
      <div className="group launch">
        <Tile {...launch} named bind={bind.launch} />
      </div>
      {groups.map((group, g) => (
        <div key={group.map((tile) => tile.id).join(" ")} className="group" {...bind.group(g)()}>
          {group.map((tile) => (
            <Tile key={tile.id} {...tile} named bind={bind.tile(tile.id)} />
          ))}
        </div>
      ))}
    </nav>
  );
}
