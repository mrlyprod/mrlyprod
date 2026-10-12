import { index, resolve } from '../kit/md/links.ts';
import { front, mathml, render as md } from '../kit/md/md.ts';
import { Contents, Figure, heads, hide, Lede, show } from './parts.jsx';

const sides = new WeakMap();

function Page({ row, data, html, slug }) {
  const act = data.button && data.link;
  return (
    <>
      {data.opener && row.figure && <Figure className="opener" name={row.figure} label={row.title} />}
      <Lede id={slug} title={row.title} lead={row.lead} />
      <div dangerouslySetInnerHTML={{ __html: html }} />
      {act && <p><a className="button primary" href={data.link}>{data.button}</a></p>}
    </>
  );
}

function Post({ row, site, html, slug }) {
  return (
    <>
      <Figure className="opener" name={row.figure} label={row.title} />
      <div className="plate">
        <h1 id={slug}>{row.title}</h1>
        <p className="by">{row.date} · {site.title}</p>
      </div>
      <div dangerouslySetInnerHTML={{ __html: html }} />
    </>
  );
}

function Standard({ row, html }) {
  return (
    <>
      {row.figure && <Figure className="opener" name={row.figure} label={row.title} />}
      <Lede id={row.title.toLowerCase()} title={row.title} lead={row.lead} />
      <div dangerouslySetInnerHTML={{ __html: html }} />
    </>
  );
}

export const layout = { right: 'contents' };

export async function render(row, site, host, { text, rows, right }) {
  const { data, body } = front(text);
  const idx = index(rows);
  const was = row.meta.prose;
  const source = was === 'source' ? body.replace(/^# .+\n/, '') : body;
  const html = md(source, { math: await mathml(source), lazy: true, link: (url) => resolve(row.source, url, idx) });
  const slug = was === 'source' ? row.source.split('/').at(-1).replace(/\.md$/, '') : row.source.split('/').at(-2);
  const Kind = was === 'post' ? Post : was === 'source' ? Standard : Page;
  show(host, <Kind row={row} site={site} data={data} html={html} slug={slug} />);
  show(right, <Contents items={heads(host)} />);
  sides.set(host, right);
}

export const page = async () => ({ layout, render, unmount });

export function unmount(host) {
  hide(sides.get(host) ?? host);
  sides.delete(host);
  hide(host);
}
