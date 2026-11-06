type Value = unknown;
type Read = () => Value;
type Shape = (read: Read) => Value;
type Fn = (...args: never[]) => Value;
type Fns = ((...args: number[]) => number)[];
type Direct = { at: number; bool: boolean; int: boolean; fail: boolean; self: boolean; n: number };
type ClassInfo = { fields: [string, boolean][]; methods: string[]; statics: string[]; from: boolean; json: boolean };
type Api = { unit: string; doors: string[]; direct: Record<string, Direct>; classes: Record<string, ClassInfo>; consts: string[] };
type Exports = Record<string, (...args: number[]) => number> & { memory: WebAssembly.Memory };
type Thing = { $h?: number; $u?: string; free(): void; toJSON?(): Value; __state?(): Value; __restore?(state: Value): void };
type Typed = (typeof KINDS)[number];
type Source = { module_or_path?: WebAssembly.Module | BufferSource | string | URL | Request | Response } | string | URL;

const KINDS = [
  Uint8Array,
  Uint16Array,
  Uint32Array,
  Int8Array,
  Int16Array,
  Int32Array,
  Float32Array,
  Float64Array,
  BigUint64Array,
  BigInt64Array,
];
const CODES = new Map<unknown, number>(KINDS.map((kind, code) => [kind, code]));
CODES.set(Uint8ClampedArray, 0);
const RAW = new Set(["data", "types", "colors", "tags"]);
const NUL = 0x00;
const FALSE = 0x01;
const TRUE = 0x02;
const I32 = 0x03;
const F32 = 0x04;
const F64 = 0x05;
const I53 = 0x06;
const STR = 0x07;
const ARR = 0x08;
const MAP = 0x09;
const SHP = 0x0a;
const SYM = 0x0b;
const BLOB_OUT = 0x0c;
const BLOB_IN = 0x0d;
const HANDLE = 0x0e;
const BIG = 0x0f;
const VAR = 0x10;
const FORM = 0x11;
const WIDE = 0x12;
const BACK = 0x13;
const BIG64 = 0x14;
const UBIG64 = 0x15;
const INLINE = 0x16;
const SMALL = 0x80;

export function blob(name: string, built: Shape[], offsets: Record<string, number>, rng: string[]) {
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();
  const names: string[] = [];
  const index = new Map<string, number>();
  const classes = new Map<string, new (...args: never[]) => Thing>();
  const syms: string[] = [];
  const shapes: Shape[] = built.slice();
  const binders: (() => void)[] = [];
  let pending: Promise<Exports> | null = null;
  let module: WebAssembly.Module | null = null;
  let x = null as unknown as Exports;
  let memory = null as unknown as WebAssembly.Memory;
  let u8 = new Uint8Array(0);
  let dv = new DataView(new ArrayBuffer(0));
  let base = 0;
  let end = 0;
  let room = 0;
  let p = 0;
  let inbox = 0;
  let epoch = 0;
  let viewing: unknown[] | null = null;
  let back: [number, Value][] | null = null;
  const live = new FinalizationRegistry<number>((id) => x?.mrly_drop(id));

  const fresh = () => {
    if (u8.length === 0) {
      u8 = new Uint8Array(memory.buffer);
      dv = new DataView(memory.buffer);
    }
  };

  const start = () => {
    x = new WebAssembly.Instance(module as WebAssembly.Module, {}).exports as unknown as Exports;
    memory = x.memory;
    x.mrly_seed(epoch);
    room = 4096;
    base = x.mrly_buf(room);
    end = base + room;
    u8 = new Uint8Array(memory.buffer);
    dv = new DataView(memory.buffer);
    syms.length = 0;
    shapes.length = built.length;
    for (const bind of binders) bind();
  };

  const grow = (need: number) => {
    const used = p - base;
    room = Math.max(room * 2, used + need + 64);
    base = x.mrly_buf(room);
    end = base + room;
    p = base + used;
    fresh();
  };

  const text = (value: string) => {
    const n = value.length;
    if (p + n * 3 + 8 > end) grow(n * 3 + 8);
    const at = p + 4;
    if (n < 64) {
      let q = at;
      let plain = true;
      for (let i = 0; i < n; i++) {
        const c = value.charCodeAt(i);
        if (c > 127) {
          plain = false;
          break;
        }
        u8[q++] = c;
      }
      if (plain) {
        dv.setUint32(p, n, true);
        p = q;
        return;
      }
    }
    const { written } = encoder.encodeInto(value, u8.subarray(at, at + n * 3));
    dv.setUint32(p, written, true);
    p = at + written;
  };

  const blob = (array: InstanceType<Typed>) => {
    const kind = CODES.get(array.constructor) as number;
    const length = array.length;
    const at = x.mrly_in(kind, length);
    fresh();
    (new KINDS[kind](memory.buffer, at, length) as Uint8Array).set(array as Uint8Array);
    if (p + 16 > end) grow(16);
    u8[p] = BLOB_IN;
    dv.setUint32(p + 1, inbox++, true);
    p += 5;
  };

  const put = (value: Value, raw: boolean) => {
    if (p + 32 > end) grow(32);
    switch (typeof value) {
      case "number": {
        if ((value | 0) === value && !(value === 0 && 1 / value < 0)) {
          if (value >= 0 && value < 128) {
            u8[p++] = SMALL | value;
          } else {
            u8[p] = I32;
            dv.setInt32(p + 1, value, true);
            p += 5;
          }
        } else {
          u8[p] = Number.isSafeInteger(value) && value !== 0 ? I53 : F64;
          dv.setFloat64(p + 1, value, true);
          p += 9;
        }
        return;
      }
      case "string":
        u8[p++] = STR;
        text(value);
        return;
      case "boolean":
        u8[p++] = value ? TRUE : FALSE;
        return;
      case "undefined":
        u8[p++] = NUL;
        return;
      case "bigint":
        u8[p++] = BIG;
        text(String(value));
        return;
      default:
    }
    if (value === null) {
      u8[p++] = NUL;
      return;
    }
    const thing = value as Thing;
    if (thing.$h !== undefined) {
      if (thing.$u === name) {
        u8[p] = HANDLE;
        dv.setUint32(p + 1, thing.$h, true);
        p += 5;
      } else {
        put(thing.toJSON?.(), false);
      }
      return;
    }
    if (ArrayBuffer.isView(value)) {
      const array = value as InstanceType<Typed>;
      if (raw) {
        blob(array);
        return;
      }
      u8[p] = ARR;
      dv.setUint32(p + 1, array.length, true);
      p += 5;
      for (let i = 0; i < array.length; i++) put(array[i], false);
      return;
    }
    if (Array.isArray(value)) {
      const n = value.length;
      u8[p] = ARR;
      dv.setUint32(p + 1, n, true);
      p += 5;
      for (let i = 0; i < n; i++) put(value[i], false);
      return;
    }
    const shaped = "shape" in (value as object);
    u8[p] = MAP;
    const at = p + 1 - base;
    p += 5;
    let n = 0;
    const record = value as Record<string, Value>;
    for (const key in record) {
      const one = record[key];
      if (one === undefined) continue;
      if (p + 8 > end) grow(8);
      u8[p++] = STR;
      text(key);
      put(one, shaped && RAW.has(key));
      n++;
    }
    dv.setUint32(base + at, n, true);
  };

  const fail = (code: number, door: string) => {
    fresh();
    const at = x.mrly_out_ptr();
    const message = decoder.decode(new Uint8Array(memory.buffer, at, x.mrly_out_len()));
    const error = new Error(code === 1 ? `${door}${message}` : message) as Error & { kind: string };
    error.kind = code === 1 ? "usage" : "library";
    return error;
  };

  const rescue = (error: unknown, door: string) => {
    if (!(error instanceof WebAssembly.RuntimeError)) return error;
    let said = error.message;
    try {
      const at = x.mrly_panic_ptr();
      const n = x.mrly_panic_len();
      if (n > 0) said = decoder.decode(new Uint8Array(memory.buffer, at, n));
    } catch {}
    epoch++;
    start();
    const made = new Error(`${door} panicked: ${said}`) as Error & { kind: string };
    made.kind = "panic";
    return made;
  };

  const define = (at: number) => {
    const stop = x.mrly_out_ptr() + x.mrly_out_len();
    let q = at;
    while (q < stop) {
      const kind = u8[q];
      const id = u8[q + 1] | (u8[q + 2] << 8);
      q += 3;
      if (kind === 1) {
        const n = dv.getUint32(q, true);
        syms[id] = Object.keys({ [decoder.decode(u8.subarray(q + 4, q + 4 + n))]: 0 })[0];
        q += 4 + n;
      } else {
        const n = u8[q] | (u8[q + 1] << 8);
        q += 2;
        const fields: string[] = [];
        for (let i = 0; i < n; i++) fields.push(syms[u8[q + 2 * i] | (u8[q + 2 * i + 1] << 8)]);
        q += 2 * n;
        shapes[id] = (r) => {
          const out: Record<string, Value> = {};
          for (let i = 0; i < n; i++) out[fields[i]] = r();
          return out;
        };
      }
    }
  };

  const string = (q: number, n: number) => {
    if (n < 24) {
      let out = "";
      for (let i = 0; i < n; i++) {
        const c = u8[q + i];
        if (c > 127) return decoder.decode(u8.subarray(q, q + n));
        out += String.fromCharCode(c);
      }
      return out;
    }
    return decoder.decode(u8.subarray(q, q + n));
  };

  const wrap = (id: number, of: string) => {
    const made = Object.create((classes.get(of) as { prototype: object }).prototype) as Thing;
    made.$h = id;
    made.$u = name;
    live.register(made, id, made);
    return made;
  };

  const read: Read = () => {
    const tag = u8[p++];
    if (tag >= SMALL) return tag & 0x7f;
    switch (tag) {
      case F32: {
        const value = dv.getFloat32(p, true);
        p += 4;
        return value;
      }
      case F64:
      case I53: {
        const value = dv.getFloat64(p, true);
        p += 8;
        return value;
      }
      case I32: {
        const value = dv.getInt32(p, true);
        p += 4;
        return value;
      }
      case SHP: {
        const id = u8[p] | (u8[p + 1] << 8);
        p += 2;
        return shapes[id](read);
      }
      case ARR: {
        const n = dv.getUint32(p, true);
        p += 4;
        const out = new Array(n);
        for (let i = 0; i < n; i++) out[i] = read();
        return out;
      }
      case STR: {
        const n = dv.getUint32(p, true);
        const q = p + 4;
        p = q + n;
        return string(q, n);
      }
      case WIDE: {
        const n = dv.getUint32(p, true);
        const q = p + 4;
        p = q + n;
        return string(q, n);
      }
      case SYM: {
        const id = u8[p] | (u8[p + 1] << 8);
        p += 2;
        return syms[id];
      }
      case VAR: {
        const id = u8[p] | (u8[p + 1] << 8);
        p += 2;
        return { [syms[id]]: read() };
      }
      case FORM:
        p++;
        return read();
      case NUL:
        return undefined;
      case FALSE:
        return false;
      case TRUE:
        return true;
      case BLOB_OUT: {
        const kind = u8[p];
        const at = dv.getUint32(p + 1, true);
        const length = dv.getUint32(p + 9, true);
        p += 17;
        if (viewing) {
          const view = {
            get array() {
              return new KINDS[kind](memory.buffer, at, length);
            },
          };
          viewing.push(view);
          return view;
        }
        return new KINDS[kind](memory.buffer, at, length).slice();
      }
      case HANDLE: {
        const id = dv.getUint32(p, true);
        const sym = u8[p + 4] | (u8[p + 5] << 8);
        p += 6;
        return wrap(id, syms[sym]);
      }
      case MAP: {
        const n = dv.getUint32(p, true);
        p += 4;
        const out: Record<string, Value> = {};
        for (let i = 0; i < n; i++) {
          const key = read() as string;
          out[key] = read();
        }
        return out;
      }
      case BIG64: {
        const value = dv.getBigInt64(p, true);
        p += 8;
        return value;
      }
      case UBIG64: {
        const value = dv.getBigUint64(p, true);
        p += 8;
        return value;
      }
      case INLINE: {
        const made = KINDS[u8[p]];
        const q = p + 5;
        const stop = q + dv.getUint32(p + 1, true) * made.BYTES_PER_ELEMENT;
        p = stop;
        const copy = u8.slice(q, stop);
        const out = made === Uint8Array ? copy : new made(copy.buffer);
        if (!viewing) return out;
        const view = { array: out };
        viewing.push(view);
        return view;
      }
      case BIG: {
        const n = dv.getUint32(p, true);
        const q = p + 4;
        p = q + n;
        return BigInt(string(q, n));
      }
      case BACK: {
        const n = u8[p++];
        back = [];
        for (let i = 0; i < n; i++) {
          const at = u8[p++];
          back.push([at, read()]);
        }
        return read();
      }
      default:
        throw new Error(`unknown wire tag ${tag}`);
    }
  };

  const foreign = (args: Value[]) => {
    const swaps: [Thing, Thing][] = [];
    for (let i = 0; i < args.length; i++) {
      const one = args[i] as Thing | null | undefined;
      if (one && one.$h !== undefined && one.$u !== name && one.__state) {
        const Rng = classes.get("Rng") as unknown as { from_state(state: Value): Thing };
        const local = Rng.from_state(one.__state());
        args[i] = local;
        swaps.push([one, local]);
      }
    }
    return swaps;
  };

  const invoke = (door: number, input: Value[]) => {
    if (x === null) throw new Error(`${name}: call init() first`);
    let args = input;
    const swaps = args.some((one) => (one as Thing | null)?.$u !== undefined && (one as Thing).$u !== name)
      ? foreign((args = args.slice()))
      : null;
    try {
      fresh();
      inbox = 0;
      p = base;
      const count = args.length;
      u8[p++] = count;
      for (let i = 0; i < count; i++) {
        if (p + 8 > end) grow(8);
        const at = p - base;
        p += 4;
        put(args[i], true);
        dv.setUint32(base + at, p - base - at - 4, true);
      }
      const out = x.mrly_call(door, p - base);
      if (out < 0) throw fail(-out, names[door]);
      fresh();
      const defs = dv.getUint32(out, true);
      if (defs !== 0) define(out + defs);
      p = out + 4;
      const value = read();
      if (back !== null) {
        const list = back;
        back = null;
        for (const [at, one] of list) {
          const target = args[at] as object | null;
          if (target && typeof target === "object") Object.assign(target, one);
        }
      }
      return value;
    } catch (error) {
      throw rescue(error, names[door]);
    } finally {
      if (swaps) {
        for (const [was, local] of swaps) {
          try {
            was.__restore?.((local as Thing).__state?.());
          } catch {}
        }
      }
    }
  };

  const check = (value: number, door: string) => {
    if (value === value) return value;
    const code = x.mrly_failed();
    if (code !== 0) throw fail(code, door);
    return value;
  };

  const bound = (door: string, info: Direct, fns: Fns) => {
    const k = info.at;
    if (info.self) {
      const made = (h: number, a: number, b: number, c: number, d: number) => {
        try {
          return check(fns[k](h, a, b, c, d), door);
        } catch (error) {
          throw rescue(error, door);
        }
      };
      if (info.bool)
        return function (this: Thing, a: number, b: number, c: number, d: number) {
          return made(this.$h as number, a, b, c, d) !== 0;
        };
      return function (this: Thing, a: number, b: number, c: number, d: number) {
        return made(this.$h as number, a, b, c, d);
      };
    }
    const made = (a: number, b: number, c: number, d: number, e: number, f: number) => {
      try {
        const value = fns[k](a, b, c, d, e, f);
        return info.fail ? check(value, door) : value;
      } catch (error) {
        throw rescue(error, door);
      }
    };
    if (info.bool) return (a: number, b: number, c: number, d: number, e: number, f: number) => made(a, b, c, d, e, f) !== 0;
    return made;
  };

  const make = (full: string, info: ClassInfo, door: (at: number) => Fn, direct: Record<string, Direct>, fns: Fns) => {
    const at = (leaf: string) => index.get(`${full}.${leaf}`) as number;
    const fresh_door = index.get(`${full}.new`);
    const Made = class {
      $h?: number;
      $u?: string;
      constructor(...args: Value[]) {
        const made = invoke(fresh_door as number, args) as Thing;
        live.unregister(made);
        this.$h = made.$h;
        this.$u = name;
        live.register(this, this.$h as number, this);
      }
      free() {
        if (this.$h === undefined) return;
        live.unregister(this);
        x.mrly_drop(this.$h);
        this.$h = undefined;
      }
    };
    Object.defineProperty(Made, "name", { value: full.split(".").at(-1) });
    const proto = Made.prototype as unknown as Record<string, unknown>;
    const statics = Made as unknown as Record<string, unknown>;
    for (const [field, settable] of info.fields) {
      const get = at(`${field}$get`);
      const set = settable ? at(`${field}$set`) : -1;
      Object.defineProperty(proto, field, {
        get(this: Thing) {
          return invoke(get, [this]);
        },
        set(this: Thing, value: Value) {
          if (set >= 0) invoke(set, [this, value]);
        },
      });
    }
    for (const method of info.methods) {
      const one = at(method);
      const fast = direct[`${full}.${method}`];
      proto[method] = fast
        ? bound(`${full}.${method}`, fast, fns)
        : function (this: Thing, ...args: Value[]) {
            return invoke(one, [this, ...args]);
          };
    }
    for (const fixed of info.statics) {
      if (fixed !== "new") statics[fixed] = door(at(fixed));
    }
    if (info.from) statics.from = door(at("from"));
    if (info.json) {
      const one = at("toJSON");
      proto.toJSON = function (this: Thing) {
        return invoke(one, [this]);
      };
    }
    return Made;
  };

  const rngInfo: ClassInfo = {
    fields: [],
    methods: ["range", "boolean", "chance", "sample_indices", "order", "__state", "__restore"],
    statics: [],
    from: false,
    json: false,
  };
  const rngAt = offsets.Rng as number;
  rng.forEach((one, at) => {
    names[rngAt + at] = one;
    index.set(one, rngAt + at);
  });
  const rngDoor = (at: number): Fn => ((...args: Value[]) => invoke(at, args)) as unknown as Fn;
  const Rng = make("Rng", rngInfo, rngDoor, {}, []) as unknown as (new (seed: Value) => Thing) & { from_state(state: Value): Thing };
  classes.set("Rng", Rng as unknown as new (...args: never[]) => Thing);
  const proto = Rng.prototype as unknown as Record<string, unknown>;
  proto.unit = function (this: Thing) {
    try {
      return check(x.mrly_rng_unit(this.$h as number), "Rng.unit");
    } catch (error) {
      throw rescue(error, "Rng.unit");
    }
  };
  proto.below = function (this: Thing, n: number) {
    try {
      return check(x.mrly_rng_below(this.$h as number, n), "Rng.below");
    } catch (error) {
      throw rescue(error, "Rng.below");
    }
  };
  proto.choice = function (this: { below(n: number): number }, items: ArrayLike<Value>) {
    return items[this.below(items.length)];
  };
  proto.shuffle = function (this: { order(n: number): Uint32Array }, items: Value[]) {
    const moved = Array.from(this.order(items.length), (i) => items[i]);
    for (let i = 0; i < moved.length; i++) items[i] = moved[i];
  };
  Rng.from_state = (state: Value) => {
    const made = new Rng(0);
    (made as unknown as { __restore(state: Value): void }).__restore(state);
    return made;
  };

  const initSync = (given: { module: BufferSource | WebAssembly.Module } | BufferSource | WebAssembly.Module) => {
    if (module) return x;
    const source = (given as { module?: BufferSource | WebAssembly.Module }).module ?? (given as BufferSource | WebAssembly.Module);
    module = source instanceof WebAssembly.Module ? source : new WebAssembly.Module(source);
    start();
    return x;
  };

  const compile = async (given: Source) => {
    const source = (typeof given === "object" && "module_or_path" in given ? given.module_or_path : given) as unknown;
    if (source instanceof WebAssembly.Module) {
      module = source;
    } else if (source instanceof ArrayBuffer || ArrayBuffer.isView(source)) {
      module = await WebAssembly.compile(source as BufferSource);
    } else {
      const response = source instanceof Response ? source : await fetch(source as RequestInfo | URL);
      module = await WebAssembly.compile(await response.arrayBuffer());
    }
    start();
    return x;
  };

  const init = (given: Source) => {
    if (module) return Promise.resolve(x);
    pending ??= compile(given).catch((error) => {
      pending = null;
      throw error;
    });
    return pending;
  };

  const $view = (full: string, args: Value[]) => {
    viewing = [];
    try {
      const value = invoke(index.get(full) as number, args);
      const held = x.mrly_keep();
      return { value, kept: viewing, release: () => x.mrly_release(held) };
    } finally {
      viewing = null;
    }
  };

  const attach = (api: Api) => {
    const base = offsets[api.unit] ?? -1;
    if (base < 0 && api.doors.length) throw new Error(`${name}: the blob holds no crate ${api.unit}`);
    api.doors.forEach((one, at) => {
      names[base + at] = one;
      index.set(one, base + at);
    });
    const fns: Fns = [];
    const directs = Object.values(api.direct);
    const bind = () => {
      for (const info of directs) fns[info.at] = x[`mrly_${api.unit}_d${info.at}`];
    };
    binders.push(bind);
    if (x !== null) bind();

    const door = (at: number): Fn => {
      const full = api.doors[at] as string;
      const info = api.direct[full];
      if (info && !info.self) return bound(full, info, fns) as unknown as Fn;
      const absolute = base + at;
      return ((...args: Value[]) => invoke(absolute, args)) as unknown as Fn;
    };

    const constant = (at: number): Fn => {
      let held: Value;
      let done = false;
      const run = door(at);
      return (() => {
        if (!done) {
          held = (run as () => Value)();
          done = typeof held !== "object" || held === null;
        }
        return held;
      }) as Fn;
    };

    for (const [full, info] of Object.entries(api.classes)) {
      const absolute = (at: number) => door(at - base);
      classes.set(full, make(full, info, absolute, api.direct, fns) as unknown as new (...args: never[]) => Thing);
    }

    const klass = (full: string) => classes.get(full);

    return { door, constant, klass, init, initSync, Rng, $view };
  };

  return { attach, init, initSync, Rng, klass: (full: string) => classes.get(full) };
}

export function unit(home: ReturnType<typeof blob>, api: Api) {
  return home.attach(api);
}
