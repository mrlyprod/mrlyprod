import { configure } from '../ui/config.js';
import { title, since, prefix, tint, menu, cart, company, tree, shelves } from '../site.json';

const site = { title, since, prefix, tint, menu, cart, company, tree, shelves };

configure(site);

export default site;
