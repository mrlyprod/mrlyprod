import { page } from '../../lib/scene.jsx';
import { DESIGN, PAGE } from './scene.js';
import { Widget } from './widget.jsx';
export const { mount, unmount } = page(Widget, { seed: 0, ...DESIGN }, PAGE);
