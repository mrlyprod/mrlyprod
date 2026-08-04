# MrlyTree

```
mrlyprod
├── apps
│   ├── cli
│   │   ├── src
│   │   │   ├── main.rs
│   │   │   ├── mcp.rs
│   │   │   ├── term.rs
│   │   │   └── tui.rs
│   │   ├── tests
│   │   │   ├── screenplays
│   │   │   │   ├── chess.jsonl
│   │   │   │   ├── mandelbrot.jsonl
│   │   │   │   ├── menu.jsonl
│   │   │   │   ├── settings.jsonl
│   │   │   │   ├── snake.jsonl
│   │   │   │   └── twenty48.jsonl
│   │   │   └── cli.rs
│   │   └── Cargo.toml
│   ├── git
│   │   ├── public
│   │   │   ├── icons
│   │   │   │   ├── mrly_192_192.png
│   │   │   │   └── mrly_512_512.png
│   │   │   ├── boot.js
│   │   │   ├── favicon.ico
│   │   │   ├── mark.svg
│   │   │   ├── robots.txt
│   │   │   └── site.webmanifest
│   │   ├── scripts
│   │   │   ├── build.ts
│   │   │   ├── dev.ts
│   │   │   ├── links.ts
│   │   │   └── scan.ts
│   │   ├── src
│   │   │   ├── components
│   │   │   │   ├── Grip.tsx
│   │   │   │   ├── Listing.tsx
│   │   │   │   ├── More.tsx
│   │   │   │   ├── Panel.tsx
│   │   │   │   ├── Search.tsx
│   │   │   │   ├── Shell.tsx
│   │   │   │   ├── Skeleton.tsx
│   │   │   │   ├── Status.tsx
│   │   │   │   └── Tree.tsx
│   │   │   ├── lib
│   │   │   │   ├── code.ts
│   │   │   │   ├── data.ts
│   │   │   │   ├── find.ts
│   │   │   │   ├── langs.ts
│   │   │   │   ├── md.ts
│   │   │   │   ├── panes.ts
│   │   │   │   ├── repo.ts
│   │   │   │   ├── site.ts
│   │   │   │   ├── text.ts
│   │   │   │   ├── theme.ts
│   │   │   │   └── tree.ts
│   │   │   ├── views
│   │   │   │   ├── NotFound.tsx
│   │   │   │   └── Route.tsx
│   │   │   ├── App.tsx
│   │   │   └── main.tsx
│   │   ├── index.html
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── vite.config.ts
│   ├── jsx
│   │   └── src
│   │       └── apps
│   ├── net
│   │   ├── blog
│   │   │   ├── README.md
│   │   │   ├── millennium.md
│   │   │   └── vicsek.md
│   │   ├── pages
│   │   │   ├── ABOUT.md
│   │   │   ├── BRICKS.md
│   │   │   ├── CONTACT.md
│   │   │   ├── HOME.md
│   │   │   ├── MATH.md
│   │   │   ├── PRIVACY.md
│   │   │   ├── RESEARCH.md
│   │   │   ├── SCULPTURES.md
│   │   │   ├── SHEETS.md
│   │   │   └── TERMS.md
│   │   ├── public
│   │   │   ├── icons
│   │   │   │   ├── mrly_192_192.png
│   │   │   │   └── mrly_512_512.png
│   │   │   ├── boot.js
│   │   │   ├── favicon.ico
│   │   │   ├── install.sh
│   │   │   ├── manifest.json
│   │   │   ├── mark.svg
│   │   │   └── robots.txt
│   │   ├── scripts
│   │   │   ├── build.ts
│   │   │   ├── dev.ts
│   │   │   ├── links.ts
│   │   │   └── scan.ts
│   │   ├── src
│   │   │   ├── components
│   │   │   │   ├── Build.tsx
│   │   │   │   ├── Cart.tsx
│   │   │   │   ├── Landing.tsx
│   │   │   │   ├── Menu.tsx
│   │   │   │   ├── Panel.tsx
│   │   │   │   └── Shell.tsx
│   │   │   ├── lib
│   │   │   │   ├── data.ts
│   │   │   │   ├── md.ts
│   │   │   │   ├── orders.ts
│   │   │   │   ├── site.ts
│   │   │   │   └── text.ts
│   │   │   ├── views
│   │   │   │   ├── NotFound.tsx
│   │   │   │   └── Page.tsx
│   │   │   ├── App.tsx
│   │   │   └── main.tsx
│   │   ├── index.html
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── vite.config.ts
│   ├── web
│   │   ├── fixtures
│   │   ├── public
│   │   │   ├── icons
│   │   │   │   ├── mrly_192_192.png
│   │   │   │   └── mrly_512_512.png
│   │   │   ├── 404.html
│   │   │   ├── emoji.css
│   │   │   ├── favicon.ico
│   │   │   ├── fonts.css
│   │   │   ├── icons.css
│   │   │   ├── manifest.json
│   │   │   ├── mrlyprod.png
│   │   │   ├── mrlyprod.svg
│   │   │   └── robots.txt
│   │   ├── src
│   │   │   ├── components
│   │   │   │   ├── Board.tsx
│   │   │   │   ├── DPad.tsx
│   │   │   │   ├── GameOver.tsx
│   │   │   │   ├── Meter.tsx
│   │   │   │   ├── Pager.tsx
│   │   │   │   ├── Section.tsx
│   │   │   │   ├── Shot.tsx
│   │   │   │   ├── colorpicker.tsx
│   │   │   │   ├── fractal.tsx
│   │   │   │   ├── library.tsx
│   │   │   │   ├── options.ts
│   │   │   │   └── palette.tsx
│   │   │   ├── gen
│   │   │   │   ├── mark.json
│   │   │   │   ├── palette.json
│   │   │   │   └── shaders.json
│   │   │   ├── render
│   │   │   │   ├── boards.ts
│   │   │   │   ├── fx.ts
│   │   │   │   ├── mark.ts
│   │   │   │   ├── nodes.ts
│   │   │   │   ├── orbit.ts
│   │   │   │   ├── paint.ts
│   │   │   │   ├── reconcile.ts
│   │   │   │   ├── theme.ts
│   │   │   │   └── wallpaper.ts
│   │   │   ├── shell
│   │   │   │   ├── chrome.ts
│   │   │   │   ├── effects.ts
│   │   │   │   └── mount.ts
│   │   │   ├── views
│   │   │   │   ├── company
│   │   │   │   │   └── extras.tsx
│   │   │   │   ├── creativity
│   │   │   │   │   ├── notes.tsx
│   │   │   │   │   ├── photos.tsx
│   │   │   │   │   └── piano.tsx
│   │   │   │   ├── design
│   │   │   │   │   ├── colors.tsx
│   │   │   │   │   ├── emoji.tsx
│   │   │   │   │   ├── font.tsx
│   │   │   │   │   └── pixel.tsx
│   │   │   │   ├── games
│   │   │   │   │   ├── crush.tsx
│   │   │   │   │   ├── escape.tsx
│   │   │   │   │   ├── snake.tsx
│   │   │   │   │   └── tennis.tsx
│   │   │   │   ├── math
│   │   │   │   │   ├── bang.tsx
│   │   │   │   │   ├── life.tsx
│   │   │   │   │   ├── moire.tsx
│   │   │   │   │   ├── six.tsx
│   │   │   │   │   ├── three.tsx
│   │   │   │   │   ├── tile.tsx
│   │   │   │   │   └── two.tsx
│   │   │   │   ├── physics
│   │   │   │   │   ├── billiards.tsx
│   │   │   │   │   ├── lasers.tsx
│   │   │   │   │   └── waves.tsx
│   │   │   │   ├── puzzles
│   │   │   │   │   ├── captcha.tsx
│   │   │   │   │   ├── chess.tsx
│   │   │   │   │   ├── memory.tsx
│   │   │   │   │   ├── mines.tsx
│   │   │   │   │   ├── quiz.tsx
│   │   │   │   │   ├── ttt.tsx
│   │   │   │   │   └── twenty48.tsx
│   │   │   │   ├── system
│   │   │   │   │   ├── files.tsx
│   │   │   │   │   ├── iden.tsx
│   │   │   │   │   ├── log.tsx
│   │   │   │   │   ├── menu.tsx
│   │   │   │   │   ├── settings.tsx
│   │   │   │   │   └── ui.tsx
│   │   │   │   ├── tools
│   │   │   │   │   ├── calculator.tsx
│   │   │   │   │   ├── calendar.tsx
│   │   │   │   │   ├── clock.tsx
│   │   │   │   │   ├── dice.tsx
│   │   │   │   │   ├── hash.tsx
│   │   │   │   │   └── timer.tsx
│   │   │   │   ├── toys
│   │   │   │   │   ├── julia.tsx
│   │   │   │   │   ├── mandelbrot.tsx
│   │   │   │   │   ├── matrix.tsx
│   │   │   │   │   ├── sleep.tsx
│   │   │   │   │   └── solids.tsx
│   │   │   │   └── index.ts
│   │   │   ├── builders.ts
│   │   │   ├── glyphs.ts
│   │   │   ├── icons.ts
│   │   │   ├── journal.ts
│   │   │   ├── jsx.ts
│   │   │   ├── kernel.ts
│   │   │   ├── main.ts
│   │   │   ├── palette.ts
│   │   │   ├── pwa.ts
│   │   │   ├── reads.ts
│   │   │   ├── roller.ts
│   │   │   ├── router.ts
│   │   │   ├── skin.tsx
│   │   │   ├── sound.ts
│   │   │   └── types.ts
│   │   ├── styles
│   │   │   ├── boxes.css
│   │   │   ├── fonts.css
│   │   │   ├── forms.css
│   │   │   ├── motion.css
│   │   │   ├── shell.css
│   │   │   └── tokens.css
│   │   ├── index.html
│   │   ├── index.ts
│   │   ├── mrly.css
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── verify.ts
│   └── README.md
├── files
│   ├── brand
│   │   ├── icons
│   │   │   ├── mrly_192_192.png
│   │   │   └── mrly_512_512.png
│   │   ├── favicon.ico
│   │   ├── mark.svg
│   │   ├── mrlyprod.png
│   │   └── mrlyprod.svg
│   ├── mrlyfont
│   │   ├── MrlyFont.json
│   │   ├── MrlyFont.ttf
│   │   ├── MrlyFont.woff
│   │   └── MrlyFont.woff2
│   └── vendor
│       ├── seti
│       │   ├── LICENSE-seti.txt
│       │   ├── seti.woff
│       │   └── seti.woff2
│       ├── simple-icons
│       │   ├── LICENSE.md
│       │   ├── discord.svg
│       │   ├── github.svg
│       │   ├── instagram.svg
│       │   ├── reddit.svg
│       │   ├── tiktok.svg
│       │   ├── x.svg
│       │   └── youtube.svg
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
│       ├── emoji.ttf
│       ├── fonts.css
│       ├── icons.woff2
│       ├── mono.woff2
│       ├── sans.woff2
│       ├── serif.woff2
│       ├── site.woff2
│       ├── symbols.codepoints
│       ├── symbols.ttf
│       └── symbols2.ttf
├── pkgs
│   ├── mrlycss
│   │   ├── base.css
│   │   ├── code.css
│   │   ├── colors.css
│   │   ├── doc.css
│   │   ├── faces.css
│   │   ├── fonts.css
│   │   ├── mrly.css
│   │   ├── package.json
│   │   ├── seti.css
│   │   ├── site.css
│   │   └── tokens.css
│   ├── mrlydom
│   │   ├── src
│   │   │   ├── Aside.tsx
│   │   │   ├── Brand.tsx
│   │   │   ├── Crumbs.tsx
│   │   │   ├── Fold.tsx
│   │   │   ├── Footer.tsx
│   │   │   ├── Glyph.tsx
│   │   │   ├── Letters.tsx
│   │   │   ├── head.ts
│   │   │   ├── index.ts
│   │   │   ├── router.ts
│   │   │   └── theme.ts
│   │   ├── package.json
│   │   └── tsconfig.json
│   ├── mrlygpu
│   │   ├── src
│   │   │   ├── index.ts
│   │   │   └── webgpu.ts
│   │   ├── package.json
│   │   └── tsconfig.json
│   ├── mrlyjs
│   │   └── web
│   │       ├── src
│   │       │   └── lib.rs
│   │       └── Cargo.toml
│   ├── mrlypy
│   │   └── web
│   │       ├── src
│   │       │   └── lib.rs
│   │       ├── tests
│   │       │   ├── smoke.py
│   │       │   └── test_kernel.py
│   │       ├── Cargo.toml
│   │       └── pyproject.toml
│   ├── mrlyrs
│   │   ├── mrlyapps
│   │   │   ├── src
│   │   │   │   ├── company
│   │   │   │   │   ├── extras
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── creativity
│   │   │   │   │   ├── notes
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── photos
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── piano
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── design
│   │   │   │   │   ├── colors
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── emoji
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   ├── data.rs
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── font
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── pixel
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── games
│   │   │   │   │   ├── crush
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── escape
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── snake
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── tennis
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── math
│   │   │   │   │   ├── bang
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── life
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── moire
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── six
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── three
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── tile
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   ├── helpers.rs
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   ├── render.rs
│   │   │   │   │   │   ├── rules.rs
│   │   │   │   │   │   └── state.rs
│   │   │   │   │   ├── two
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── physics
│   │   │   │   │   ├── billiards
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── lasers
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── waves
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── puzzles
│   │   │   │   │   ├── captcha
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── chess
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   ├── persist.rs
│   │   │   │   │   │   ├── render.rs
│   │   │   │   │   │   ├── rules.rs
│   │   │   │   │   │   ├── setup.rs
│   │   │   │   │   │   └── tests.rs
│   │   │   │   │   ├── memory
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── mines
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── quiz
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── ttt
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── twenty48
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── system
│   │   │   │   │   ├── files
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── iden
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── log
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── menu
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── settings
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── ui
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── tools
│   │   │   │   │   ├── calculator
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── calendar
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── clock
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── dice
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── hash
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── timer
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── toys
│   │   │   │   │   ├── julia
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── mandelbrot
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── matrix
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── sleep
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── solids
│   │   │   │   │   │   ├── README.md
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── lib.rs
│   │   │   ├── tests
│   │   │   │   └── kernel.rs
│   │   │   └── Cargo.toml
│   │   ├── mrlycore
│   │   │   ├── src
│   │   │   │   ├── atoms.rs
│   │   │   │   ├── cell.rs
│   │   │   │   ├── chacha.rs
│   │   │   │   ├── codec.rs
│   │   │   │   ├── colors.rs
│   │   │   │   ├── enums.rs
│   │   │   │   ├── errors.rs
│   │   │   │   ├── image.rs
│   │   │   │   ├── io.rs
│   │   │   │   ├── json.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── paint.rs
│   │   │   │   ├── ramp.rs
│   │   │   │   ├── rng.rs
│   │   │   │   ├── state.rs
│   │   │   │   ├── tensor.rs
│   │   │   │   ├── tile.rs
│   │   │   │   ├── time.rs
│   │   │   │   └── trig.rs
│   │   │   ├── tests
│   │   │   │   └── json.rs
│   │   │   └── Cargo.toml
│   │   ├── mrlyfont
│   │   │   ├── src
│   │   │   │   ├── glyphs.rs
│   │   │   │   ├── letters.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── models.rs
│   │   │   │   ├── names.rs
│   │   │   │   ├── raster.rs
│   │   │   │   ├── serializer.rs
│   │   │   │   └── shape.rs
│   │   │   └── Cargo.toml
│   │   ├── mrlymath
│   │   │   ├── src
│   │   │   │   ├── bang
│   │   │   │   │   ├── baseq.rs
│   │   │   │   │   ├── catalog.rs
│   │   │   │   │   ├── counting.rs
│   │   │   │   │   ├── factory.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── universe.rs
│   │   │   │   ├── crypto
│   │   │   │   │   ├── cipher
│   │   │   │   │   │   ├── block.rs
│   │   │   │   │   │   ├── feistel.rs
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   └── schedule.rs
│   │   │   │   │   ├── hash
│   │   │   │   │   │   ├── config.rs
│   │   │   │   │   │   ├── fingerprint.rs
│   │   │   │   │   │   ├── hasher.rs
│   │   │   │   │   │   ├── metrics.rs
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   ├── permute.rs
│   │   │   │   │   │   ├── sbox.rs
│   │   │   │   │   │   └── sponge.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── dim
│   │   │   │   │   ├── census.rs
│   │   │   │   │   ├── designs.rs
│   │   │   │   │   ├── geometry.rs
│   │   │   │   │   ├── graph.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── models.rs
│   │   │   │   │   ├── painter.rs
│   │   │   │   │   ├── renderer.rs
│   │   │   │   │   ├── serializer.rs
│   │   │   │   │   └── tile.rs
│   │   │   │   ├── formulas
│   │   │   │   │   ├── classics.rs
│   │   │   │   │   ├── counting.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── six.rs
│   │   │   │   │   └── surface.rs
│   │   │   │   ├── fractal
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── presets.rs
│   │   │   │   │   └── wayfinder.rs
│   │   │   │   ├── graph
│   │   │   │   │   ├── census.rs
│   │   │   │   │   ├── extract.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── models.rs
│   │   │   │   ├── life
│   │   │   │   │   ├── animate.rs
│   │   │   │   │   ├── crop.rs
│   │   │   │   │   ├── heatmap.rs
│   │   │   │   │   ├── metrics.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── models.rs
│   │   │   │   │   ├── render.rs
│   │   │   │   │   ├── sequence.rs
│   │   │   │   │   ├── step.rs
│   │   │   │   │   └── story.rs
│   │   │   │   ├── moire
│   │   │   │   │   ├── field.rs
│   │   │   │   │   ├── layer.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── render.rs
│   │   │   │   │   ├── sample.rs
│   │   │   │   │   └── stack.rs
│   │   │   │   ├── physics
│   │   │   │   │   ├── billiards.rs
│   │   │   │   │   ├── field.rs
│   │   │   │   │   ├── lasers.rs
│   │   │   │   │   ├── mask.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── rng.rs
│   │   │   │   │   ├── waves.rs
│   │   │   │   │   └── waves_luts.rs
│   │   │   │   ├── pick
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── six
│   │   │   │   │   ├── census.rs
│   │   │   │   │   ├── designs.rs
│   │   │   │   │   ├── geometry.rs
│   │   │   │   │   ├── graph.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── models.rs
│   │   │   │   │   ├── painter.rs
│   │   │   │   │   ├── renderer.rs
│   │   │   │   │   ├── serializer.rs
│   │   │   │   │   └── tile.rs
│   │   │   │   ├── space
│   │   │   │   │   ├── mesh.rs
│   │   │   │   │   ├── mesh.wgsl
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── pack.rs
│   │   │   │   │   └── vec.rs
│   │   │   │   ├── three
│   │   │   │   │   ├── census.rs
│   │   │   │   │   ├── designs.rs
│   │   │   │   │   ├── faces.rs
│   │   │   │   │   ├── geometry.rs
│   │   │   │   │   ├── graph.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── models.rs
│   │   │   │   │   ├── painter.rs
│   │   │   │   │   ├── renderer.rs
│   │   │   │   │   ├── serializer.rs
│   │   │   │   │   ├── sheets.rs
│   │   │   │   │   └── tile.rs
│   │   │   │   ├── two
│   │   │   │   │   ├── artwork.rs
│   │   │   │   │   ├── census.rs
│   │   │   │   │   ├── designs.rs
│   │   │   │   │   ├── geometry.rs
│   │   │   │   │   ├── graph.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── models.rs
│   │   │   │   │   ├── painter.rs
│   │   │   │   │   ├── renderer.rs
│   │   │   │   │   ├── serializer.rs
│   │   │   │   │   └── tile.rs
│   │   │   │   ├── boolean.rs
│   │   │   │   ├── census.rs
│   │   │   │   ├── fft.rs
│   │   │   │   ├── lib.rs
│   │   │   │   └── rules.rs
│   │   │   ├── tests
│   │   │   │   └── atoms.rs
│   │   │   └── Cargo.toml
│   │   ├── mrlymusic
│   │   │   ├── src
│   │   │   │   ├── cue.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── render.rs
│   │   │   │   ├── theory.rs
│   │   │   │   └── wave.rs
│   │   │   └── Cargo.toml
│   │   ├── mrlyos
│   │   │   ├── src
│   │   │   │   ├── kernel
│   │   │   │   │   ├── os
│   │   │   │   │   │   ├── capture.rs
│   │   │   │   │   │   └── persist.rs
│   │   │   │   │   ├── app.rs
│   │   │   │   │   ├── envelope.rs
│   │   │   │   │   ├── iden.rs
│   │   │   │   │   ├── manifest.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── os.rs
│   │   │   │   │   ├── set.rs
│   │   │   │   │   ├── shape.rs
│   │   │   │   │   └── testkit.rs
│   │   │   │   └── lib.rs
│   │   │   ├── Cargo.toml
│   │   │   └── build.rs
│   │   ├── mrlyui
│   │   │   ├── src
│   │   │   │   ├── mark
│   │   │   │   │   ├── animation.rs
│   │   │   │   │   ├── frames.rs
│   │   │   │   │   ├── letters.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── render.rs
│   │   │   │   ├── shaders
│   │   │   │   │   ├── billiards.wgsl
│   │   │   │   │   ├── julia.wgsl
│   │   │   │   │   ├── lasers.wgsl
│   │   │   │   │   ├── mandelbrot.wgsl
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── vertex.wgsl
│   │   │   │   │   └── waves.wgsl
│   │   │   │   ├── skin
│   │   │   │   │   ├── chess.rs
│   │   │   │   │   ├── memory.rs
│   │   │   │   │   ├── mines.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── ttt.rs
│   │   │   │   │   ├── twenty48.rs
│   │   │   │   │   └── two.rs
│   │   │   │   ├── draw.rs
│   │   │   │   ├── frame.rs
│   │   │   │   └── lib.rs
│   │   │   ├── tests
│   │   │   └── Cargo.toml
│   │   └── mrlyweb
│   │       ├── examples
│   │       │   ├── bake.rs
│   │       │   └── fixtures.rs
│   │       ├── src
│   │       │   ├── card.rs
│   │       │   ├── goose.rs
│   │       │   ├── lib.rs
│   │       │   └── registry.rs
│   │       ├── tests
│   │       │   ├── card.rs
│   │       │   ├── golden.rs
│   │       │   ├── goose.rs
│   │       │   └── keys.rs
│   │       └── Cargo.toml
│   └── README.md
├── utils
│   ├── brand.py
│   ├── config.py
│   ├── doors.py
│   ├── font.py
│   ├── ignore.py
│   ├── layers.py
│   ├── shot.ts
│   ├── spaghetti.py
│   ├── stats.py
│   ├── test.py
│   ├── tree.py
│   └── tsconfig.json
├── .gitignore
├── .python-version
├── CLONE.md
├── COMMANDS.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── INSTALL.md
├── LICENSE
├── README.md
├── STATS.md
├── TREE.md
├── bun.lock
├── bunfig.toml
├── clone.sh
├── git.py
├── package.json
├── pyproject.toml
├── rust-toolchain.toml
├── tsconfig.base.json
└── uv.lock
```
