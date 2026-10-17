import { page } from '../../lib/app/page.jsx';
import { PAGE } from './scene.js';
import { Widget } from './widget.jsx';
export const { mount, unmount } = page(Widget, { seed: 0 }, PAGE);
