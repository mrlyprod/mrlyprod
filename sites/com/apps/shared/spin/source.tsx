import { Fold, Segment, Pick as Select, Slider } from "@mrlyprod/ui/kit/controls.tsx";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { canon } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Option, Patch, Raw } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Src } from "./engine.ts";
import { CUT, cap, cube, LIMIT, SOURCES, title } from "./engine.ts";
import { SOURCE } from "./scene.ts";

export type RowsProps = { value: Raw; onChange: (value: Src) => void };

export type SourceProps = RowsProps & { mrly?: Mrly | null };

export const read = (value: Raw) => tidy(SOURCE, value) as Src;

const option = (key: "number" | "tile") => SOURCE.find((row) => row.key === key)!.options as readonly Option[];

export function gist(value: Raw, mrly?: Mrly | null): string {
  const src = read(value);
  if (src.source === "moire") return `Moire · ${src.preset} · ${src.limit}`;
  const dim = cube(src) ? 3 : 2;
  const code = mrly ? canon(mrly, dim, src.base, src.code) : src.code;
  const name = title({ name: mrly?.math.name.of(code, dim, src.base) ?? "", code, kind: "design" });
  return dim === 3 ? `Slice · ${name} · ${src.tile} · ${src.slevel}` : `Flat · ${name} · ${src.number} · ${src.level}`;
}

export function Rows({ value, onChange }: RowsProps) {
  const src = read(value);
  const write = (patch: Patch) => onChange(read({ ...src, ...patch }));
  return (
    <>
      <Segment label="Source" value={src.source} options={SOURCES} onChange={(v) => write({ source: v })} />
      {src.source === "design" && !cube(src) && (
        <>
          <Select label="Side" value={src.number} options={option("number")} onChange={(v) => write({ number: v })} />
          <Slider
            label="Level"
            value={src.level}
            min={1}
            max={cap(src.number)}
            step={1}
            onChange={(v) => write({ level: v })}
          />
        </>
      )}
      {src.source === "design" && cube(src) && (
        <>
          <Select label="Tile" value={src.tile} options={option("tile")} onChange={(v) => write({ tile: v })} />
          <Slider
            label="Level"
            value={src.slevel}
            min={1}
            max={cap(src.tile, CUT)}
            step={1}
            onChange={(v) => write({ slevel: v })}
          />
        </>
      )}
      {src.source === "moire" && (
        <Slider
          label="Scales up to"
          value={src.limit}
          min={1}
          max={LIMIT}
          step={2}
          onChange={(v) => write({ limit: v })}
        />
      )}
    </>
  );
}

export function Source({ value, onChange, mrly }: SourceProps) {
  return (
    <Fold name="Source" live={gist(value, mrly)}>
      <Rows value={value} onChange={onChange} />
    </Fold>
  );
}
