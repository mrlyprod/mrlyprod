import { Fold, Segment, Pick as Select, Slider } from "../app/controls.jsx";
import { useReroll as rerolled } from "../app/hooks.js";
import { tidy } from "../app/spec.js";
import { canon } from "../designs/engine.js";
import { rng } from "../rand.js";
import { CUT, cap, cube, LIMIT, SOURCES, title } from "./engine.js";
import { SOURCE } from "./scene.js";

export const read = (value) => tidy(SOURCE, value);

const option = (key) => SOURCE.find((row) => row.key === key).options;

export const useReroll = (seed, roll) => rerolled(seed, (next) => roll(rng(next)));

export function gist(value, math) {
  const src = read(value);
  if (src.source === "moire") return `Moire · ${src.preset} · ${src.limit}`;
  const dim = cube(src) ? 3 : 2;
  const code = math ? canon(math, dim, src.base, src.code) : src.code;
  const name = title({ name: math?.name.of(code, dim, src.base) ?? "", code, kind: "design" });
  return dim === 3 ? `Slice · ${name} · ${src.tile} · ${src.slevel}` : `Flat · ${name} · ${src.number} · ${src.level}`;
}

export function Rows({ value, onChange }) {
  const src = read(value);
  const write = (patch) => onChange(read({ ...src, ...patch }));
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

export function Source({ value, onChange, math }) {
  return (
    <Fold name="Source" live={gist(value, math)}>
      <Rows value={value} onChange={onChange} />
    </Fold>
  );
}
