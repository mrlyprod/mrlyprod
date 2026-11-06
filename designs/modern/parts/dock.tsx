import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Fragment } from "react";
import { Tile } from "./tile.tsx";

export function Dock({ launch, groups, bind }: Props["dock"]) {
  return (
    <nav className="dock" data-lens="dock" aria-label="Dock" {...bind.root()}>
      <a
        className="launcher launch"
        href={launch.href}
        aria-label={launch.title}
        data-name={launch.title}
        draggable="false"
        {...bind.launch()}
        {...bind.lens.item(launch.id)()}
      >
        {launch.art}
      </a>
      {groups.map((tiles, g) => (
        <Fragment key={g}>
          <span className="split" aria-hidden="true" {...bind.lens.gap(g)()} />
          <ul className="apps" {...bind.group(g)()}>
            {tiles.map((tile) => (
              <li key={tile.id} {...bind.tile(tile.id)()}>
                <Tile {...tile} named={false} bind={bind.lens.item(tile.id)} />
              </li>
            ))}
          </ul>
        </Fragment>
      ))}
    </nav>
  );
}
