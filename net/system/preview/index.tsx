import type { Site } from "site:routes";
import { index, resolve } from "../../sdk/md/links.ts";
import type { Lazy } from "../../sdk/md/md.ts";
import { front } from "../../sdk/md/md.ts";
import type { Row } from "../../sdk/rows.ts";
import { parts } from "../../shell/active.ts";
import { Contents, heads, hide, show } from "../../shell/parts.tsx";
import type { Context, Layout } from "../../shell/router.ts";
import { render as markup, mathml } from "./md.ts";

/* TYPES */

export type Front = Record<string, string>;

export type PageProps = { row: Row; site: Site; data: Front; html: string; slug: string | undefined };

export type PostProps = {
  row: Pick<Row, "title" | "date" | "lead" | "figure">;
  site: Pick<Site, "title">;
  html: string;
  slug: string | undefined;
};

const sides = new WeakMap<HTMLElement, HTMLElement>();

/* MARKDOWN */

export async function markdown(text: string, opts: Lazy = {}): Promise<string> {
  return markup(text, { ...opts, math: await mathml(text) });
}

/* PARTS */

function Legal({ site }: { site: Pick<Site, "company" | "title" | "since"> }) {
  const year = new Date().getFullYear();
  return (
    <p className="legal fine">
      Copyright © {site.company || site.title} {site.since < year ? `${site.since}-${year}` : year}. All rights
      reserved.
    </p>
  );
}

function Page({ row, site, data, html, slug }: PageProps) {
  const act = data.button && data.link;
  const after = (
    <>
      {act && (
        <p>
          <a className="button primary" href={data.link}>
            {data.button}
          </a>
        </p>
      )}
      {data.legal && <Legal site={site} />}
    </>
  );
  return (
    <parts.prose
      kind="page"
      id={slug}
      title={row.title}
      lead={row.lead}
      figure={data.opener ? (row.figure ?? undefined) : undefined}
      html={html}
      contents={[]}
      after={after}
    />
  );
}

export function Post({ row, site, html, slug }: PostProps) {
  return (
    <parts.prose
      kind="post"
      id={slug}
      title={row.title}
      lead={row.lead}
      by={`${row.date} · ${site.title}`}
      figure={row.figure ?? undefined}
      html={html}
      contents={[]}
    />
  );
}

function Standard({ row, html }: Pick<PageProps, "row" | "html">) {
  return (
    <parts.prose
      kind="page"
      id={row.title.toLowerCase()}
      title={row.title}
      lead={row.lead}
      figure={row.figure ?? undefined}
      html={html}
      contents={[]}
    />
  );
}

/* PAGE */

export const layout: Layout = { right: "contents" };

export async function render(row: Row, site: Site, host: HTMLElement, { text, rows, right }: Context) {
  const { data, body } = front(text);
  const idx = index(rows);
  const was = row.meta.prose;
  const path = row.source ?? "";
  const source = was === "source" ? body.replace(/^# .+\n/, "") : body;
  const html = await markdown(source, { lazy: true, link: (url) => resolve(path, url, idx) });
  const slug = was === "source" ? path.split("/").at(-1)!.replace(/\.md$/, "") : path.split("/").at(-2);
  const Kind = was === "post" ? Post : was === "source" ? Standard : Page;
  show(host, <Kind row={row} site={site} data={data} html={html} slug={slug} />);
  show(right, <Contents items={heads(host)} />);
  sides.set(host, right);
}

export function unmount(host: HTMLElement) {
  hide(sides.get(host) ?? host);
  sides.delete(host);
  hide(host);
}
