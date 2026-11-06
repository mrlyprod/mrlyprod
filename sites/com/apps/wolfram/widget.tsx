import { paints } from "@mrlyprod/ui/kit/color.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useStatus, useUnits } from "@mrlyprod/ui/kit/hooks.ts";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Scalar, Seeded, Units } from "@mrlyprod/ui/kit/spec.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import { useCallback, useMemo, useRef, useState } from "react";
import { MaskView, Pane, usePaint } from "../shared/life/parts.tsx";
import { sprite } from "../shared/life/scene.ts";
import type { Card, Door, Stat, Wolfram } from "./scene.ts";
import { card, FAMOUS, make, SPEC } from "./scene.ts";

type Props = Partial<WidgetProps<Units, Wolfram>> & Pick<WidgetProps<Units, Wolfram>, "onChange"> & { value: Seeded };

const UNITS = wasm(...CRATES, "wolfram");

const STEPS = 20;

const yes = (on: boolean, word: string) => (on ? word : `not ${word}`);

export const status = (rule: Scalar, read: Card | null, stat: Stat | null): Readout[] =>
  read && stat
    ? [
        ["rule", rule],
        ["design", read.name],
        ["popcount", `${read.popcount}, lambda ${read.lambda.toFixed(3)}`],
        ["degree", read.degree],
        ["genus", read.genus],
        ["traits", [yes(read.affine, "affine"), yes(read.onto, "onto"), yes(read.reversible, "reversible")].join(", ")],
        ["B/S", read.bs || "none"],
        [
          "classes",
          `Wolfram ${read.klass[0]} of ${read.klass.length}, cube ${read.mates[0].rule} of ${read.mates.length}, NPN ${read.npn}`,
        ],
        ...(read.gasket ? [["draws", read.gasket] satisfies Readout] : []),
        ["rows", stat.rows],
        ["across", stat.cols],
        ["live", stat.live],
        ["seed", stat.how],
      ]
    : [];

const cone = (wolfram: Door, rule: number) => wolfram.single_seed(rule, STEPS);

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const units = useUnits(UNITS);
  const handle = useRef<Handle<Wolfram> | null>(null);
  const [stat, setStat] = useState<Stat | null>(null);
  const tick = useTheme();
  const paint = usePaint(handle);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, wolfram: units!.wolfram, mrly: units!, onStat: setStat }),
    [units],
  );
  const ready = (scene: Handle<Wolfram> | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const rule = value.rule as number;
  const read = useMemo(() => (units ? card(units.wolfram, rule) : null), [units, rule]);
  const stamp = useMemo(() => read && { shape: [2, 4], data: read.corners }, [read]);
  const pick = (one: number) => onChange({ rule: one });
  const thumb = (one: number) => (
    <Art
      paint={(ctx, w) => {
        const { shape, data } = cone(units!.wolfram, one);
        sprite(ctx, data, shape[1], shape[0], w, paints(ctx.canvas).accent);
      }}
      deps={[units, one, tick]}
    />
  );
  useStatus(onStatus, status(rule, read, stat));
  const folds = [
    read && {
      name: "Card",
      live: `${read.popcount} of 8`,
      items: [<MaskView key="stamp" mask={stamp} dot={32} label="The eight corner bits of the rule" />],
    },
  ];
  return (
    <>
      {units && <Scene make={made} value={value} onReady={ready} {...paint} />}
      {units && (
        <Library>
          <Shelf name="Famous" grid>
            {FAMOUS.map(([one, word]) => (
              <Item
                key={one}
                label={String(one)}
                sub={word}
                on={one === rule}
                onPick={() => pick(one)}
                art={thumb(one)}
              />
            ))}
          </Shelf>
          {read && (
            <>
              <Shelf name={`Cube orbit of ${read.mates.length}`} grid>
                {read.mates.map((one) => (
                  <Item
                    key={one.rule}
                    label={String(one.rule)}
                    sub={one.onto ? "onto" : undefined}
                    on={one.rule === rule}
                    onPick={() => pick(one.rule)}
                    art={thumb(one.rule)}
                  />
                ))}
              </Shelf>
              <Shelf name={`Wolfram class of ${read.klass.length}`} grid>
                {read.klass.map((one) => (
                  <Item key={one} label={String(one)} on={one === rule} onPick={() => pick(one)} art={thumb(one)} />
                ))}
              </Shelf>
            </>
          )}
        </Library>
      )}
      <Pane spec={SPEC} value={value} onChange={onChange} folds={folds} />
    </>
  );
}
