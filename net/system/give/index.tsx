import { useState } from "react";
import { island } from "../../sdk/island.tsx";
import type { Json, Obj } from "../../sdk/system.ts";
import { named, SLICE } from "../../sdk/system.ts";
import { Form, Tree } from "../../sdk/view.tsx";
import { useCall, useMoves, useWho, useWorld } from "../../sdk/world.ts";

/* GIVE */

export const BANK = "wallet";

export const VERB = `${BANK}.give`;

type Ledger = { causes: Record<string, string>; raised: Record<string, number>; gifts: Json[] };

export function Give() {
  const signed = useWho();
  const ledger = useWorld(BANK, ["causes", "raised", "gifts"]) as Ledger | null;
  const balance = useWorld(`${BANK}/${SLICE}${signed?.who.id ?? ""}/balance`);
  const move = useMoves(BANK).find((one) => one.verb === VERB);
  const call = useCall();
  const [note, setNote] = useState<string | null>(null);
  if (!signed) return null;
  if (!ledger) return <p className="sub">This world holds no wallet, so nobody gives.</p>;
  const send = (args: Obj) => {
    const out = call(VERB, args);
    setNote(out.note);
    return out;
  };
  const causes = Object.fromEntries(
    Object.entries(ledger.causes).map(([id, name]) => [id, { name, raised: ledger.raised[id] ?? 0 }]),
  );
  return (
    <section className="world">
      <h1>Give</h1>
      <p className="sub">Give to a person or a cause, never to yourself; every gift is public.</p>
      <p className="reads">
        {signed.actor ? `Signed in as ${named(signed.who.id)}, holding ${balance}` : "Watching as Guest"}
      </p>
      <div className="arena">
        <div className="panel">
          <Tree value={{ causes, gifts: [...ledger.gifts].reverse() }} />
        </div>
        <div className="panel">
          {move && signed.actor ? <Form move={move} send={send} /> : null}
          <p className="note" role="status">
            {note}
          </p>
        </div>
      </div>
    </section>
  );
}

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return { node: <Give />, close: () => still && host.after(still) };
});
