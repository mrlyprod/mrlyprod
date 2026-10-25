import type { PointerEvent } from "react";
import { useRef } from "react";
import { useLatest } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Units } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Mosaic, Poly, Polygons, Sets } from "./scene.ts";
import { make, SETS } from "./scene.ts";
import { inks, swatch } from "./swatch.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Polygons>> & Pick<WidgetProps<Units, Polygons>, "value" | "onChange">;

type ShelvesProps = Pick<WidgetProps<Units, Polygons>, "value" | "onChange">;

const NAMES: Record<string, string> = {
  square: "4.4.4.4",
  triangle: "3.3.3.3.3.3",
  hex: "6.6.6",
  cube: "rhombille",
  mix: "3.4.6.4",
};

const named = (word: string) => word[0].toUpperCase() + word.slice(1);

export function Shelves({ value, onChange }: ShelvesProps) {
  const tick = useTheme();
  const { shape, palette, seed, noise, tint } = value as Mosaic;
  const dye = (ctx: CanvasRenderingContext2D) => inks(ctx.canvas, paints(ctx.canvas, tint));
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

export function Widget({ value, onChange, onReady }: Props) {
  const live = useRef<Handle<Poly> | null>(null);
  const now = useLatest(value);
  const tell = useLatest(onChange);
  const cycle = (key: keyof Sets, by: number) => {
    const list = SETS[key];
    tell.current?.({ [key]: list[(list.indexOf(now.current[key] as string) + by + list.length) % list.length] });
  };
  const ready = (handle: Handle<Poly> | null) => {
    live.current = handle;
    onReady?.(handle && { ...handle, cycle });
  };
  const tap = (e: PointerEvent<HTMLCanvasElement>) => {
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
