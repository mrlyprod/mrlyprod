import { defaults } from '../../lib/app/spec.js';
import { page } from '../../lib/app/page.jsx';
import { SOURCE } from '../../lib/spin/scene.js';
import { PAGE } from './scene.js';
import { Widget } from './widget.jsx';
export const { mount, unmount } = page(Widget, { seed: 0, ...defaults(SOURCE) }, PAGE);
