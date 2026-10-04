import { createRoot } from 'react-dom/client';
import { follow } from './mrly.js';

export function island(open) {
  const live = new Map();
  return {
    mount(host) {
      if (live.has(host)) return;
      const opened = open(host);
      if (!opened) return;
      const { at = host, node, close } = opened;
      const root = createRoot(at);
      const quit = follow();
      root.render(node);
      live.set(host, () => {
        root.unmount();
        quit();
        close?.();
      });
    },
    unmount(host) {
      live.get(host)?.();
      live.delete(host);
    },
  };
}
