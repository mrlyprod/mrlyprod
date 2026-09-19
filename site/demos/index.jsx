import { mount, useShelves } from '../lib/app.jsx';
import { Grid, Shell } from '../kit/ui/chrome.jsx';
import { sidebar } from '../lib/tree.js';
import site from '../lib/site.js';

const GROUPS = site.shelves.reduce((groups, shelf) => {
  const last = groups[groups.length - 1];
  if (last && last.name === shelf.group) last.shelves.push(shelf);
  else groups.push({ name: shelf.group, shelves: [shelf] });
  return groups;
}, []);

const CONTENTS = GROUPS.flatMap((group) => [
  { id: group.name.toLowerCase(), text: group.name, level: 2 },
  ...group.shelves.map((shelf) => ({ id: shelf.key, text: shelf.title, level: 3 })),
]);

const slug = (href) => href.split('/').filter(Boolean)[1];

const tile = (node) => ({
  ...node,
  figure: { dark: `/figures/demo-${slug(node.href)}-dark.png`, light: `/figures/demo-${slug(node.href)}-light.png` },
});

function Shelf({ shelf, nodes }) {
  if (!nodes.length) return null;
  return (
    <>
      <div className="shelf" id={shelf.key}>
        <h2>{shelf.title}</h2>
        <p>{shelf.blurb}</p>
      </div>
      <Grid nodes={nodes.map(tile)} />
    </>
  );
}

function App() {
  const shelves = useShelves();
  const held = (key) => shelves.find((one) => one.key === key)?.nodes ?? [];
  return (
    <Shell route="/demos/" title="The eyes of MrlyMath" lead="Every number and pixel on these pages comes out of the Rust crates through wasm. The browser only draws." tree={sidebar({ demos: shelves })} contents={CONTENTS} wide>
      {GROUPS.map((group) => (
        <section key={group.name} id={group.name.toLowerCase()}>
          <h2 className="group">{group.name}</h2>
          {group.shelves.map((shelf) => <Shelf key={shelf.key} shelf={shelf} nodes={held(shelf.key)} />)}
        </section>
      ))}
    </Shell>
  );
}

mount(<App />);
