import { useCallback, useMemo, useRef, useState } from "react";
import type { Readout } from "../../lib/app/hooks.ts";
import { useReroll, useStatus } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Scalar, Units } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Door } from "../../lib/life/engine.ts";
import { MaskView, Pane, useLife, usePaint } from "../../lib/life/parts.tsx";
import { sprite } from "../../lib/life/scene.ts";
import { rng } from "../../lib/rand.ts";
import type { Card, Stat, Wolfram } from "./scene.ts";
import { card, FAMOUS, make, SPEC } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Wolfram>> & Pick<WidgetProps<Units, Wolfram>, "value" | "onChange">;

const RULES = 256;
const STEPS = 20;
const LEGEND = "corner i = 4 x0 + 2 x1 + x2: the top row is l = 0, the columns run 00, 01, 10, 11 in (c, r)";
const CENSUS = "Exactly 30 of the 256 rules are onto and exactly 6 are reversible: 15, 51, 85, 170, 204 and 240.";

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

const cone = (life: Door, rule: number) => life.single_seed(rule, STEPS);

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const units = useLife();
  const handle = useRef<Handle<Wolfram> | null>(null);
  const now = useRef(value);
  now.current = value;
  const change = useRef(onChange);
  change.current = onChange;
  const [stat, setStat] = useState<Stat | null>(null);
  const tick = useTheme();
  const paint = usePaint(handle);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units!, onStat: setStat }),
    [units],
  );
  const turn = (by: number) => change.current({ rule: ((now.current.rule as number) + by + RULES) % RULES });
  const ready = (scene: Handle<Wolfram> | null) => {
    const rich = scene && { ...scene, turn };
    handle.current = rich;
    onReady?.(rich);
  };
  const rule = value.rule as number;
  const read = useMemo(() => (units ? card(units.life, rule) : null), [units, rule]);
  const stamp = useMemo(() => read && { shape: [2, 4], data: read.corners }, [read]);
  const pick = (one: number) => onChange({ rule: one });
  const thumb = (one: number) => (
    <Art
      paint={(ctx, w) => {
        const { shape, data } = cone(units!.life, one);
        sprite(ctx, data, shape[1], shape[0], w, paints(ctx.canvas).accent);
      }}
      deps={[units, one, tick]}
    />
  );
  useStatus(onStatus, status(rule, read, stat));
  useReroll(value.seed as number, (seed) => change.current({ rule: Math.floor(rng(seed)() * RULES) }));
  const folds = [
    read && {
      name: "Card",
      live: `${read.popcount} of 8`,
      items: [
        <MaskView key="stamp" mask={stamp} dot={32} label="The eight corner bits of the rule" />,
        <small key="legend">{LEGEND}</small>,
        <small key="census">{CENSUS}</small>,
        <small key="mixed">
          {read.mixed
            ? "Onto is not constant on this orbit, so the cube symmetry is not a symmetry of the dynamics."
            : "Every rule in this orbit answers onto the same way."}
        </small>,
      ],
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
