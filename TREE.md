# MrlyTree

```
mrlyprod
├── files
│   ├── brand
│   │   ├── icons
│   │   │   ├── mrly_192_192.png
│   │   │   └── mrly_512_512.png
│   │   ├── favicon.ico
│   │   ├── mark.svg
│   │   ├── mrlyprod.png
│   │   └── mrlyprod.svg
│   ├── logos
│   │   ├── mrlycolors.txt
│   │   ├── mrlygrid.png
│   │   ├── mrlygrid.svg
│   │   ├── mrlylogo.png
│   │   ├── mrlylogo.svg
│   │   ├── mrlyprod.gif
│   │   ├── mrlyprod.mp4
│   │   ├── mrlyprod.png
│   │   └── mrlyprod.svg
│   ├── vendor
│   │   ├── seti
│   │   │   ├── LICENSE-seti.txt
│   │   │   ├── seti.woff
│   │   │   └── seti.woff2
│   │   ├── simple-icons
│   │   │   ├── LICENSE.md
│   │   │   ├── discord.svg
│   │   │   ├── github.svg
│   │   │   ├── instagram.svg
│   │   │   ├── reddit.svg
│   │   │   ├── tiktok.svg
│   │   │   ├── x.svg
│   │   │   └── youtube.svg
│   │   ├── LICENSE-display.txt
│   │   ├── LICENSE-emoji.txt
│   │   ├── LICENSE-icons.txt
│   │   ├── LICENSE-mono.txt
│   │   ├── LICENSE-sans.txt
│   │   ├── LICENSE-serif.txt
│   │   ├── display.woff2
│   │   ├── emoji.0.woff2
│   │   ├── emoji.1.woff2
│   │   ├── emoji.2.woff2
│   │   ├── emoji.3.woff2
│   │   ├── emoji.4.woff2
│   │   ├── emoji.5.woff2
│   │   ├── emoji.6.woff2
│   │   ├── emoji.7.woff2
│   │   ├── emoji.8.woff2
│   │   ├── emoji.9.woff2
│   │   ├── emoji.css
│   │   ├── emoji.ttf
│   │   ├── fonts.css
│   │   ├── icons.woff2
│   │   ├── mono.woff2
│   │   ├── sans.woff2
│   │   ├── serif.woff2
│   │   ├── site.woff2
│   │   ├── symbols.codepoints
│   │   ├── symbols.ttf
│   │   ├── symbols2.ttf
│   │   └── ui.woff2
│   └── MIT.txt
├── lambdas
│   └── mrlygame
│       ├── README.md
│       ├── handler.py
│       └── video.py
├── pkgs
│   ├── js
│   │   ├── mrlygpu
│   │   │   ├── src
│   │   │   │   ├── index.ts
│   │   │   │   └── webgpu.ts
│   │   │   ├── package.json
│   │   │   └── tsconfig.json
│   │   ├── mrlyui
│   │   │   ├── demo
│   │   │   │   ├── src
│   │   │   │   │   ├── Sink.tsx
│   │   │   │   │   ├── main.tsx
│   │   │   │   │   └── sink.css
│   │   │   │   └── index.html
│   │   │   ├── src
│   │   │   │   ├── gen
│   │   │   │   │   ├── mark.json
│   │   │   │   │   └── mrlyfont.json
│   │   │   │   ├── Alert.tsx
│   │   │   │   ├── Autocomplete.tsx
│   │   │   │   ├── Badge.tsx
│   │   │   │   ├── Banner.tsx
│   │   │   │   ├── Board.tsx
│   │   │   │   ├── Box.tsx
│   │   │   │   ├── Brand.tsx
│   │   │   │   ├── Button.tsx
│   │   │   │   ├── Calendar.tsx
│   │   │   │   ├── Canvas.tsx
│   │   │   │   ├── Checkbox.tsx
│   │   │   │   ├── Chip.tsx
│   │   │   │   ├── Choice.tsx
│   │   │   │   ├── ColorPicker.tsx
│   │   │   │   ├── Crumbs.tsx
│   │   │   │   ├── Drawer.tsx
│   │   │   │   ├── Dropdown.tsx
│   │   │   │   ├── Field.tsx
│   │   │   │   ├── Fold.tsx
│   │   │   │   ├── Footer.tsx
│   │   │   │   ├── Glyphs.tsx
│   │   │   │   ├── Grip.tsx
│   │   │   │   ├── Header.tsx
│   │   │   │   ├── Image.tsx
│   │   │   │   ├── Input.tsx
│   │   │   │   ├── Label.tsx
│   │   │   │   ├── Letters.tsx
│   │   │   │   ├── Mark.tsx
│   │   │   │   ├── Modal.tsx
│   │   │   │   ├── Pager.tsx
│   │   │   │   ├── Panes.tsx
│   │   │   │   ├── Popover.tsx
│   │   │   │   ├── Progress.tsx
│   │   │   │   ├── Radio.tsx
│   │   │   │   ├── Search.tsx
│   │   │   │   ├── Select.tsx
│   │   │   │   ├── Setting.tsx
│   │   │   │   ├── Sheet.tsx
│   │   │   │   ├── Skeleton.tsx
│   │   │   │   ├── Slider.tsx
│   │   │   │   ├── Spinner.tsx
│   │   │   │   ├── Splash.tsx
│   │   │   │   ├── Stepper.tsx
│   │   │   │   ├── Tabs.tsx
│   │   │   │   ├── Text.tsx
│   │   │   │   ├── Textarea.tsx
│   │   │   │   ├── Toast.tsx
│   │   │   │   ├── Toggle.tsx
│   │   │   │   ├── Tooltip.tsx
│   │   │   │   ├── Tree.tsx
│   │   │   │   ├── colors.ts
│   │   │   │   ├── index.ts
│   │   │   │   ├── lib.ts
│   │   │   │   ├── pane.ts
│   │   │   │   ├── prefs.ts
│   │   │   │   ├── route.ts
│   │   │   │   ├── seti.ts
│   │   │   │   ├── sound.ts
│   │   │   │   ├── theme.ts
│   │   │   │   └── variant.ts
│   │   │   ├── styles
│   │   │   │   ├── boxes.css
│   │   │   │   ├── code.css
│   │   │   │   ├── colors.css
│   │   │   │   ├── controls.css
│   │   │   │   ├── doc.css
│   │   │   │   ├── faces.css
│   │   │   │   ├── feedback.css
│   │   │   │   ├── glyphs.css
│   │   │   │   ├── local.css
│   │   │   │   ├── motion.css
│   │   │   │   ├── mrly.css
│   │   │   │   ├── nav.css
│   │   │   │   ├── overlay.css
│   │   │   │   ├── panes.css
│   │   │   │   ├── pickers.css
│   │   │   │   ├── reset.css
│   │   │   │   ├── seti.css
│   │   │   │   ├── text.css
│   │   │   │   └── tokens.css
│   │   │   ├── README.md
│   │   │   ├── boot.js
│   │   │   ├── package.json
│   │   │   ├── tsconfig.json
│   │   │   └── vite.config.ts
│   │   └── web
│   │       ├── src
│   │       │   └── lib.rs
│   │       ├── Cargo.toml
│   │       ├── LICENSE
│   │       └── README.md
│   ├── py
│   │   └── web
│   │       ├── src
│   │       │   └── lib.rs
│   │       ├── tests
│   │       │   ├── smoke.py
│   │       │   └── test_kernel.py
│   │       ├── Cargo.toml
│   │       ├── LICENSE
│   │       ├── README.md
│   │       └── pyproject.toml
│   ├── rs
│   │   ├── apps
│   │   │   ├── mrlyarc
│   │   │   │   ├── corpus
│   │   │   │   │   ├── LICENSE-ONE
│   │   │   │   │   ├── LICENSE-TWO
│   │   │   │   │   ├── SOURCES.md
│   │   │   │   │   ├── one.bin
│   │   │   │   │   └── two.bin
│   │   │   │   ├── examples
│   │   │   │   │   └── vendor.rs
│   │   │   │   ├── src
│   │   │   │   │   ├── corpus.rs
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlybang
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlycalculator
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlycalendar
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlycaptcha
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlychess
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   ├── persist.rs
│   │   │   │   │   ├── rules.rs
│   │   │   │   │   ├── setup.rs
│   │   │   │   │   ├── skin.rs
│   │   │   │   │   └── tests.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyclock
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlycolors
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlycrush
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlydice
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyemojis
│   │   │   │   ├── src
│   │   │   │   │   ├── data.rs
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyescape
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyfiles
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyfonts
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlygraph
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyhash
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyiden
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyjulia
│   │   │   │   ├── src
│   │   │   │   │   ├── julia.wgsl
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlylife
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlylog
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlymandelbrot
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── mandelbrot.wgsl
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlymatrix
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlymemory
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlymenu
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlymines
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlymoire
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── moire.wgsl
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlynotes
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlypaint
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyphotos
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyquiz
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlysettings
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlysix
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlysleep
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── sleep.wgsl
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlysnake
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlysolids
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlystudio
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlytennis
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlythree
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlytile
│   │   │   │   ├── src
│   │   │   │   │   ├── helpers.rs
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   ├── render.rs
│   │   │   │   │   ├── rules.rs
│   │   │   │   │   ├── skin.rs
│   │   │   │   │   └── state.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlytimer
│   │   │   │   ├── src
│   │   │   │   │   └── lib.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlyttt
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   ├── mrlytwenty48
│   │   │   │   ├── src
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── skin.rs
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── LICENSE
│   │   │   │   └── README.md
│   │   │   └── mrlytwo
│   │   │       ├── src
│   │   │       │   ├── lib.rs
│   │   │       │   └── skin.rs
│   │   │       ├── Cargo.toml
│   │   │       ├── LICENSE
│   │   │       └── README.md
│   │   ├── mrlycli
│   │   │   ├── src
│   │   │   │   ├── main.rs
│   │   │   │   ├── mcp.rs
│   │   │   │   ├── term.rs
│   │   │   │   └── tui.rs
│   │   │   ├── tests
│   │   │   │   ├── frames
│   │   │   │   │   ├── chess.20x6.txt
│   │   │   │   │   ├── chess.80x24.txt
│   │   │   │   │   ├── mandelbrot.20x6.txt
│   │   │   │   │   ├── mandelbrot.80x24.txt
│   │   │   │   │   ├── menu.20x6.txt
│   │   │   │   │   ├── menu.80x24.txt
│   │   │   │   │   ├── settings.20x6.txt
│   │   │   │   │   ├── settings.80x24.txt
│   │   │   │   │   ├── snake.20x6.txt
│   │   │   │   │   ├── snake.80x24.txt
│   │   │   │   │   ├── twenty48.20x6.txt
│   │   │   │   │   └── twenty48.80x24.txt
│   │   │   │   ├── screenplays
│   │   │   │   │   ├── chess.jsonl
│   │   │   │   │   ├── mandelbrot.jsonl
│   │   │   │   │   ├── menu.jsonl
│   │   │   │   │   ├── settings.jsonl
│   │   │   │   │   ├── snake.jsonl
│   │   │   │   │   └── twenty48.jsonl
│   │   │   │   ├── shots
│   │   │   │   └── cli.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlycore
│   │   │   ├── src
│   │   │   │   ├── codec
│   │   │   │   │   ├── base64.rs
│   │   │   │   │   ├── deflate.rs
│   │   │   │   │   ├── gif.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── png.rs
│   │   │   │   │   └── wav.rs
│   │   │   │   ├── atoms.rs
│   │   │   │   ├── audio.rs
│   │   │   │   ├── cell.rs
│   │   │   │   ├── chacha.rs
│   │   │   │   ├── colors.rs
│   │   │   │   ├── data.rs
│   │   │   │   ├── enums.rs
│   │   │   │   ├── errors.rs
│   │   │   │   ├── image.rs
│   │   │   │   ├── io.rs
│   │   │   │   ├── json.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── logs.rs
│   │   │   │   ├── music.rs
│   │   │   │   ├── paint.rs
│   │   │   │   ├── ramp.rs
│   │   │   │   ├── resample.rs
│   │   │   │   ├── rng.rs
│   │   │   │   ├── state.rs
│   │   │   │   ├── tensor.rs
│   │   │   │   ├── tile.rs
│   │   │   │   ├── time.rs
│   │   │   │   └── trig.rs
│   │   │   ├── tests
│   │   │   │   └── json.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlydata
│   │   │   ├── src
│   │   │   │   ├── emit.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── main.rs
│   │   │   │   ├── press.rs
│   │   │   │   ├── trails.rs
│   │   │   │   └── wells.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlyfont
│   │   │   ├── assets
│   │   │   │   ├── MrlyFont.json
│   │   │   │   ├── MrlyFont.ttf
│   │   │   │   ├── MrlyFont.woff
│   │   │   │   └── MrlyFont.woff2
│   │   │   ├── examples
│   │   │   │   ├── cycle.rs
│   │   │   │   └── strip.rs
│   │   │   ├── src
│   │   │   │   ├── animate.rs
│   │   │   │   ├── assets.rs
│   │   │   │   ├── data.rs
│   │   │   │   ├── glyphs.rs
│   │   │   │   ├── letters.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── models.rs
│   │   │   │   ├── names.rs
│   │   │   │   ├── paths.rs
│   │   │   │   ├── raster.rs
│   │   │   │   ├── serializer.rs
│   │   │   │   └── shape.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlygame
│   │   │   ├── src
│   │   │   │   ├── config.rs
│   │   │   │   ├── emit.rs
│   │   │   │   ├── frames.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── main.rs
│   │   │   │   ├── music.rs
│   │   │   │   ├── quest.rs
│   │   │   │   ├── sequence.rs
│   │   │   │   └── variations.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlymath
│   │   │   ├── examples
│   │   │   │   └── paints.rs
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
│   │   │   │   │   ├── presets.rs
│   │   │   │   │   ├── render.rs
│   │   │   │   │   ├── sample.rs
│   │   │   │   │   └── stack.rs
│   │   │   │   ├── name
│   │   │   │   │   ├── bang.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── rule.rs
│   │   │   │   │   ├── text.rs
│   │   │   │   │   └── tile.rs
│   │   │   │   ├── pick
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── saga
│   │   │   │   │   ├── factory.rs
│   │   │   │   │   ├── metrics.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── ops.rs
│   │   │   │   │   └── solve.rs
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
│   │   │   │   │   ├── reach.rs
│   │   │   │   │   ├── renderer.rs
│   │   │   │   │   ├── serializer.rs
│   │   │   │   │   ├── sheets.rs
│   │   │   │   │   └── tile.rs
│   │   │   │   ├── two
│   │   │   │   │   ├── artwork.rs
│   │   │   │   │   ├── carry.rs
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
│   │   │   │   ├── wave
│   │   │   │   │   ├── gaps.rs
│   │   │   │   │   ├── medium.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── spectrum.rs
│   │   │   │   │   └── stepper.rs
│   │   │   │   ├── boolean.rs
│   │   │   │   ├── census.rs
│   │   │   │   ├── data.rs
│   │   │   │   ├── fft.rs
│   │   │   │   ├── lattice.rs
│   │   │   │   ├── lib.rs
│   │   │   │   └── rules.rs
│   │   │   ├── tests
│   │   │   │   └── atoms.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   ├── NAMES.md
│   │   │   └── README.md
│   │   ├── mrlymoji
│   │   │   ├── src
│   │   │   │   └── lib.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlyos
│   │   │   ├── src
│   │   │   │   ├── kernel
│   │   │   │   │   ├── os
│   │   │   │   │   │   ├── persist.rs
│   │   │   │   │   │   └── shot.rs
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
│   │   │   ├── LICENSE
│   │   │   ├── README.md
│   │   │   └── build.rs
│   │   ├── mrlyrun
│   │   │   ├── src
│   │   │   │   └── lib.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlyskin
│   │   │   ├── src
│   │   │   │   ├── draw.rs
│   │   │   │   ├── dress.rs
│   │   │   │   ├── lib.rs
│   │   │   │   └── paint.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlysolo
│   │   │   ├── src
│   │   │   │   └── lib.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   ├── mrlytorch
│   │   │   ├── src
│   │   │   │   ├── graph.rs
│   │   │   │   ├── grid.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── math.rs
│   │   │   │   ├── nn.rs
│   │   │   │   ├── ops.rs
│   │   │   │   ├── optim.rs
│   │   │   │   ├── rng.rs
│   │   │   │   └── tensor.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── GPU.md
│   │   │   ├── LICENSE
│   │   │   └── README.md
│   │   └── mrlyweb
│   │       ├── examples
│   │       │   ├── bake.rs
│   │       │   └── fixtures.rs
│   │       ├── src
│   │       │   ├── eye
│   │       │   │   ├── flat.rs
│   │       │   │   ├── fragment.rs
│   │       │   │   ├── mesh.rs
│   │       │   │   └── mod.rs
│   │       │   ├── card.rs
│   │       │   ├── goose.rs
│   │       │   ├── lib.rs
│   │       │   ├── registry.rs
│   │       │   ├── shaders.rs
│   │       │   └── vertex.wgsl
│   │       ├── tests
│   │       │   ├── card.rs
│   │       │   ├── golden.rs
│   │       │   ├── goose.rs
│   │       │   ├── kernel.rs
│   │       │   └── keys.rs
│   │       ├── Cargo.toml
│   │       ├── LICENSE
│   │       └── README.md
│   └── README.md
├── sites
│   ├── bot
│   │   ├── public
│   │   │   ├── icons
│   │   │   │   ├── mrly_192_192.png
│   │   │   │   └── mrly_512_512.png
│   │   │   ├── boot.js
│   │   │   ├── favicon.ico
│   │   │   ├── manifest.json
│   │   │   ├── mark.svg
│   │   │   ├── notes.json
│   │   │   └── robots.txt
│   │   ├── src
│   │   │   ├── App.tsx
│   │   │   └── main.tsx
│   │   ├── index.html
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── vite.config.ts
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
│   │   │   │   ├── Finder.tsx
│   │   │   │   ├── Listing.tsx
│   │   │   │   ├── More.tsx
│   │   │   │   ├── Panel.tsx
│   │   │   │   ├── Shell.tsx
│   │   │   │   └── Status.tsx
│   │   │   ├── lib
│   │   │   │   ├── code.ts
│   │   │   │   ├── data.ts
│   │   │   │   ├── find.ts
│   │   │   │   ├── langs.ts
│   │   │   │   ├── md.ts
│   │   │   │   ├── repo.ts
│   │   │   │   ├── site.ts
│   │   │   │   ├── text.ts
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
│   ├── net
│   │   ├── blog
│   │   │   ├── README.md
│   │   │   ├── millennium.md
│   │   │   ├── riemann-zeta-1.md
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
│   │   │   │   ├── Landing.tsx
│   │   │   │   ├── Menu.tsx
│   │   │   │   ├── Panel.tsx
│   │   │   │   └── Shell.tsx
│   │   │   ├── lib
│   │   │   │   ├── data.ts
│   │   │   │   ├── md.ts
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
│   │   │   ├── boot.js
│   │   │   ├── favicon.ico
│   │   │   ├── manifest.json
│   │   │   ├── mrlyprod.png
│   │   │   ├── mrlyprod.svg
│   │   │   └── robots.txt
│   │   ├── src
│   │   │   ├── components
│   │   │   │   ├── DPad.tsx
│   │   │   │   ├── Fractal.tsx
│   │   │   │   ├── GameOver.tsx
│   │   │   │   ├── Library.tsx
│   │   │   │   ├── Palette.tsx
│   │   │   │   ├── Shot.tsx
│   │   │   │   ├── Transport.tsx
│   │   │   │   └── options.ts
│   │   │   ├── eyes
│   │   │   │   ├── Bits.tsx
│   │   │   │   ├── Carve.tsx
│   │   │   │   ├── Cells.tsx
│   │   │   │   ├── Face.tsx
│   │   │   │   ├── Shader.tsx
│   │   │   │   ├── orbit.ts
│   │   │   │   ├── skin.ts
│   │   │   │   ├── theme.ts
│   │   │   │   └── wallpaper.ts
│   │   │   ├── gen
│   │   │   │   ├── palette.json
│   │   │   │   ├── rigs.json
│   │   │   │   ├── shaders.json
│   │   │   │   └── skins.json
│   │   │   ├── views
│   │   │   │   ├── creativity
│   │   │   │   │   ├── notes.tsx
│   │   │   │   │   ├── photos.tsx
│   │   │   │   │   └── studio.tsx
│   │   │   │   ├── design
│   │   │   │   │   ├── colors.tsx
│   │   │   │   │   ├── emojis.tsx
│   │   │   │   │   ├── fonts.tsx
│   │   │   │   │   └── paint.tsx
│   │   │   │   ├── games
│   │   │   │   │   ├── crush.tsx
│   │   │   │   │   ├── escape.tsx
│   │   │   │   │   ├── snake.tsx
│   │   │   │   │   └── tennis.tsx
│   │   │   │   ├── math
│   │   │   │   │   ├── bang.tsx
│   │   │   │   │   ├── graph.tsx
│   │   │   │   │   ├── life.tsx
│   │   │   │   │   ├── moire.tsx
│   │   │   │   │   ├── six.tsx
│   │   │   │   │   ├── three.tsx
│   │   │   │   │   ├── tile.tsx
│   │   │   │   │   └── two.tsx
│   │   │   │   ├── puzzles
│   │   │   │   │   ├── arc.tsx
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
│   │   │   │   │   └── settings.tsx
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
│   │   │   │   └── index.tsx
│   │   │   ├── App.tsx
│   │   │   ├── ask.tsx
│   │   │   ├── boot.ts
│   │   │   ├── builders.ts
│   │   │   ├── effects.ts
│   │   │   ├── eyes.css
│   │   │   ├── journal.ts
│   │   │   ├── kernel.ts
│   │   │   ├── main.tsx
│   │   │   ├── palette.ts
│   │   │   ├── pwa.ts
│   │   │   ├── reads.ts
│   │   │   ├── roller.ts
│   │   │   ├── router.ts
│   │   │   ├── send.ts
│   │   │   ├── splash.ts
│   │   │   ├── time.ts
│   │   │   └── types.ts
│   │   ├── index.html
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── verify.ts
│   │   └── vite.config.ts
│   └── README.md
├── utils
│   ├── brand.py
│   ├── config.py
│   ├── doors.py
│   ├── font.py
│   ├── layers.py
│   ├── license.py
│   ├── logos.py
│   ├── paints.py
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
├── LICENSE.md
├── README.md
├── STATS.md
├── TREE.md
├── bun.lock
├── clone.sh
├── package.json
├── pyproject.toml
├── rust-toolchain.toml
├── tsconfig.base.json
└── uv.lock
```
