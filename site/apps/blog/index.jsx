import { hub } from '../../ui/hub.jsx';
import { Grid } from '../../ui/parts.jsx';

const posts = (rows) => rows.filter((one) => one.meta?.prose === 'post').map((one) => ({ name: one.title, href: one.route, figure: one.figure, text: one.lead, dates: [one.date] }));

export const { layout, render, mount, unmount } = hub((row, site, { rows }) => <Grid nodes={posts(rows)} />);
