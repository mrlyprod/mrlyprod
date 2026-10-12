import { hub } from '../../ui/hub.jsx';
import { filled, Menu } from '../../ui/parts.jsx';

export const { layout, render, mount, unmount } = hub((row, site, { rows }) => <Menu tree={filled(site, rows)} />);
