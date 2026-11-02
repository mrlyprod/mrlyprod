import type { Props } from "../../net/shell/design.ts";
import { Tile } from "./tile.tsx";

/* DOCK */

export function Dock({ launch, groups, bind }: Props["dock"]) {
  return (
    <nav className="classic dock" aria-label="Desk" {...bind.root()}>
      <div className="group">
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
