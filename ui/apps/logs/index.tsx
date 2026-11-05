import { named } from "@mrlyprod/os/contract/index.ts";
import { island } from "../../kit/island.tsx";
import { Tree } from "../../kit/view.tsx";
import { useWho, useWorld } from "../../kit/world.ts";
import { current } from "../../shell/world.ts";

const mineOf = (id: string) =>
  [...(current()?.lines() ?? [])]
    .filter((line) => line.who.id === id && !line.via)
    .reverse()
    .map(({ tick, at, verb, args }) => ({ tick, at, verb, args }));

export function Logs() {
  const signed = useWho();
  useWorld();
  if (!signed) return null;
  const mine = mineOf(signed.who.id);
  return (
    <section className="world">
      <h1>Logs</h1>
      <p className="sub">Your own acts, newest first, from this world's journal.</p>
      <p className="reads">{signed.actor ? `Signed in as ${named(signed.who.id)}` : "Watching as Guest"}</p>
      <div className="arena">
        <div className="panel">{mine.length ? <Tree value={mine} /> : <p className="sub">Nothing logged yet.</p>}</div>
      </div>
    </section>
  );
}

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return { node: <Logs />, close: () => still && host.after(still) };
});
