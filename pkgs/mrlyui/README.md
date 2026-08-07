# mrlyui

- The mrly design system: one stylesheet, one component kit.
- React 19 peers; the visual truth lives in the CSS.
- `import "mrlyui/mrly.css"`, then compose the components.
- Fonts ride cdn.mrly.net by default; `utils/brand.py bundle` bakes them local.
- After bundling, `import "mrlyui/local.css"` needs no network at all.
- The sink shows every piece on one page: `bun run dev` here.
- Boxes compose; skin and shape stay orthogonal.
- Tokens cascade from one `--unit`; retune by scope, never fork.
- Ink and paper invert for interaction; accent means meaning.
