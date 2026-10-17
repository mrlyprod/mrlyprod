import { useEffect, useRef, useState } from 'react';
import { index, resolve } from '../../lib/md/links.ts';
import { front, mathml, render as md } from '../../lib/md/md.ts';
import { Icon } from '../../lib/app/controls.jsx';
import { crumbs, paint } from '../../shell/crumbs.js';
import { Figure, Grid, Lede, Lost } from '../../shell/parts.jsx';
import { ROOT, cards, slugOf, sourceOf } from './scene.js';

const TOPICS = '/research.json';

const NONE = index([]);

let listed = null;

function topics() {
  listed ??= fetch(TOPICS).then((reply) => (reply.ok ? reply.json() : Promise.reject(new Error(`research: ${TOPICS} answered ${reply.status}`)))).then((data) => data.topics);
  listed.catch(() => (listed = null));
  return listed;
}

async function read(slug, title) {
  const source = sourceOf(slug);
  const reply = await fetch(`/raw/${source}`);
  if (!reply.ok) throw new Error(`research: /raw/${source} answered ${reply.status}`);
  const { body } = front(await reply.text());
  const text = /^#\s/.test(body.trimStart()) ? body : `# ${title}\n\n${body}`;
  return md(text, { math: await mathml(text), lazy: true, link: (url) => resolve(source, url, NONE) });
}

function Missing({ rows, site }) {
  useEffect(() => {
    const robots = Object.assign(document.createElement('meta'), { name: 'robots', content: 'noindex' });
    document.head.append(robots);
    return () => robots.remove();
  }, []);
  return <Lost site={site} rows={rows} />;
}

export function Gallery({ names = [] }) {
  if (!names.length) return null;
  return (
    <section className="gallery grid" aria-label="Figures">
      {names.map((name) => (
        <div className="tile" key={name}>
          <Figure name={name} label={name} caption={name} />
        </div>
      ))}
    </section>
  );
}

function Topic({ topic, draw, shut }) {
  const box = useRef(null);
  const [html, setHtml] = useState(null);
  useEffect(() => {
    const back = document.activeElement;
    const title = document.title;
    document.title = `${topic.title} · ${title}`;
    box.current.showModal();
    return () => {
      document.title = title;
      if (back?.isConnected) back.focus?.();
    };
  }, []);
  useEffect(() => {
    let on = true;
    read(topic.slug, topic.title).then((made) => on && setHtml(made), (error) => (console.error(error), on && setHtml(false)));
    return () => {
      on = false;
    };
  }, []);
  useEffect(() => {
    if (html) draw().then((one) => one.scan(box.current));
  }, [html]);
  return (
    <dialog className="topic" ref={box} aria-label={topic.title} onClose={shut} onClick={(e) => e.target === e.currentTarget && shut()}>
      <div className="body">
        <div className="close">
          <button type="button" className="icon" aria-label="Close" onClick={shut}>
            <Icon name="close" />
          </button>
        </div>
        {html === false && <p>This topic did not load. <a href={`/git/${sourceOf(topic.slug)}`}>Read it in the code viewer</a>.</p>}
        {html && <article className="prose" dangerouslySetInnerHTML={{ __html: html }} />}
        {html && <Gallery names={topic.figures} />}
      </div>
    </dialog>
  );
}

export function Widget({ rows, site, draw }) {
  const [path, setPath] = useState(() => location.pathname);
  const [list, setList] = useState(null);
  const row = rows.find((one) => one.route === ROOT);
  const grid = useRef(null);
  const leaving = useRef(false);
  const first = useRef(true);
  useEffect(() => {
    topics().then(setList, (error) => (console.error(error), setList(false)));
  }, []);
  useEffect(() => {
    const click = (e) => {
      if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
      const a = e.target instanceof Element ? e.target.closest('a[href]') : null;
      if (!a || a.target || a.hasAttribute('download') || a.origin !== location.origin || !a.pathname.startsWith(ROOT)) return;
      if (a.hash && a.pathname === location.pathname) return;
      e.preventDefault();
      if (a.pathname === location.pathname) return;
      leaving.current = false;
      history.pushState({ over: !slugOf(location.pathname) && Boolean(slugOf(a.pathname)) }, '', a.pathname);
      setPath(a.pathname);
    };
    const pop = () => {
      leaving.current = false;
      if (location.pathname.startsWith(ROOT)) setPath(location.pathname);
    };
    document.addEventListener('click', click);
    addEventListener('popstate', pop);
    return () => {
      document.removeEventListener('click', click);
      removeEventListener('popstate', pop);
    };
  }, []);
  useEffect(() => {
    if (first.current) return void (first.current = false);
    paint(crumbs(path));
    document.querySelector('link[rel="canonical"]')?.setAttribute('href', site.root + path);
  }, [path]);
  useEffect(() => {
    if (list && grid.current) draw().then((one) => one.scan(grid.current));
  }, [list]);
  const shut = () => {
    if (!slugOf(location.pathname)) return;
    if (history.state?.over) {
      if (leaving.current) return;
      leaving.current = true;
      return history.back();
    }
    history.replaceState(null, '', ROOT);
    setPath(ROOT);
  };
  const slug = slugOf(path);
  const topic = list && slug ? list.find((one) => one.slug === slug) : null;
  if (list && slug && !topic) return <Missing rows={rows} site={site} />;
  return (
    <div ref={grid}>
      <Lede id="research" title={row.title} lead={row.lead} />
      {list && <Grid nodes={cards(list)} />}
      {list === false && <p>The topics did not load. Reload to try again.</p>}
      {topic && <Topic key={topic.slug} topic={topic} draw={draw} shut={shut} />}
    </div>
  );
}
