import { useCallback, useMemo, useRef } from "react";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { globe } from "../../lib/space/globe.js";
import { NAMES, world } from "../../lib/space/worlds.js";
import { make, named, next } from "./scene.js";

const tag = (params) => {
  if (params.ring) return "rings";
  const n = params.moons.length;
  return n ? `${n} moon${n > 1 ? "s" : ""}` : "";
};

export function Widget({ value, onChange, onReady }) {
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const step = (by) => tell.current?.({ world: next(now.current.world, by) });
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, live: now }), []);
  const { world: current, moons, rings, seed } = value;
  const close = useMemo(() => NAMES.filter((name) => name !== "exo").map((name) => [name, world(name, 0)]), []);
  const far = useMemo(() => world("exo", seed), [seed]);
  const tile = (name, params, deps) => (
    <Item
      key={name}
      label={named(name)}
      sub={name === "exo" ? far.kind : tag(params)}
      on={name === current}
      onPick={() => onChange({ world: name })}
      art={<Art paint={(ctx, w, h) => globe(ctx, w, h, params)} deps={deps} />}
    />
  );
  return (
    <>
      <Scene
        make={made}
        value={{ world: current, moons, rings, seed }}
        onReady={(handle) => onReady?.(handle && { ...handle, step })}
      />
      <Library name="Worlds">
        <Shelf name="Solar system" grid>
          {close.map(([name, params]) => tile(name, params, [name]))}
        </Shelf>
        <Shelf name="Beyond" grid>
          {tile("exo", far, [seed])}
        </Shelf>
      </Library>
    </>
  );
}
