import { configure } from '../ui/config.js';
import { title, since, prefix, tint, menu, cart, company } from '../site.json';

const site = { title, since, prefix, tint, menu, cart, company };

configure(site);

export default site;
