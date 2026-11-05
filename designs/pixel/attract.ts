import { ink } from "./kit.ts";
import { SPRITES } from "./sprite.tsx";

type Dir = 0 | 1 | 2 | 3;

type Actor = { x: number; y: number; dir: Dir; speed: number; ink: string; ghost: boolean; p: number };

type Maze = {
  cols: number;
  rows: number;
  wall: Uint8Array;
  food: Uint8Array;
  left: number;
  walls: HTMLCanvasElement;
};

const DX = [1, 0, -1, 0];

const DY = [0, 1, 0, -1];

const TILE = 24;

export function rng(seed: number) {
  let s = seed >>> 0 || 1;
  return () => {
    s ^= s << 13;
    s ^= s >>> 17;
    s ^= s << 5;
    return (s >>> 0) / 4294967296;
  };
}

export function carve(cols: number, rows: number, rand: () => number) {
  const wall = new Uint8Array(cols * rows).fill(1);
  const at = (x: number, y: number) => y * cols + x;
  const stack: [number, number][] = [[1, 1]];
  wall[at(1, 1)] = 0;
  while (stack.length) {
    const [x, y] = stack.at(-1)!;
    const next = ([0, 1, 2, 3] as Dir[])
      .map((d) => [x + DX[d]! * 2, y + DY[d]! * 2, d] as const)
      .filter(([nx, ny]) => nx > 0 && ny > 0 && nx < cols - 1 && ny < rows - 1 && wall[at(nx, ny)]);
    if (!next.length) {
      stack.pop();
      continue;
    }
    const [nx, ny, d] = next[Math.floor(rand() * next.length)]!;
    wall[at(x + DX[d]!, y + DY[d]!)] = 0;
    wall[at(nx, ny)] = 0;
    stack.push([nx, ny]);
  }
  for (let y = 1; y < rows - 1; y += 2)
    for (let x = 1; x < cols - 1; x += 2) {
      const open = ([0, 1, 2, 3] as Dir[]).filter((d) => !wall[at(x + DX[d]!, y + DY[d]!)]);
      if (open.length > 1) continue;
      const shut = ([0, 1, 2, 3] as Dir[]).filter((d) => {
        const wx = x + DX[d]!;
        const wy = y + DY[d]!;
        return wall[at(wx, wy)] && wx > 0 && wy > 0 && wx < cols - 1 && wy < rows - 1;
      });
      const d = shut[Math.floor(rand() * shut.length)];
      if (d !== undefined) wall[at(x + DX[d]!, y + DY[d]!)] = 0;
    }
  return wall;
}

function maze(width: number, height: number, rand: () => number, ink: string, ground: string): Maze {
  const odd = (n: number) => Math.max(7, n % 2 ? n : n - 1);
  const cols = odd(Math.floor(width / TILE));
  const rows = odd(Math.floor(height / TILE));
  const wall = carve(cols, rows, rand);
  const food = new Uint8Array(cols * rows);
  let left = 0;
  for (let i = 0; i < wall.length; i++)
    if (!wall[i]) {
      food[i] = 1;
      left++;
    }
  for (const [x, y] of [
    [1, 1],
    [cols - 2, 1],
    [1, rows - 2],
    [cols - 2, rows - 2],
  ])
    food[y! * cols + x!] = 2;
  const walls = document.createElement("canvas");
  walls.width = cols * TILE;
  walls.height = rows * TILE;
  const g = walls.getContext("2d")!;
  g.fillStyle = ground;
  g.fillRect(0, 0, walls.width, walls.height);
  const solid = (x: number, y: number) => x >= 0 && y >= 0 && x < cols && y < rows && wall[y * cols + x] === 1;
  const tube = (half: number) => {
    const lo = Math.round(TILE * (0.5 - half));
    const w = Math.round(TILE * half * 2);
    for (let y = 0; y < rows; y++)
      for (let x = 0; x < cols; x++) {
        if (!solid(x, y)) continue;
        g.fillRect(x * TILE + lo, y * TILE + lo, w, w);
        if (solid(x + 1, y)) g.fillRect(x * TILE + TILE / 2, y * TILE + lo, TILE, w);
        if (solid(x, y + 1)) g.fillRect(x * TILE + lo, y * TILE + TILE / 2, w, TILE);
      }
  };
  g.fillStyle = ink;
  tube(0.25);
  g.fillStyle = ground;
  tube(0.125);
  return { cols, rows, wall, food, left, walls };
}

function stamp(g: CanvasRenderingContext2D, art: string, x: number, y: number, size: number, ink: string, dir = 0) {
  const rows = art.split(" ");
  const unit = size / rows.length;
  g.save();
  g.translate(x, y);
  g.rotate((dir * Math.PI) / 2);
  for (let r = 0; r < rows.length; r++)
    for (let c = 0; c < rows[r]!.length; c++) {
      const ch = rows[r]![c];
      if (ch === ".") continue;
      g.fillStyle = ch === "w" ? "#ffffff" : ch === "b" ? "#2121de" : ink;
      g.fillRect(Math.round(-size / 2 + c * unit), Math.round(-size / 2 + r * unit), Math.ceil(unit), Math.ceil(unit));
    }
  g.restore();
}

export function inks() {
  return {
    a: ink("--px-a", "#ffd400"),
    b: ink("--px-b", "#ff2a2a"),
    c: ink("--px-c", "#3ff3ff"),
    d: ink("--px-d", "#ff6ad5"),
    wall: ink("--px-wall", "#2121de"),
    ground: "#000000",
    fg: ink("--fg", "#ffffff"),
  };
}

function chase(canvas: HTMLCanvasElement, still: boolean) {
  const g = canvas.getContext("2d")!;
  const rand = rng(Date.now());
  const ink = inks();
  const scale = Math.min(2, devicePixelRatio || 1);
  canvas.width = Math.floor(canvas.clientWidth * scale);
  canvas.height = Math.floor(canvas.clientHeight * scale);
  let world = maze(canvas.clientWidth, canvas.clientHeight, rand, ink.wall, ink.ground);
  let fright = 0;
  let dead = 0;
  const open = (x: number, y: number) =>
    x >= 0 && y >= 0 && x < world.cols && y < world.rows && !world.wall[y * world.cols + x];
  const spawn = (x: number, y: number, speed: number, inkName: string, ghost: boolean): Actor => ({
    x,
    y,
    dir: 0,
    speed,
    ink: inkName,
    ghost,
    p: 0,
  });
  const cast = () => {
    const mx = (world.cols >> 2) * 2 + 1;
    const my = (world.rows >> 2) * 2 + 1;
    return [
      spawn(mx, world.rows - 2, 6, ink.a, false),
      spawn(world.cols - 2, world.rows - 2, 5, ink.b, true),
      spawn(world.cols - 2, 1, 4.6, ink.d, true),
      spawn(1, world.rows - 2, 4.4, ink.c, true),
      spawn(mx, my, 4.2, "#ffb852", true),
    ];
  };
  let actors = cast();
  const pac = () => actors[0]!;
  const choose = (one: Actor) => {
    const { x, y } = one;
    const back = ((one.dir + 2) % 4) as Dir;
    let ways = ([0, 1, 2, 3] as Dir[]).filter((d) => open(x + DX[d]!, y + DY[d]!));
    if (ways.length > 1) ways = ways.filter((d) => d !== back);
    if (!ways.length) return back;
    if (!one.ghost) {
      const fed = ways.filter((d) => world.food[(y + DY[d]!) * world.cols + x + DX[d]!]);
      const pool = fed.length ? fed : ways;
      return pool[Math.floor(rand() * pool.length)]!;
    }
    const p = pac();
    const far = (d: Dir) => Math.hypot(x + DX[d]! - p.x, y + DY[d]! - p.y);
    if (rand() < 0.55) {
      const sorted = [...ways].sort((m, n) => far(m) - far(n));
      return fright > 0 ? sorted.at(-1)! : sorted[0]!;
    }
    return ways[Math.floor(rand() * ways.length)]!;
  };
  const step = (dt: number) => {
    if (dead > 0) {
      dead -= dt;
      if (dead <= 0) actors = cast();
      return;
    }
    fright = Math.max(0, fright - dt);
    for (const one of actors) {
      if (one.p === 0) one.dir = choose(one);
      if (!open(one.x + DX[one.dir]!, one.y + DY[one.dir]!)) continue;
      one.p += one.speed * dt * (one.ghost && fright > 0 ? 0.6 : 1);
      while (one.p >= 1) {
        one.x += DX[one.dir]!;
        one.y += DY[one.dir]!;
        one.p -= 1;
        one.dir = choose(one);
        if (!open(one.x + DX[one.dir]!, one.y + DY[one.dir]!)) one.p = 0;
      }
    }
    const p = pac();
    const i = p.y * world.cols + p.x;
    if (world.food[i]) {
      if (world.food[i] === 2) fright = 6;
      world.food[i] = 0;
      world.left--;
    }
    for (const one of actors.slice(1)) {
      if (Math.abs(one.x - p.x) + Math.abs(one.y - p.y) > 0) continue;
      if (fright > 0) {
        one.x = (world.cols >> 2) * 2 + 1;
        one.y = (world.rows >> 2) * 2 + 1;
        one.p = 0;
      } else dead = 1.6;
    }
    if (world.left <= 0) {
      world = maze(canvas.clientWidth, canvas.clientHeight, rand, ink.wall, ink.ground);
      actors = cast();
    }
  };
  const draw = (t: number) => {
    g.setTransform(scale, 0, 0, scale, 0, 0);
    g.imageSmoothingEnabled = false;
    g.fillStyle = ink.ground;
    g.fillRect(0, 0, canvas.clientWidth, canvas.clientHeight);
    const ox = Math.floor((canvas.clientWidth - world.cols * TILE) / 2);
    const oy = Math.floor((canvas.clientHeight - world.rows * TILE) / 2);
    g.drawImage(world.walls, ox, oy);
    const blink = Math.floor(t * 4) % 2 === 0;
    for (let y = 0; y < world.rows; y++)
      for (let x = 0; x < world.cols; x++) {
        const f = world.food[y * world.cols + x];
        if (!f) continue;
        g.fillStyle = "#ffcfa8";
        const s = f === 2 ? (blink ? 12 : 0) : 4;
        if (s) g.fillRect(ox + x * TILE + (TILE - s) / 2, oy + y * TILE + (TILE - s) / 2, s, s);
      }
    actors.forEach((one, n) => {
      const px = ox + (one.x + DX[one.dir]! * one.p) * TILE + TILE / 2;
      const py = oy + (one.y + DY[one.dir]! * one.p) * TILE + TILE / 2;
      if (n === 0) {
        const art = dead > 0 ? SPRITES.shut : Math.floor(t * 10) % 2 ? SPRITES.pac : SPRITES.shut;
        const size = dead > 0 ? Math.max(0, 20 * (dead / 1.6)) : 20;
        stamp(g, art, px, py, size, one.ink, one.dir);
        return;
      }
      const scared = fright > 0 && !(fright < 2 && blink);
      stamp(g, SPRITES.ghost, px, py, 20, fright > 0 ? (scared ? "#2121de" : "#ffffff") : one.ink);
    });
  };
  if (still) {
    for (let i = 0; i < 40; i++) step(0.05);
    draw(0);
    return () => {};
  }
  let last = performance.now();
  let frame = 0;
  const tick = (now: number) => {
    const dt = Math.min(0.05, (now - last) / 1000);
    last = now;
    step(dt);
    draw(now / 1000);
    frame = requestAnimationFrame(tick);
  };
  frame = requestAnimationFrame(tick);
  return () => cancelAnimationFrame(frame);
}

function stars(canvas: HTMLCanvasElement, still: boolean) {
  const g = canvas.getContext("2d")!;
  const ink = inks();
  const colours = [ink.a, ink.b, ink.c, ink.d, ink.fg];
  const rand = rng(7);
  const N = 420;
  const sx = new Float32Array(N);
  const sy = new Float32Array(N);
  const sz = new Float32Array(N);
  const reset = (i: number, z = 1) => {
    sx[i] = rand() * 2 - 1;
    sy[i] = rand() * 2 - 1;
    sz[i] = z;
  };
  for (let i = 0; i < N; i++) reset(i, rand());
  const draw = (dt: number) => {
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    if (canvas.width !== w) {
      canvas.width = w;
      canvas.height = h;
    }
    g.fillStyle = ink.ground;
    g.fillRect(0, 0, w, h);
    for (let i = 0; i < N; i++) {
      sz[i] = sz[i]! - dt * 0.35;
      if (sz[i]! <= 0.02) reset(i);
      const k = 1 / sz[i]!;
      const x = w / 2 + sx[i]! * k * w * 0.25;
      const y = h / 2 + sy[i]! * k * h * 0.25;
      if (x < 0 || y < 0 || x > w || y > h) {
        reset(i);
        continue;
      }
      const s = Math.max(3, Math.min(9, Math.floor(((1 - sz[i]!) * 9) / 3) * 3));
      g.fillStyle = colours[i % colours.length]!;
      g.fillRect(Math.floor(x / 3) * 3, Math.floor(y / 3) * 3, s, s);
    }
  };
  if (still) {
    draw(0);
    return () => {};
  }
  let last = performance.now();
  let frame = 0;
  const tick = (now: number) => {
    draw(Math.min(0.05, (now - last) / 1000));
    last = now;
    frame = requestAnimationFrame(tick);
  };
  frame = requestAnimationFrame(tick);
  return () => cancelAnimationFrame(frame);
}

export const SAVERS = { "": chase, stars } as const;

export function play(canvas: HTMLCanvasElement, kind: string, still: boolean) {
  const run = SAVERS[kind as keyof typeof SAVERS];
  return run ? run(canvas, still) : () => {};
}
