import { page } from '../../lib/scene.jsx';
import { PAGE } from './scene.js';
import { Widget } from './widget.jsx';
export const { mount, unmount } = page(Widget, { seed: 0, dim: 2, base: 3, code: '' }, PAGE);
