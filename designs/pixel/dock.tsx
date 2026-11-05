import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Tile } from "./tile.tsx";

export function Dock({ launch, groups, bind }: Props["dock"]) {
  return (
    <nav className="pixel dock" aria-label="Dock" {...bind.root()}>
      <div className="group launch">
        <Tile {...launch} named={false} bind={bind.launch} />
      </div>
      {groups.map((group, g) => (
        <div key={group.map((tile) => tile.id).join(" ")} className="group" {...bind.group(g)()}>
          {group.map((tile) => (
            <Tile key={tile.id} {...tile} named={false} bind={bind.tile(tile.id)} />
          ))}
        </div>
      ))}
    </nav>
  );
}
