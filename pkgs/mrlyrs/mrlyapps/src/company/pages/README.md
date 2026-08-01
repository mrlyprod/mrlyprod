# pages

The reader behind the company pages. Give it a slug like privacy or terms and it fetches that page's markdown from the CDN, then shows it either rendered or as raw source. Status is always plain - empty, loading, ready or error - and a fetch that comes back as HTML is called not found instead of being dressed up as a page.

## Using

- Opening a slug starts the fetch; the page arrives as plain markdown text.
- Slugs are strictly letters, digits, dashes, underscores and slashes - anything else is refused before a request is made.
- **flip** switches between the rendered preview and the raw markdown.
- The built-in dummy page opens without any network at all, for trying the renderer.
- A failed fetch keeps its reason, so an error page says why.
