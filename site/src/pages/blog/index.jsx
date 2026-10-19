import { hub } from '../../shell/hub.jsx';
import { Grid } from '../../shell/parts.jsx';

const posts = (rows) => rows.filter((one) => one.meta?.prose === 'post').map((one) => ({ name: one.title, href: one.route, figure: one.figure, text: one.lead, dates: [one.date] }));

export const { layout, render, unmount } = hub((row, site, { rows }) => <Grid nodes={posts(rows)} />, { bare: true });
