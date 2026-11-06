import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Tile } from "./tile.tsx";

export function Dock({ launch, groups, bind }: Props["dock"]) {
  return (
    <nav className="dock" aria-label="Desk" {...bind.root()}>
      <div className="shelf">
        <Tile {...launch} named bind={bind.launch} />
      </div>
      {groups.map((group, g) => (
        <div key={group.map((tile) => tile.id).join(" ")} className="shelf" {...bind.group(g)()}>
          {group.map((tile) => (
            <Tile key={tile.id} {...tile} named bind={bind.tile(tile.id)} />
          ))}
        </div>
      ))}
    </nav>
  );
}
