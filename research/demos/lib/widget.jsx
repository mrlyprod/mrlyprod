import { island } from './island.jsx';
import { follow } from './mrly.js';

export function embed(views) {
  return island((host) => {
    const View = views[host.dataset.view];
    const at = host.querySelector('.mount');
    return View && at ? { at, node: <View />, close: follow() } : null;
  });
}
