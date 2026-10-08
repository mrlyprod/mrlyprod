import { page } from '../../lib/scene.jsx';
import { Widget } from './widget.jsx';
export const { mount, unmount } = page(Widget, { seed: 0 });
