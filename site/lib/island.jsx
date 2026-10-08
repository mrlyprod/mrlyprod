import { createRoot } from 'react-dom/client';

export function island(open) {
  const live = new Map();
  return {
    mount(host) {
      if (live.has(host)) return;
      const opened = open(host);
      if (!opened) return;
      const { at = host, node, close } = opened;
      const root = createRoot(at);
      root.render(node);
      live.set(host, () => {
        root.unmount();
        close?.();
      });
    },
    unmount(host) {
      live.get(host)?.();
      live.delete(host);
    },
  };
}
