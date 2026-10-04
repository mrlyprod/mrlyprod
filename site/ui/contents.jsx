import { fills } from './bar.js';

export function Contents({ items = [] }) {
  if (!fills(items)) return null;
  return (
    <nav className="contents" aria-label="Contents">
      <details>
        <summary>Contents</summary>
        <ol>
          {items.map((item) => (
            <li key={item.id} className={`h${item.level ?? 2}`}>
              <a href={`#${item.id}`}>{item.text}</a>
            </li>
          ))}
        </ol>
      </details>
    </nav>
  );
}
