import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { markdown, Post } from "./index.tsx";

test("a post shows its lead under the title and byline", () => {
  const html = renderToStaticMarkup(
    <Post
      row={{ title: "Launch", date: "2026-09-02", lead: "One address.", figure: "blog-launch" }}
      site={{ title: "mrly" }}
      html=""
      slug="launch"
    />,
  );
  expect(html).toContain('<p class="by">2026-09-02 · mrly</p><p class="lead">One address.</p></div>');
});

test("markdown is the one call the other apps make: math as MathML and every link through the resolver", async () => {
  const html = await markdown("$x^2$ and [a](b)", { link: (url) => `/L/${url}` });
  expect(html).toContain("<math");
  expect(html).toContain('<a href="/L/b">a</a>');
});
