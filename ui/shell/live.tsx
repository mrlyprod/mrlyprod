import { useEffect, useRef, useState } from "react";
import { Alert, Toast, useLive } from "../kit/pieces.tsx";
import { show } from "./parts.tsx";

function Live() {
  const { notes, asks } = useLive();
  const region = useRef<HTMLDivElement>(null);
  const [up, setUp] = useState(false);
  const [over, setOver] = useState(false);
  const [within, setWithin] = useState(false);
  const one = asks[0];
  useEffect(() => {
    const el = region.current!;
    setWithin(el.contains(document.activeElement));
    const fit = () => {
      const open = el.matches(":popover-open");
      if (!notes.length) {
        if (open) el.hidePopover();
      } else if (!open && !document.querySelector("dialog:modal")) el.showPopover();
    };
    fit();
    document.addEventListener("close", fit, true);
    return () => document.removeEventListener("close", fit, true);
  }, [notes]);
  return (
    <>
      <div
        ref={region}
        className="toasts"
        popover="manual"
        role="status"
        onToggle={(e) => {
          setUp(e.newState === "open");
          setOver(false);
        }}
        onPointerEnter={() => setOver(true)}
        onPointerLeave={() => setOver(false)}
        onFocus={() => setWithin(true)}
        onBlur={(e) => setWithin(e.currentTarget.contains(e.relatedTarget))}
      >
        {up && notes.map((note) => <Toast key={note.id} note={note} held={over || within} />)}
      </div>
      {one && <Alert key={one.id} one={one} />}
    </>
  );
}

export function start() {
  const host = document.createElement("div");
  document.body.append(host);
  show(host, <Live />);
}
