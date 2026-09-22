import { expect, test } from "bun:test";

const manifest = await Bun.file(new URL("../../bridge/manifest.json", import.meta.url)).json();
const units = (await Bun.file(new URL("../../bridge/units.txt", import.meta.url)).text())
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
const pkg = await Bun.file(new URL("./package.json", import.meta.url)).json();
const kinds = new Map(manifest.types.map((t) => [t.path, t.cross.kind]));

function parent(path) {
    const at = path.lastIndexOf("::");
    return at < 0 ? "" : path.slice(0, at);
}

function relative(unit, path) {
    return unit === "all" ? path : path.slice(unit.length + 2);
}

function walk(mod, path) {
    let node = mod;
    for (const seg of path.split("::").filter(Boolean)) {
        if (node === undefined || node === null) return undefined;
        node = node[seg];
    }
    return node;
}

function holds(unit, path) {
    return unit === "all" || path.split("::")[0] === unit;
}

function check(mod, unit, f) {
    const kind = f.owner ? kinds.get(f.owner) : undefined;
    if (kind === "class") {
        const cls = walk(mod, relative(unit, f.owner));
        if (typeof cls !== "function") return false;
        if (f.self_kind === null && f.name === "new") return true;
        if (f.self_kind === null) return typeof cls[f.name] === "function";
        return typeof cls.prototype[f.name] === "function";
    }
    if (kind === "plain" || kind === "enum") {
        const holder = walk(mod, relative(unit, f.owner));
        return typeof holder === "function" && typeof holder[f.name] === "function";
    }
    return typeof walk(mod, relative(unit, f.path)) === "function";
}

for (const unit of units) {
    test(`${unit} exports every ok name`, async () => {
        expect(pkg.exports[unit === "all" ? "." : `./${unit}`]).toBeDefined();
        const mod = await import(`./${unit}.js`);
        const bytes = await Bun.file(new URL(`./pkg/${unit}/mrlyjs_${unit}_bg.wasm`, import.meta.url)).arrayBuffer();
        mod.initSync({ module: bytes });
        expect(typeof mod.Rng).toBe("function");
        const missing = [];
        const names = new Set();
        for (const f of manifest.functions) {
            if (f.cross.status !== "ok" || !holds(unit, f.module)) continue;
            names.add(f.path);
            if (!check(mod, unit, f)) missing.push(f.path);
        }
        for (const c of manifest.consts) {
            if (c.cross.status !== "ok" || !holds(unit, c.path)) continue;
            names.add(c.path);
            if (typeof walk(mod, relative(unit, c.path)) !== "function") missing.push(c.path);
        }
        for (const t of manifest.types) {
            if (t.cross.kind !== "class" || !holds(unit, t.path)) continue;
            names.add(t.path);
            if (typeof walk(mod, relative(unit, t.path)) !== "function") missing.push(t.path);
        }
        expect(missing).toEqual([]);
        console.log(`${unit}: ${names.size} names`);
    });
}
