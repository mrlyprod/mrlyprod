import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useUnits } from "@mrlyprod/ui/kit/hooks.ts";
import { Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { PointerEvent } from "react";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { ledger } from "../shared/sequences/engine.ts";
import type { Picked } from "./cards.ts";
import { build, CARDS, shown } from "./cards.ts";
import type { Feed, Tour } from "./scene.ts";
import { make, walked } from "./scene.ts";

export type Asked = Picked & { seed: number };

type Contract = WidgetProps<Units, Tour>;

type Props = Pick<Contract, "value" | "onChange" | "onReady"> & Partial<Pick<Contract, "onStatus">> & { value: Asked };

const UNITS = wasm("math", "num");

const STILL = {};

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const units = useUnits(UNITS);
  const live = useRef<Feed>({ made: null, version: 0 });
  const stage = useRef<Handle<Tour> | null>(null);
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const book = useMemo(() => units && ledger(units.math), [units]);
  const { card, step } = value;
  const made = useMemo(() => (units && book ? build({ ...units, book }, card, step) : null), [units, book, card, step]);
  const said = shown(card, step);
  useEffect(() => {
    live.current = {
      made,
      card,
      shown: `${CARDS[card].label.toLowerCase()} ${said}`,
      version: live.current.version + 1,
    };
    stage.current?.draw?.();
  }, [made]);
  const link = made?.lines.find((line) => line.link);
  const toldKey = made ? `${card}|${step}|${made.lines.map((line) => line.terms.join()).join("|")}` : "";
  useEffect(() => {
    if (!made) return onStatus?.([]);
    const rows: Readout[] = [[CARDS[card].label.toLowerCase(), said]];
    for (const line of made.lines)
      rows.push([line.label, `${line.terms.join(", ")}${line.capped ? ", to the budget" : ""}`]);
    if (link)
      rows.push([
        "ledger",
        <a key="ledger" href={link.link}>
          open in sequences
        </a>,
      ]);
    onStatus?.(rows);
  }, [toldKey]);
  const ready = (got: Handle<Tour> | null) => {
    stage.current = got;
    onReady?.(got);
  };
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, live }),
    [],
  );
  const tap = (e: PointerEvent<HTMLCanvasElement>) => {
    const got = stage.current;
    if (!got?.tap) return;
    const box = e.currentTarget.getBoundingClientRect();
    const k = e.currentTarget.width / (box.width || 1);
    if (got.tap((e.clientX - box.left) * k, (e.clientY - box.top) * k)?.picture)
      tell.current?.(walked(now.current.card, now.current.step, 1));
  };
  const one = CARDS[card];
  if (!units) return null;
  return (
    <>
      <Scene
        make={scene}
        value={STILL}
        onReady={ready}
        onPointerUp={tap}
        role="img"
        aria-label={`${card + 1}, ${one.title}`}
      />
      <Library name="Cards">
        <Shelf>
          {CARDS.map((each, i) => (
            <Item
              key={each.key}
              label={each.title}
              sub={String(i + 1)}
              on={i === card}
              onPick={() => tell.current?.({ card: i, step: each.first })}
            />
          ))}
        </Shelf>
      </Library>
    </>
  );
}
