import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix, xorshift } from "@mrlyprod/ui/kit/rand.ts";

export type Kind = "K" | "Q" | "R" | "B" | "N" | "P";

export type Side = "white" | "black";

export type Piece = { kind: Kind; side: Side; moved: boolean };

export type Board = (Piece | null)[];

export type Move = { from: number; to: number };

export type Act = { verb: "move"; args: Move };

export type Over = "checkmate" | "stalemate" | "draw" | null;

export type State = {
  board: Board;
  turn: Side;
  human: Side;
  passant: number;
  last: Move | null;
  check: boolean;
  over: Over;
  winner: Side | null;
  quiet: number;
  ply: number;
  seed: number;
};

export const SIZE = 8;

const WIDE = 4294967296;
const DEPTH = 3;
const MATE = 99999;
const QUIET = 100;
const PAIR = 30;
const ROAM = 2;
const PAWN = 100;
const LEAD = 10;
const CROWN = 900;
const NONE = -1;

const RANK: Kind[] = ["R", "N", "B", "Q", "K", "B", "N", "R"];

const WORTH: Record<Kind, number> = { P: 1, N: 3, B: 3, R: 5, Q: 9, K: 100 };

const ORTHO = [
  [0, -1],
  [0, 1],
  [-1, 0],
  [1, 0],
];

const DIAG = [
  [-1, -1],
  [1, -1],
  [-1, 1],
  [1, 1],
];

const ROUND = [...ORTHO, ...DIAG];

const JUMPS = [
  [1, -2],
  [2, -1],
  [2, 1],
  [1, 2],
  [-1, 2],
  [-2, 1],
  [-2, -1],
  [-1, -2],
];

const TABLE: Record<Kind, readonly number[]> = {
  P: [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5, 10, 25, 25, 10, 5, 5,
    0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
  ],
  N: [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10, 0, -30, -30, 5, 15,
    20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
  ],
  B: [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0, -10, -10, 5, 5, 10, 10,
    5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10,
    -10, -10, -10, -10, -10, -20,
  ],
  R: [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
    0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 5, 5, 0, 0, 0,
  ],
  Q: [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10, -5, 0, 5, 5, 5, 5, 0,
    -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10,
    -10, -20,
  ],
  K: [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40,
    -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30, -30, -20, -10, -20, -20, -20, -20, -20,
    -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20,
  ],
};

const on = (x: number, y: number) => x >= 0 && x < SIZE && y >= 0 && y < SIZE;

const seat = (x: number, y: number) => y * SIZE + x;

const foe = (side: Side): Side => (side === "white" ? "black" : "white");

function opening(): Board {
  const board: Board = Array(SIZE * SIZE).fill(null);
  for (const [x, kind] of RANK.entries()) {
    board[seat(x, 0)] = { kind, side: "black", moved: false };
    board[seat(x, 1)] = { kind: "P", side: "black", moved: false };
    board[seat(x, 6)] = { kind: "P", side: "white", moved: false };
    board[seat(x, 7)] = { kind, side: "white", moved: false };
  }
  return board;
}

function reach(board: Board, at: number, passant: number): number[] {
  const piece = board[at]!;
  const px = at % SIZE;
  const py = Math.floor(at / SIZE);
  const out: number[] = [];
  const slide = ([dx, dy]: number[]) => {
    for (let x = px + dx, y = py + dy; on(x, y); x += dx, y += dy) {
      const there = board[seat(x, y)];
      if (!there || there.side !== piece.side) out.push(seat(x, y));
      if (there) return;
    }
  };
  const hop = ([dx, dy]: number[]) => {
    const x = px + dx;
    const y = py + dy;
    if (on(x, y) && board[seat(x, y)]?.side !== piece.side) out.push(seat(x, y));
  };
  if (piece.kind === "P") {
    const dir = piece.side === "white" ? -1 : 1;
    if (on(px, py + dir) && !board[seat(px, py + dir)]) {
      out.push(seat(px, py + dir));
      if (!piece.moved && on(px, py + 2 * dir) && !board[seat(px, py + 2 * dir)]) out.push(seat(px, py + 2 * dir));
    }
    for (const dx of [-1, 1]) {
      if (!on(px + dx, py + dir)) continue;
      const to = seat(px + dx, py + dir);
      const there = board[to];
      if ((there && there.side !== piece.side) || to === passant) out.push(to);
    }
  }
  if (piece.kind === "N") JUMPS.forEach(hop);
  if (piece.kind === "B" || piece.kind === "Q") DIAG.forEach(slide);
  if (piece.kind === "R" || piece.kind === "Q") ORTHO.forEach(slide);
  if (piece.kind === "K") ROUND.forEach(hop);
  return out;
}

function attacked(board: Board, at: number, by: Side): boolean {
  const px = at % SIZE;
  const py = Math.floor(at / SIZE);
  const holds = (x: number, y: number, kind: Kind) => {
    if (!on(x, y)) return false;
    const there = board[seat(x, y)];
    return there?.side === by && there.kind === kind;
  };
  const back = by === "white" ? 1 : -1;
  if (holds(px - 1, py + back, "P") || holds(px + 1, py + back, "P")) return true;
  if (JUMPS.some(([dx, dy]) => holds(px + dx, py + dy, "N"))) return true;
  if (ROUND.some(([dx, dy]) => holds(px + dx, py + dy, "K"))) return true;
  const ray = (dirs: number[][], kind: Kind) =>
    dirs.some(([dx, dy]) => {
      for (let x = px + dx, y = py + dy; on(x, y); x += dx, y += dy) {
        const there = board[seat(x, y)];
        if (there) return there.side === by && (there.kind === kind || there.kind === "Q");
      }
      return false;
    });
  return ray(ORTHO, "R") || ray(DIAG, "B");
}

const kingOf = (board: Board, side: Side) => board.findIndex((one) => one?.kind === "K" && one.side === side);

const inCheck = (board: Board, side: Side) => attacked(board, kingOf(board, side), foe(side));

function apply(board: Board, { from, to }: Move, passant: number): Board {
  const next = board.slice();
  const piece = next[from]!;
  const row = from - (from % SIZE);
  if (piece.kind === "P" && to === passant) next[row + (to % SIZE)] = null;
  if (piece.kind === "K" && Math.abs((to % SIZE) - (from % SIZE)) === 2) {
    const [rook, into] = to % SIZE === 6 ? [row + 7, row + 5] : [row, row + 3];
    next[into] = { ...next[rook]!, moved: true };
    next[rook] = null;
  }
  const crowned = piece.kind === "P" && (to < SIZE || to >= SIZE * (SIZE - 1));
  next[to] = { ...piece, moved: true, kind: crowned ? "Q" : piece.kind };
  next[from] = null;
  return next;
}

const doubled = (piece: Piece, { from, to }: Move) =>
  piece.kind === "P" && Math.abs(to - from) === 2 * SIZE ? (from + to) / 2 : NONE;

function legal(board: Board, at: number, passant: number): number[] {
  const piece = board[at]!;
  const { side } = piece;
  const enemy = foe(side);
  const king = kingOf(board, side);
  const found = reach(board, at, passant).filter(
    (to) => !attacked(apply(board, { from: at, to }, passant), piece.kind === "K" ? to : king, enemy),
  );
  if (piece.kind !== "K" || piece.moved || attacked(board, at, enemy)) return found;
  const row = at - (at % SIZE);
  const rook = (col: number) => {
    const there = board[row + col];
    return there?.kind === "R" && !there.moved && there.side === side;
  };
  const clear = (cols: number[]) => cols.every((col) => !board[row + col]);
  const calm = (cols: number[]) => cols.every((col) => !attacked(board, row + col, enemy));
  if (rook(7) && clear([5, 6]) && calm([5, 6])) found.push(row + 6);
  if (rook(0) && clear([1, 2, 3]) && calm([2, 3])) found.push(row + 2);
  return found;
}

function anyMove(board: Board, side: Side, passant: number): boolean {
  for (let at = 0; at < board.length; at++)
    if (board[at]?.side === side && legal(board, at, passant).length) return true;
  return false;
}

function ranked(board: Board, side: Side, passant: number): Move[] {
  const out: { move: Move; order: number }[] = [];
  for (let from = 0; from < board.length; from++) {
    const piece = board[from];
    if (piece?.side !== side) continue;
    for (const to of legal(board, from, passant)) {
      const victim = board[to];
      const taken = victim ? victim.kind : piece.kind === "P" && to === passant ? "P" : null;
      const gain = taken ? WORTH[taken] * PAWN - WORTH[piece.kind] * LEAD : 0;
      const crowned = piece.kind === "P" && (to < SIZE || to >= SIZE * (SIZE - 1));
      out.push({ move: { from, to }, order: gain + (crowned ? CROWN : 0) });
    }
  }
  return out.sort((a, b) => b.order - a.order).map((one) => one.move);
}

function worth(board: Board, side: Side): number {
  const score = { white: 0, black: 0 };
  const bishops = { white: 0, black: 0 };
  for (let at = 0; at < board.length; at++) {
    const piece = board[at];
    if (!piece) continue;
    const spot = piece.side === "white" ? at : (SIZE - 1 - Math.floor(at / SIZE)) * SIZE + (at % SIZE);
    score[piece.side] += WORTH[piece.kind] * PAWN + TABLE[piece.kind][spot] + reach(board, at, NONE).length * ROAM;
    if (piece.kind === "B") bishops[piece.side]++;
  }
  if (bishops.white >= 2) score.white += PAIR;
  if (bishops.black >= 2) score.black += PAIR;
  return score[side] - score[foe(side)];
}

function search(
  board: Board,
  passant: number,
  depth: number,
  floor: number,
  ceil: number,
  mine: boolean,
  side: Side,
): number {
  const turn = mine ? side : foe(side);
  const ended = () => {
    if (!inCheck(board, turn)) return 0;
    const left = DEPTH - depth;
    return mine ? left - MATE : MATE - left;
  };
  if (depth === 0) return anyMove(board, turn, passant) ? worth(board, side) : ended();
  const list = ranked(board, turn, passant);
  if (!list.length) return ended();
  let best = mine ? -Infinity : Infinity;
  let low = floor;
  let high = ceil;
  for (const move of list) {
    const after = apply(board, move, passant);
    const score = search(after, doubled(board[move.from]!, move), depth - 1, low, high, !mine, side);
    best = mine ? Math.max(best, score) : Math.min(best, score);
    if (mine) low = Math.max(low, score);
    else high = Math.min(high, score);
    if (low >= high) break;
  }
  return best;
}

export function boot({ seed }: { seed: number }): State {
  const next = xorshift(mix(seed));
  return {
    board: opening(),
    turn: "white",
    human: next & 1 ? "white" : "black",
    passant: NONE,
    last: null,
    check: false,
    over: null,
    winner: null,
    quiet: 0,
    ply: 0,
    seed: next,
  };
}

export function targets(state: State, at: number): number[] {
  return !state.over && state.board[at]?.side === state.turn ? legal(state.board, at, state.passant) : [];
}

export function moves(state: State): Act[] {
  return state.board.flatMap((_, from) =>
    targets(state, from).map((to): Act => ({ verb: "move", args: { from, to } })),
  );
}

const bare = (board: Board) => {
  const rest = board.filter((one) => one && one.kind !== "K");
  return !rest.length || (rest.length === 1 && (rest[0]!.kind === "N" || rest[0]!.kind === "B"));
};

export function step(state: State, act: Act): State {
  if (act.verb !== "move") return state;
  const { from, to } = act.args;
  if (!targets(state, from).includes(to)) return state;
  const piece = state.board[from]!;
  const board = apply(state.board, act.args, state.passant);
  const turn = foe(state.turn);
  const passant = doubled(piece, act.args);
  const taken = state.board[to] !== null || (piece.kind === "P" && to === state.passant);
  const quiet = piece.kind === "P" || taken ? 0 : state.quiet + 1;
  const check = inCheck(board, turn);
  const stuck = !anyMove(board, turn, passant);
  const over: Over = stuck ? (check ? "checkmate" : "stalemate") : quiet >= QUIET || bare(board) ? "draw" : null;
  return {
    ...state,
    board,
    turn,
    passant,
    last: act.args,
    check,
    over,
    winner: over === "checkmate" ? state.turn : null,
    quiet,
    ply: state.ply + 1,
  };
}

export function reply(state: State): Act | null {
  if (state.over) return null;
  let top = -Infinity;
  let best: Move[] = [];
  for (const move of ranked(state.board, state.turn, state.passant)) {
    const after = apply(state.board, move, state.passant);
    const passant = doubled(state.board[move.from]!, move);
    const score = search(after, passant, DEPTH - 1, top - 1, Infinity, false, state.turn);
    if (score > top) {
      top = score;
      best = [move];
    } else if (score === top) best.push(move);
  }
  if (!best.length) return null;
  const pick = xorshift(mix((state.seed ^ state.ply) >>> 0)) % best.length;
  return { verb: "move", args: best[pick] };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
