import { expect, test } from "bun:test";

const manifest = await Bun.file(new URL("../manifest.json", import.meta.url)).json();
const unit = "num";
const kinds = new Map(manifest.types.map((t) => [t.path, t.cross.kind]));
const reserved = new Set(["new", "void", "class", "function", "default", "delete", "in", "var", "let", "const", "export", "import", "switch", "case", "for", "if", "else", "this", "super", "with", "yield", "enum", "await", "typeof", "instanceof", "return", "try", "catch", "finally", "throw", "while", "do", "break", "continue", "debugger", "static", "interface", "package", "private", "protected", "public", "implements", "null", "true", "false", "extends"]);

function relative(path) {
    return path.slice(unit.length + 2);
}

function walk(mod, path) {
    let node = mod;
    const segs = path.split("::").filter(Boolean);
    for (const [at, seg] of segs.entries()) {
        if (node === undefined || node === null) return undefined;
        node = node[at === segs.length - 1 && reserved.has(seg) ? `${seg}_` : seg];
    }
    return node;
}

function holds(path) {
    return path.split("::")[0] === unit;
}

function check(mod, f) {
    const kind = f.owner ? kinds.get(f.owner) : undefined;
    if (kind === "class") {
        const cls = walk(mod, relative(f.owner));
        if (typeof cls !== "function") return false;
        if (f.self_kind === null && f.name === "new") return true;
        if (f.self_kind === null) return typeof cls[f.name] === "function";
        return typeof cls.prototype[f.name] === "function";
    }
    if (kind === "plain" || kind === "enum") {
        const holder = walk(mod, relative(f.owner));
        return typeof holder === "function" && typeof holder[f.name] === "function";
    }
    return typeof walk(mod, relative(f.path)) === "function";
}

test(`${unit} exports every ok name`, async () => {
    const mod = await import("./index.js");
    const bytes = await Bun.file(new URL(`./pkg/mrly_${unit}_js_bg.wasm`, import.meta.url)).arrayBuffer();
    mod.initSync({ module: bytes });
    expect(typeof mod.Rng).toBe("function");
    const missing = [];
    const names = new Set();
    for (const f of manifest.functions) {
        if (f.cross.status !== "ok" || !holds(f.module)) continue;
        names.add(f.path);
        if (!check(mod, f)) missing.push(f.path);
    }
    for (const c of manifest.consts) {
        if (c.cross.status !== "ok" || !holds(c.path)) continue;
        names.add(c.path);
        if (typeof walk(mod, relative(c.path)) !== "function") missing.push(c.path);
    }
    for (const t of manifest.types) {
        if (t.cross.kind !== "class" || !holds(t.path)) continue;
        names.add(t.path);
        if (typeof walk(mod, relative(t.path)) !== "function") missing.push(t.path);
    }
    expect(missing).toEqual([]);
    console.log(`${unit}: ${names.size} names`);
});
