# Pages

A tiny page that landed from the CDN, rendered by our own parser.

## What it does

- reads **bold** and *italic* without fuss
- turns markdown into clean html
- refuses anything sketchy

The steps are simple:

1. fetch the file
2. parse the text
3. render the page

Read the [privacy](./privacy.md) note, or visit [mrly](https://mrly.net).

```
cargo test -p mrly pages
```

### this deep heading is refused and shown as plain text
