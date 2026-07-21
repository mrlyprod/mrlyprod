# MrlyTree

```
mrlyprod
├── .claude
│   ├── skills
│   ├── worktrees
│   ├── launch.json
│   ├── settings.json
│   └── settings.local.json
├── apps
│   └── web
│       ├── fixtures
│       ├── public
│       │   ├── icons
│       │   │   ├── mrly_192_192.png
│       │   │   └── mrly_512_512.png
│       │   ├── 404.html
│       │   ├── favicon.ico
│       │   ├── manifest.json
│       │   ├── mrlyprod.png
│       │   ├── mrlyprod.svg
│       │   └── robots.txt
│       ├── src
│       │   ├── components
│       │   │   ├── Board.tsx
│       │   │   ├── ColorPicker.tsx
│       │   │   ├── DPad.tsx
│       │   │   ├── GameOver.tsx
│       │   │   ├── GlyphPicker.tsx
│       │   │   ├── Meter.tsx
│       │   │   ├── Pager.tsx
│       │   │   ├── Section.tsx
│       │   │   ├── Shot.tsx
│       │   │   ├── TilePicker.tsx
│       │   │   ├── TimePicker.tsx
│       │   │   ├── fractal.tsx
│       │   │   ├── options.ts
│       │   │   └── palette.tsx
│       │   ├── render
│       │   │   ├── boards.ts
│       │   │   ├── fx.ts
│       │   │   ├── mark.ts
│       │   │   ├── nodes.ts
│       │   │   ├── paint.ts
│       │   │   ├── reconcile.ts
│       │   │   ├── theme.ts
│       │   │   └── wallpaper.ts
│       │   ├── shell
│       │   │   ├── chrome.ts
│       │   │   ├── effects.ts
│       │   │   └── mount.ts
│       │   ├── views
│       │   │   ├── company
│       │   │   │   ├── extras.tsx
│       │   │   │   └── pages.tsx
│       │   │   ├── creativity
│       │   │   │   ├── notes.tsx
│       │   │   │   ├── photos.tsx
│       │   │   │   └── piano.tsx
│       │   │   ├── design
│       │   │   │   ├── colors.tsx
│       │   │   │   ├── emoji.tsx
│       │   │   │   ├── font.tsx
│       │   │   │   ├── pixel.tsx
│       │   │   │   └── text.tsx
│       │   │   ├── games
│       │   │   │   ├── crush.tsx
│       │   │   │   ├── escape.tsx
│       │   │   │   ├── snake.tsx
│       │   │   │   └── tennis.tsx
│       │   │   ├── math
│       │   │   │   ├── bang.tsx
│       │   │   │   ├── life.tsx
│       │   │   │   ├── moire.tsx
│       │   │   │   ├── six.tsx
│       │   │   │   ├── three.tsx
│       │   │   │   ├── tile.tsx
│       │   │   │   └── two.tsx
│       │   │   ├── physics
│       │   │   │   ├── billiards.tsx
│       │   │   │   ├── lasers.tsx
│       │   │   │   └── waves.tsx
│       │   │   ├── puzzles
│       │   │   │   ├── captcha.tsx
│       │   │   │   ├── chess.tsx
│       │   │   │   ├── memory.tsx
│       │   │   │   ├── mines.tsx
│       │   │   │   ├── quiz.tsx
│       │   │   │   ├── ttt.tsx
│       │   │   │   └── twenty48.tsx
│       │   │   ├── system
│       │   │   │   ├── files.tsx
│       │   │   │   ├── iden.tsx
│       │   │   │   ├── log.tsx
│       │   │   │   ├── menu.tsx
│       │   │   │   ├── settings.tsx
│       │   │   │   └── ui.tsx
│       │   │   ├── tools
│       │   │   │   ├── calculator.tsx
│       │   │   │   ├── calendar.tsx
│       │   │   │   ├── clock.tsx
│       │   │   │   ├── dice.tsx
│       │   │   │   ├── hash.tsx
│       │   │   │   └── timer.tsx
│       │   │   ├── toys
│       │   │   │   ├── julia.tsx
│       │   │   │   ├── mandelbrot.tsx
│       │   │   │   ├── matrix.tsx
│       │   │   │   ├── sleep.tsx
│       │   │   │   └── solids.tsx
│       │   │   └── index.ts
│       │   ├── builders.ts
│       │   ├── glyphs.ts
│       │   ├── gpu.ts
│       │   ├── icons.ts
│       │   ├── journal.ts
│       │   ├── jsx.ts
│       │   ├── kernel.ts
│       │   ├── main.ts
│       │   ├── palette.ts
│       │   ├── router.ts
│       │   ├── sound.ts
│       │   ├── types.ts
│       │   └── webgpu.d.ts
│       ├── styles
│       │   ├── boxes.css
│       │   ├── doc.css
│       │   ├── fonts.css
│       │   ├── forms.css
│       │   ├── motion.css
│       │   ├── shell.css
│       │   └── tokens.css
│       ├── bun.lock
│       ├── index.html
│       ├── index.ts
│       ├── mrly.css
│       ├── package.json
│       ├── shot.ts
│       ├── tsconfig.json
│       └── verify.ts
├── cdn
│   └── pages
│       ├── about.md
│       ├── privacy.md
│       └── terms.md
├── docs
│   ├── COMMENTS.md
│   ├── LAYERS.md
│   ├── PICKERS.md
│   └── STATE.md
├── files
│   ├── mrlyfont
│   │   ├── MrlyFont.json
│   │   ├── MrlyFont.ttf
│   │   ├── MrlyFont.woff
│   │   └── MrlyFont.woff2
│   └── vendor
│       ├── LICENSE-display.txt
│       ├── LICENSE-emoji.txt
│       ├── LICENSE-icons.txt
│       ├── LICENSE-mono.txt
│       ├── LICENSE-sans.txt
│       ├── LICENSE-serif.txt
│       ├── display.woff2
│       ├── emoji.0.woff2
│       ├── emoji.1.woff2
│       ├── emoji.2.woff2
│       ├── emoji.3.woff2
│       ├── emoji.4.woff2
│       ├── emoji.5.woff2
│       ├── emoji.6.woff2
│       ├── emoji.7.woff2
│       ├── emoji.8.woff2
│       ├── emoji.9.woff2
│       ├── emoji.css
│       ├── fonts.css
│       ├── icons.css
│       ├── icons.woff2
│       ├── mono.woff2
│       ├── sans.woff2
│       └── serif.woff2
├── lab
│   └── demo.py
├── ops
│   ├── acm.py
│   ├── budget.py
│   ├── cdn.py
│   ├── cloudfront.py
│   ├── deploy.py
│   ├── dns.py
│   └── s3.py
├── pkgs
│   ├── mrlyjs
│   │   ├── src
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── mrlypy
│   │   ├── src
│   │   │   ├── graphics.rs
│   │   │   └── lib.rs
│   │   ├── tests
│   │   │   ├── smoke.py
│   │   │   └── test_kernel.py
│   │   ├── Cargo.toml
│   │   ├── pyproject.toml
│   │   └── uv.lock
│   └── mrlyrs
│       ├── examples
│       │   ├── fixtures.rs
│       │   ├── og.rs
│       │   ├── pages.rs
│       │   └── routes.rs
│       ├── src
│       │   ├── apps
│       │   │   ├── company
│       │   │   │   ├── extras
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── pages
│       │   │   │   │   ├── dummy.md
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── creativity
│       │   │   │   ├── notes
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── photos
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── piano
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── design
│       │   │   │   ├── colors
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── emoji
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── font
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── pixel
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── text
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── games
│       │   │   │   ├── crush
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── escape
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── snake
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── tennis
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── math
│       │   │   │   ├── bang
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── life
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── moire
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── six
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── three
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── tile
│       │   │   │   │   ├── helpers.rs
│       │   │   │   │   ├── mod.rs
│       │   │   │   │   ├── render.rs
│       │   │   │   │   ├── rules.rs
│       │   │   │   │   └── state.rs
│       │   │   │   ├── two
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── physics
│       │   │   │   ├── billiards
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── lasers
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── waves
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── puzzles
│       │   │   │   ├── captcha
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── chess
│       │   │   │   │   ├── mod.rs
│       │   │   │   │   ├── persist.rs
│       │   │   │   │   ├── render.rs
│       │   │   │   │   ├── rules.rs
│       │   │   │   │   ├── setup.rs
│       │   │   │   │   └── tests.rs
│       │   │   │   ├── memory
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── mines
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── quiz
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── ttt
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── twenty48
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── system
│       │   │   │   ├── files
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── iden
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── log
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── menu
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── settings
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── ui
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── tools
│       │   │   │   ├── calculator
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── calendar
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── clock
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── dice
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── hash
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── timer
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   ├── toys
│       │   │   │   ├── julia
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── mandelbrot
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── matrix
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── sleep
│       │   │   │   │   └── mod.rs
│       │   │   │   ├── solids
│       │   │   │   │   └── mod.rs
│       │   │   │   └── mod.rs
│       │   │   └── mod.rs
│       │   ├── core
│       │   │   ├── atoms.rs
│       │   │   ├── boolean.rs
│       │   │   ├── cell.rs
│       │   │   ├── census.rs
│       │   │   ├── codec.rs
│       │   │   ├── colors.rs
│       │   │   ├── emoji.rs
│       │   │   ├── enums.rs
│       │   │   ├── errors.rs
│       │   │   ├── fft.rs
│       │   │   ├── md.rs
│       │   │   ├── mod.rs
│       │   │   ├── paint.rs
│       │   │   ├── ramp.rs
│       │   │   ├── rng.rs
│       │   │   ├── rules.rs
│       │   │   ├── state.rs
│       │   │   ├── tensor.rs
│       │   │   ├── tile.rs
│       │   │   ├── time.rs
│       │   │   └── trig.rs
│       │   ├── crypto
│       │   │   ├── cipher
│       │   │   │   ├── block.rs
│       │   │   │   ├── feistel.rs
│       │   │   │   ├── mod.rs
│       │   │   │   └── schedule.rs
│       │   │   ├── hash
│       │   │   │   ├── config.rs
│       │   │   │   ├── fingerprint.rs
│       │   │   │   ├── hasher.rs
│       │   │   │   ├── metrics.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── permute.rs
│       │   │   │   ├── sbox.rs
│       │   │   │   └── sponge.rs
│       │   │   └── mod.rs
│       │   ├── font
│       │   │   ├── glyphs.rs
│       │   │   ├── letters.rs
│       │   │   ├── mod.rs
│       │   │   ├── models.rs
│       │   │   ├── names.rs
│       │   │   ├── raster.rs
│       │   │   ├── serializer.rs
│       │   │   └── shape.rs
│       │   ├── io
│       │   │   └── mod.rs
│       │   ├── math
│       │   │   ├── bang
│       │   │   │   ├── baseq.rs
│       │   │   │   ├── catalog.rs
│       │   │   │   ├── counting.rs
│       │   │   │   ├── factory.rs
│       │   │   │   ├── mod.rs
│       │   │   │   └── universe.rs
│       │   │   ├── dim
│       │   │   │   ├── census.rs
│       │   │   │   ├── designs.rs
│       │   │   │   ├── geometry.rs
│       │   │   │   ├── graph.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── models.rs
│       │   │   │   ├── painter.rs
│       │   │   │   ├── renderer.rs
│       │   │   │   ├── serializer.rs
│       │   │   │   └── tile.rs
│       │   │   ├── formulas
│       │   │   │   ├── classics.rs
│       │   │   │   ├── counting.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── six.rs
│       │   │   │   └── surface.rs
│       │   │   ├── fractal
│       │   │   │   ├── mod.rs
│       │   │   │   ├── presets.rs
│       │   │   │   └── wayfinder.rs
│       │   │   ├── graph
│       │   │   │   ├── census.rs
│       │   │   │   ├── extract.rs
│       │   │   │   ├── mod.rs
│       │   │   │   └── models.rs
│       │   │   ├── life
│       │   │   │   ├── animate.rs
│       │   │   │   ├── crop.rs
│       │   │   │   ├── heatmap.rs
│       │   │   │   ├── metrics.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── models.rs
│       │   │   │   ├── render.rs
│       │   │   │   ├── sequence.rs
│       │   │   │   ├── step.rs
│       │   │   │   └── story.rs
│       │   │   ├── moire
│       │   │   │   ├── field.rs
│       │   │   │   ├── layer.rs
│       │   │   │   ├── metrics.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── render.rs
│       │   │   │   ├── sample.rs
│       │   │   │   └── stack.rs
│       │   │   ├── pick
│       │   │   │   └── mod.rs
│       │   │   ├── six
│       │   │   │   ├── census.rs
│       │   │   │   ├── designs.rs
│       │   │   │   ├── geometry.rs
│       │   │   │   ├── graph.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── models.rs
│       │   │   │   ├── painter.rs
│       │   │   │   ├── renderer.rs
│       │   │   │   ├── serializer.rs
│       │   │   │   └── tile.rs
│       │   │   ├── space
│       │   │   │   ├── camera.rs
│       │   │   │   ├── mesh.rs
│       │   │   │   ├── mod.rs
│       │   │   │   └── vec.rs
│       │   │   ├── three
│       │   │   │   ├── census.rs
│       │   │   │   ├── designs.rs
│       │   │   │   ├── faces.rs
│       │   │   │   ├── geometry.rs
│       │   │   │   ├── graph.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── models.rs
│       │   │   │   ├── painter.rs
│       │   │   │   ├── renderer.rs
│       │   │   │   ├── serializer.rs
│       │   │   │   └── tile.rs
│       │   │   ├── two
│       │   │   │   ├── artwork.rs
│       │   │   │   ├── census.rs
│       │   │   │   ├── designs.rs
│       │   │   │   ├── geometry.rs
│       │   │   │   ├── graph.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── models.rs
│       │   │   │   ├── painter.rs
│       │   │   │   ├── renderer.rs
│       │   │   │   ├── serializer.rs
│       │   │   │   └── tile.rs
│       │   │   └── mod.rs
│       │   ├── music
│       │   │   ├── cue.rs
│       │   │   ├── mod.rs
│       │   │   ├── render.rs
│       │   │   ├── theory.rs
│       │   │   └── wave.rs
│       │   ├── net
│       │   │   ├── mod.rs
│       │   │   └── registry.rs
│       │   ├── os
│       │   │   ├── kernel
│       │   │   │   ├── os
│       │   │   │   │   ├── capture.rs
│       │   │   │   │   └── persist.rs
│       │   │   │   ├── app.rs
│       │   │   │   ├── envelope.rs
│       │   │   │   ├── iden.rs
│       │   │   │   ├── manifest.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── os.rs
│       │   │   │   ├── set.rs
│       │   │   │   └── testkit.rs
│       │   │   └── mod.rs
│       │   ├── physics
│       │   │   ├── billiards.rs
│       │   │   ├── field.rs
│       │   │   ├── lasers.rs
│       │   │   ├── mask.rs
│       │   │   ├── mod.rs
│       │   │   ├── rng.rs
│       │   │   ├── waves.rs
│       │   │   └── waves_luts.rs
│       │   ├── sys
│       │   │   └── mod.rs
│       │   ├── ui
│       │   │   ├── mark
│       │   │   │   ├── animation.rs
│       │   │   │   ├── frames.rs
│       │   │   │   ├── letters.rs
│       │   │   │   ├── mod.rs
│       │   │   │   └── render.rs
│       │   │   ├── shaders
│       │   │   │   ├── billiards.wgsl
│       │   │   │   ├── julia.wgsl
│       │   │   │   ├── lasers.wgsl
│       │   │   │   ├── mandelbrot.wgsl
│       │   │   │   ├── mesh.wgsl
│       │   │   │   ├── mod.rs
│       │   │   │   ├── vertex.wgsl
│       │   │   │   └── waves.wgsl
│       │   │   ├── card.rs
│       │   │   ├── frame.rs
│       │   │   ├── mod.rs
│       │   │   ├── picker.rs
│       │   │   ├── raster.rs
│       │   │   └── scene.rs
│       │   └── lib.rs
│       ├── tests
│       │   └── golden.rs
│       ├── Cargo.toml
│       └── build.rs
├── utils
│   ├── assets.py
│   ├── config.py
│   ├── font.py
│   ├── git.py
│   ├── ignore.py
│   ├── layers.py
│   ├── parity.py
│   ├── ship.py
│   ├── spaghetti.py
│   └── tree.py
├── .gitignore
├── .python-version
├── CLAUDE.md
├── COMMANDS.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── TREE.md
├── bunfig.toml
├── pyproject.toml
└── uv.lock
```
