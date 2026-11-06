import type { Action } from "@mrlyprod/ui/kit/controls.tsx";
import { Fold } from "@mrlyprod/ui/kit/controls.tsx";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { canon } from "@mrlyprod/ui/kit/designs/engine.ts";
import { download, text } from "@mrlyprod/ui/kit/export.ts";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { Knob } from "@mrlyprod/ui/kit/knobs.tsx";
import type { Patch, Row, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import type { ReactNode, RefObject } from "react";
import { useEffect } from "react";
import { DIM, named, read } from "./engine.ts";
import { SIZES } from "./scene.ts";

export type Factual<F> = { facts?: () => F | null | undefined };

export type Pictured = { svg?: () => string | null | undefined; shot?: (size: number) => Promise<Blob> };

export type Pane = { name: string; live?: ReactNode; body: ReactNode };

export type FoldsProps = { groups: readonly Pane[] };

export type Titled<U> = Row<U> & { label: string };

const WAIT = 90;

export function useStatus<F>(
  live: RefObject<Factual<F> | null>,
  key: string,
  onStatus: ((rows: Readout[]) => void) | undefined,
  rows: (facts: F) => Readout[],
) {
  useEffect(() => {
    if (!onStatus) return undefined;
    const timer = setTimeout(() => {
      const facts = live.current?.facts?.();
      if (facts) onStatus(rows(facts));
    }, WAIT);
    return () => clearTimeout(timer);
  }, [key]);
}

export function useShots(
  onExport: ((rows: Action[]) => void) | undefined,
  live: RefObject<Pictured | null>,
  have: boolean,
  name: () => string,
) {
  const svg = () => {
    const made = live.current?.svg?.();
    if (made) text(made, name(), "svg");
  };
  const shot = (size: number) => live.current?.shot?.(size).then((blob) => download(blob, `${name()}-${size}.png`));
  useEffect(
    () => onExport?.(have ? [...SIZES.map((size): Action => [`PNG ${size}`, () => shot(size)]), ["SVG", svg]] : []),
    [have],
  );
}

export function Folds({ groups }: FoldsProps) {
  return groups.map(({ name, live, body }) => (
    <Fold key={name} name={name} live={live}>
      {body}
    </Fold>
  ));
}

export function knobs<U>(
  keys: readonly string[],
  value: Value,
  onChange: (patch: Patch) => void,
  spec: Spec<U>,
  units?: U,
) {
  return keys.map((key) => {
    const one = spec.find((it) => it.key === key)!;
    return <Knob key={key} row={one} value={value[key]} values={value} units={units} onChange={onChange} />;
  });
}

export function design(
  value: Value,
  onChange: (patch: Patch) => void,
  unit: Mrly | null | undefined,
  spec: readonly Titled<Mrly>[],
): Pane {
  const { base, code } = read(value);
  const shown = unit ? canon(unit, DIM, base, code) : code;
  const name = unit ? named(unit, shown, base) : "";
  const word = spec.find((one) => one.key === "number")!.label.toLowerCase();
  return {
    name: "Design",
    live: [name || shown, `${word} ${value.number}`, `level ${value.level}`].join(" · "),
    body: knobs(["number", "level"], value, onChange, spec, unit ?? undefined),
  };
}
