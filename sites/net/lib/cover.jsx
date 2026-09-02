import { createRoot } from 'react-dom/client';
import { ready } from './mrly.js';
import { thumb } from './thumbs.jsx';
import { Wordmark } from '../../ui/chrome.jsx';
import manifest from '../pages.json';

const m = await ready();
const params = new URLSearchParams(location.search);
const key = params.get('k') ?? '';
const page = manifest.pages.find((row) => row.name === key);
const shelf = manifest.shelves.find((row) => row.key === page?.category);
const name = params.get('t') ?? page?.title ?? key;
const kind = params.get('kind') ?? shelf?.title ?? 'Demo';
const by = params.get('by') ?? 'Drawn live by the crates through wasm';

function Cover() {
  return (
    <div className="cover">
      <header><Wordmark /><span className="kind">{kind}</span></header>
      <div className="art live">{thumb(m, key)}</div>
      <footer>
        <h1 className={name.length > 76 ? 'epic' : name.length > 44 ? 'long' : undefined}>{name}</h1>
        <p className="by">{by}</p>
      </footer>
    </div>
  );
}

createRoot(document.getElementById('root')).render(<Cover />);

setTimeout(() => {
  document.documentElement.dataset.theme = 'light';
  delete document.documentElement.dataset.wait;
}, 200);
