import { hub } from '../../shell/hub.jsx';
import { filled, Menu } from '../../shell/parts.jsx';

export const { layout, render, mount, unmount } = hub((row, site, { rows }) => <Menu tree={filled(site, rows)} />);
