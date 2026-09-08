import { mount } from '../lib/app.jsx';
import { Shell } from '../../../pkgs/js/mrlyjs/ui/chrome.jsx';
import { tree } from '../lib/tree.js';
import manifest from '../pages.json';

const GROUPS = manifest.shelves.reduce((groups, shelf) => {
  const last = groups[groups.length - 1];
  if (last && last.name === shelf.group) last.shelves.push(shelf);
  else groups.push({ name: shelf.group, shelves: [shelf] });
  return groups;
}, []);

function Shelf({ shelf }) {
  const rows = manifest.pages.filter((page) => page.category === shelf.key);
  if (!rows.length) return null;
  return (
    <>
      <div className="shelf" id={shelf.key}>
        <h2>{shelf.title}</h2>
        <p>{shelf.blurb}</p>
      </div>
      <div className="gallery">
        {rows.map((page) => (
          <a key={page.name} className="tile" href={`/demos/${page.name}/`}>
            <img className="dark" src={`/figures/demo-${page.name}-dark.png`} alt="" width="1024" height="1024" loading="lazy" decoding="async" />
            <img className="light" src={`/figures/demo-${page.name}-light.png`} alt="" width="1024" height="1024" loading="lazy" decoding="async" />
            <h2>{page.title}</h2>
            <p>{page.blurb}</p>
          </a>
        ))}
      </div>
    </>
  );
}

const NODES = tree();

const CONTENTS = GROUPS.flatMap((group) => [
  { id: group.name.toLowerCase(), text: group.name, level: 2 },
  ...group.shelves.map((shelf) => ({ id: shelf.key, text: shelf.title, level: 3 })),
]);

function App() {
  return (
    <Shell route="/demos/" title="The eyes of MrlyMath" lead="Every number and pixel on these pages comes out of the Rust crates through wasm. The browser only draws." tree={NODES} contents={CONTENTS} wide>
      {GROUPS.map((group) => (
        <section key={group.name} id={group.name.toLowerCase()}>
          <h2 className="group">{group.name}</h2>
          {group.shelves.map((shelf) => <Shelf key={shelf.key} shelf={shelf} />)}
        </section>
      ))}
    </Shell>
  );
}

mount(<App />);
