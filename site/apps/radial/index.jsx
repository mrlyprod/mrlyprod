import { defaults } from '../../lib/knobs.js';
import { page } from '../../lib/scene.jsx';
import { SOURCE } from '../spin/scene.js';
import { PAGE } from './scene.js';
import { Widget } from './widget.jsx';
export const { mount, unmount } = page(Widget, { seed: 0, ...defaults(SOURCE) }, PAGE);
