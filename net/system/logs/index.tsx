import { island } from "../../sdk/island.tsx";
import { named, SLICE } from "../../sdk/system.ts";
import { Tree } from "../../sdk/view.tsx";
import { useWho, useWorld } from "../../sdk/world.ts";

/* LOGS */

const SIM = "sim";

export function Logs() {
  const signed = useWho();
  const mine = useWorld(signed ? `${SIM}/${SLICE}${signed.who.id}` : SIM);
  if (!signed) return null;
  return (
    <section className="world">
      <h1>Logs</h1>
      <p className="sub">Your own acts, newest first, as the world ticked them.</p>
      <p className="reads">{signed.actor ? `Signed in as ${named(signed.who.id)}` : "Watching as Guest"}</p>
      <div className="arena">
        <div className="panel">
          {mine === null ? (
            <p className="sub">This world holds no {SIM}, so nothing is logged.</p>
          ) : (
            <Tree value={mine} />
          )}
        </div>
      </div>
    </section>
  );
}

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return { node: <Logs />, close: () => still && host.after(still) };
});
