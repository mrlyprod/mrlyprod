import { index, resolve } from '../../kit/md/links.ts';
import { front, render as md } from '../../kit/md/md.ts';
import { Contents, Figure, heads, hide, Lede, show } from '../parts.jsx';

const sides = new WeakMap();

async function math(text) {
  if (!text.includes('$')) return undefined;
  const { default: katex } = await import('katex');
  return (tex, display) => katex.renderToString(tex, { output: 'mathml', throwOnError: false, displayMode: display });
}

const blob = (site) => `https://github.com/${site.git.slug}/blob/${site.git.branch}/`;

function Page({ row, data, html, slug }) {
  const act = data.button && data.link;
  return (
    <>
      {data.figure && <Figure className="opener" name={data.figure} label={row.title} />}
      <Lede id={slug} title={row.title} lead={data.lead ?? row.lead} />
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
      <Figure className="opener" name={row.figure} label={row.title} />
      <Lede id={row.title.toLowerCase()} title={row.title} lead={row.lead} />
      <div dangerouslySetInnerHTML={{ __html: html }} />
    </>
  );
}

export async function render(row, site, host, { text, rows, right }) {
  const { data, body } = front(text);
  const idx = index(rows, blob(site));
  const was = row.meta.was;
  const source = was === 'math' ? body.replace(/^# .+\n/, '') : body;
  const html = md(source, { math: await math(source), lazy: true, link: (url) => resolve(row.source, url, idx) });
  const slug = row.source.split('/').at(was === 'post' ? -2 : -1).replace(/\.md$/, '');
  const Kind = was === 'post' ? Post : was === 'math' ? Standard : Page;
  show(host, <Kind row={row} site={site} data={data} html={html} slug={slug} />);
  show(right, <Contents items={heads(host)} />);
  sides.set(host, right);
}

export function unmount(host) {
  hide(sides.get(host) ?? host);
  sides.delete(host);
  hide(host);
}
