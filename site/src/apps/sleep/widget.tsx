import { useLatest } from "../../lib/app/hooks.ts";
import type { Paint } from "../../lib/app/library.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Units } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { DESIGNS, ink, NUMBERS, tile } from "../../lib/designs/tiles.ts";
import type { Sleep } from "./scene.ts";
import { make, next } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Sleep>> & Pick<WidgetProps<Units, Sleep>, "value" | "onChange">;

type ShelvesProps = Pick<WidgetProps<Units, Sleep>, "value" | "onChange">;

const named = (word: string) => word[0].toUpperCase() + word.slice(1);

const paint =
  (tint: string, draw: (ctx: CanvasRenderingContext2D, w: number) => void): Paint =>
  (ctx, w) => {
    ctx.fillStyle = paints(ctx.canvas, tint).accent;
    draw(ctx, w);
  };

const stamp = (ctx: CanvasRenderingContext2D, design: string, n: number, x: number, y: number, side: number) => {
  const mark = tile(design, n, 1);
  const edge = Math.max(1, Math.floor(side / mark.size)) * mark.size;
  ink(ctx, mark, Math.round(x + (side - edge) / 2), Math.round(y + (side - edge) / 2), edge);
};

const heap = (ctx: CanvasRenderingContext2D, w: number) => {
  const half = Math.floor(w / 2);
  for (const [i, design] of DESIGNS.slice(0, 4).entries())
    stamp(ctx, design, NUMBERS[0], (i % 2) * half, Math.floor(i / 2) * half, half);
};

export function Shelves({ value, onChange }: ShelvesProps) {
  const tick = useTheme();
  const { design, number } = value;
  const tint = value.tint as string;
  return (
    <Library>
      <Shelf name="Random" grid>
        <Item
          label="Mixed"
          sub="any"
          on={design === "" && number === ""}
          onPick={() => onChange({ design: "", number: "" })}
          art={<Art paint={paint(tint, heap)} deps={[tint, tick]} />}
        />
      </Shelf>
      {DESIGNS.map((one) => (
        <Shelf key={one} name={named(one)} grid>
          {NUMBERS.map((n) => (
            <Item
              key={n}
              label={`${n} x ${n}`}
              on={design === one && number === n}
              onPick={() => onChange({ design: one, number: n })}
              art={<Art paint={paint(tint, (ctx, w) => stamp(ctx, one, n, 0, 0, w))} deps={[one, n, tint, tick]} />}
            />
          ))}
        </Shelf>
      ))}
    </Library>
  );
}

export function Widget({ value, onChange, onReady }: Props) {
  const now = useLatest(value);
  const tell = useLatest(onChange);
  const step = (by: number) => tell.current?.(next(now.current, by));
  return (
    <>
      <Scene make={make} value={value} onReady={(handle) => onReady?.(handle && { ...handle, step })} />
      <Shelves value={value} onChange={onChange} />
    </>
  );
}
