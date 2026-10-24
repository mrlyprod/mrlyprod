import { useEffect, useRef, useState, useSyncExternalStore } from "react";
import { Btn, Group, Icon, Pick, Segment, Slider, Toggle } from "../../lib/app/controls.jsx";
import { HUES, paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { fed, source } from "../../lib/glass/feed.js";
import { tints } from "../../lib/theme/hues.js";
import { palette } from "../../lib/theme/palette.gen.js";
import { glass, icons, KEY, read, subscribe, theme, tint } from "../../shell/prefs.js";
import { Tile } from "../../shell/tile.jsx";

const usePref = (key) =>
  useSyncExternalStore(
    subscribe,
    () => read(key),
    () => "",
  );

/* THEME */

const SHADES = [
  ["light", "Light"],
  ["dark", "Dark"],
  ["", "Auto"],
];

const TINTS = tints("Auto");

export function Theme() {
  const shade = usePref(KEY.theme);
  const hue = usePref(KEY.tint);
  return (
    <div className="lanes">
      <div className="controls">
        <Group name="Appearance">
          <Segment label="Theme" value={shade} options={SHADES} onChange={theme} />
          <Pick label="Tint" value={hue} options={TINTS} onChange={tint} />
        </Group>
      </div>
      <div className="hues" role="group" aria-label="Tint">
        <Btn on={hue === ""} onClick={() => tint("")}>
          Auto
        </Btn>
        {HUES.map((one) => (
          <Btn
            key={one}
            className="hue"
            style={{ "--hue": `var(--${one})` }}
            aria-pressed={hue === one}
            aria-label={one}
            title={one}
            onClick={() => tint(one)}
          />
        ))}
      </div>
    </div>
  );
}

/* BANDS */

const WIDTH = 960;
const HEIGHT = 360;
const STRIPES = [
  [0.14, 0.1, "paper"],
  [0.5, 0.06, "ink"],
  [0.8, 0.1, "paper"],
];
const WAKE = 120;

export function bands(ctx, w, h, tones) {
  ctx.fillStyle = tones.paper;
  ctx.fillRect(0, 0, w, h);
  HUES.forEach((hue, i) => {
    const left = Math.round((i * w) / HUES.length);
    ctx.fillStyle = palette[hue];
    ctx.fillRect(left, 0, Math.round(((i + 1) * w) / HUES.length) - left, h);
  });
  for (const [top, tall, tone] of STRIPES) {
    ctx.fillStyle = tones[tone];
    ctx.fillRect(0, Math.round(top * h), w, Math.round(tall * h));
  }
}

function visible(canvas) {
  const box = canvas.getBoundingClientRect();
  const view = canvas.closest("main")?.getBoundingClientRect();
  if (!view) return box;
  const left = Math.max(box.left, view.left);
  const top = Math.max(box.top, view.top);
  return new DOMRect(
    left,
    top,
    Math.max(0, Math.min(box.right, view.right) - left),
    Math.max(0, Math.min(box.bottom, view.bottom) - top),
  );
}

function Backdrop({ stage }) {
  const tick = useTheme();
  useEffect(() => {
    const canvas = stage.current;
    bands(canvas.getContext("2d"), canvas.width, canvas.height, paints(canvas));
    fed(canvas);
  }, [tick]);
  useEffect(() => source(stage.current, () => visible(stage.current)), []);
  useEffect(() => {
    const main = stage.current.closest("main");
    const wake = () => dispatchEvent(new CustomEvent("shift", { detail: WAKE }));
    main?.addEventListener("scroll", wake, { passive: true });
    return () => main?.removeEventListener("scroll", wake);
  }, []);
  return <canvas ref={stage} width={WIDTH} height={HEIGHT} aria-hidden="true" />;
}

/* GLASS */

const STYLES = [
  ["clear", "Clear"],
  ["", "Tinted"],
  ["frosted", "Frosted"],
  ["off", "Off"],
];

const ROUND = [
  ["info", "Info"],
  ["ios_share", "Share"],
  ["keyboard", "Keys"],
  ["download", "Save"],
];

const MENU = ["Copy link", "Duplicate", "Move to", "Delete"];

export function Glass() {
  const style = usePref(KEY.glass);
  const stage = useRef(null);
  const [depth, setDepth] = useState(40);
  const [rim, setRim] = useState(1);
  return (
    <>
      <div className="controls">
        <Group name="Glass">
          <Segment label="Style" value={style} options={STYLES} onChange={glass} />
        </Group>
      </div>
      <div className="backdrop">
        <Backdrop stage={stage} />
        <figure className="piece">
          <div data-lens="pill">
            {ROUND.map(([name, label]) => (
              <Btn key={name} aria-label={label} title={label}>
                <Icon name={name} />
              </Btn>
            ))}
          </div>
          <figcaption>pill</figcaption>
        </figure>
        <figure className="piece">
          <div data-lens="panel">
            <div className="controls">
              <Group name="Knobs">
                <Slider label="Depth" value={depth} min={0} max={100} onChange={setDepth} />
                <Toggle label="Rim" value={rim} onChange={setRim} />
              </Group>
            </div>
          </div>
          <figcaption>panel</figcaption>
        </figure>
        <figure className="piece">
          <div className="pop menu" data-lens="menu" role="group" aria-label="Menu">
            {MENU.map((text) => (
              <button type="button" key={text}>
                {text}
              </button>
            ))}
          </div>
          <figcaption>menu</figcaption>
        </figure>
        <figure className="piece">
          <div className="frost">A box of CSS glass</div>
          <figcaption>frost</figcaption>
        </figure>
      </div>
    </>
  );
}

/* ICONS */

const LOOKS = [
  ["", "Default"],
  ["dark", "Dark"],
  ["clear-light", "Clear light"],
  ["clear-dark", "Clear dark"],
  ["tinted-light", "Tinted light"],
  ["tinted-dark", "Tinted dark"],
];

const KINDS = [
  ["", "Default"],
  ["dark", "Dark"],
  ["clear", "Clear"],
  ["tinted", "Tinted"],
];

const APPS = ["about", "blog", "moire", "spin", "three", "polygons"];

export function Icons() {
  const style = usePref(KEY.icons);
  const always = usePref(KEY.always) === "1";
  return (
    <>
      <div className="controls">
        <Group name="Icons">
          <Segment label="Style" value={style} options={KINDS} onChange={(next) => icons(next, always)} />
          <Toggle label="Always dark" value={always ? 1 : 0} onChange={(on) => icons(style, on === 1)} />
        </Group>
      </div>
      <div className="looks">
        {LOOKS.map(([look, text]) => (
          <figure key={look} data-icons={look}>
            <div className="tiles">
              {APPS.map((id) => (
                <Tile key={id} id={id} name={false} />
              ))}
            </div>
            <figcaption>{text}</figcaption>
          </figure>
        ))}
      </div>
    </>
  );
}
