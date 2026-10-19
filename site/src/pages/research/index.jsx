import { own } from '../../lib/app/reader.jsx';
import { figures } from '../../shell/figures.js';
import { Widget } from './widget.jsx';

export const { layout, render, mount, unmount } = own((host, { rows, site }) => <Widget rows={rows} site={site} draw={figures} />);
