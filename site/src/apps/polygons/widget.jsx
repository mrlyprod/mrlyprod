import { useRef } from "react";
import { useLatest } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { make, SETS } from "./scene.js";
import { inks, swatch } from "./swatch.js";

const NAMES = { square: "4.4.4.4", triangle: "3.3.3.3.3.3", hex: "6.6.6", cube: "rhombille", mix: "3.4.6.4" };

const named = (word) => word[0].toUpperCase() + word.slice(1);

export function Shelves({ value, onChange }) {
  const tick = useTheme();
  const { shape, palette, seed, noise, tint } = value;
  const dye = (ctx) => inks(ctx.canvas, paints(ctx.canvas, tint));
  return (
    <Library>
      <Shelf name="Shapes" grid>
        {SETS.shape.map((one) => (
          <Item
            key={one}
            label={named(one)}
            sub={NAMES[one]}
            on={one === shape}
            onPick={() => onChange({ shape: one })}
            art={
              <Art
                paint={(ctx, w, h) => swatch(ctx, w, h, dye(ctx), { shape: one, palette, seed, noise })}
                deps={[one, palette, seed, noise, tint, tick]}
              />
            }
          />
        ))}
      </Shelf>
      <Shelf name="Palettes" grid>
        {SETS.palette.map((one) => (
          <Item
            key={one || "seeded"}
            label={one ? named(one) : "Seeded"}
            on={one === palette}
            onPick={() => onChange({ palette: one })}
            art={
              <Art
                paint={(ctx, w, h) =>
                  swatch(ctx, w, h, dye(ctx), { shape: "square", palette: one, seed, noise, cells: 5 })
                }
                deps={[one, seed, noise, tint, tick]}
              />
            }
          />
        ))}
      </Shelf>
    </Library>
  );
}

export function Widget({ value, onChange, onReady }) {
  const live = useRef(null);
  const now = useLatest(value);
  const tell = useLatest(onChange);
  const cycle = (key, by) => {
    const list = SETS[key];
    tell.current?.({ [key]: list[(list.indexOf(now.current[key]) + by + list.length) % list.length] });
  };
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle && { ...handle, cycle });
  };
  const tap = (e) => {
    const box = e.currentTarget.getBoundingClientRect();
    live.current?.poke((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height);
  };
  return (
    <>
      <Scene make={make} value={value} onReady={ready} onPointerDown={tap} />
      <Shelves value={value} onChange={onChange} />
    </>
  );
}
