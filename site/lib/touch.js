import { useRef } from 'react';

const at = (e) => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

export function useTouch(handle, change) {
  const down = useRef(false);
  const send = (e, start) => {
    const patch = handle.current?.touch?.(...at(e), start);
    if (patch) change.current?.(patch);
  };
  const up = (e) => {
    if (!down.current) return;
    down.current = false;
    const patch = handle.current?.lift?.(...at(e));
    if (patch) change.current?.(patch);
  };
  return {
    onPointerDown: (e) => {
      if (e.button) return;
      e.currentTarget.setPointerCapture?.(e.pointerId);
      down.current = true;
      send(e, true);
    },
    onPointerMove: (e) => down.current && send(e, false),
    onPointerUp: up,
    onPointerCancel: up,
  };
}
