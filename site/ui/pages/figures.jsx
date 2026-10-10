import { figures } from 'site:figures';
import { Figure, hide, Lede, show } from '../parts.jsx';

const live = new WeakMap();

export function render(row, site, host) {
  const names = Object.keys(figures).sort();
  show(
    host,
    <>
      <Lede id="figures" title={row.title} lead={`${row.lead} ${names.length} figures.`} />
      <div className="gallery grid" data-eager>
        {names.map((name) => (
          <div className="tile" key={name}>
            <Figure name={name} label={name} caption={name} />
          </div>
        ))}
      </div>
    </>,
  );
}

export function mount(host) {
  const drawn = (event) => {
    const caption = event.target.querySelector('figcaption');
    if (caption) caption.textContent = `${event.target.dataset.figure} ${event.detail.toFixed(1)} ms`;
  };
  host.addEventListener('drawn', drawn);
  live.set(host, () => host.removeEventListener('drawn', drawn));
}

export function unmount(host) {
  live.get(host)?.();
  live.delete(host);
  hide(host);
}
