import { expect, test } from "bun:test";
import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";

const manifest = await Bun.file(new URL("../../crates/bridge/manifest.json", import.meta.url)).json();
const units = readdirSync(new URL("./units", import.meta.url)).sort();
const pkg = await Bun.file(new URL("./package.json", import.meta.url)).json();
const kinds = new Map(manifest.types.map((t) => [t.path, t.cross.kind]));
const reserved = new Set(["new", "void", "class", "function", "default", "delete", "in", "var", "let", "const", "export", "import", "switch", "case", "for", "if", "else", "this", "super", "with", "yield", "enum", "await", "typeof", "instanceof", "return", "try", "catch", "finally", "throw", "while", "do", "break", "continue", "debugger", "static", "interface", "package", "private", "protected", "public", "implements", "null", "true", "false", "extends"]);

function parent(path) {
    const at = path.lastIndexOf("::");
    return at < 0 ? "" : path.slice(0, at);
}

function relative(unit, path) {
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

function holds(unit, path) {
    return path.split("::")[0] === unit;
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

test("every export ships in files and exists", () => {
    const listed = new Set(pkg.files);
    for (const file of ["LICENSE", "README.md"]) {
        expect(listed.has(file)).toBe(true);
        expect(existsSync(new URL(`./${file}`, import.meta.url))).toBe(true);
    }
    const license = new URL("./LICENSE", import.meta.url);
    expect(lstatSync(license).isSymbolicLink()).toBe(false);
    expect(readFileSync(license, "utf8")).toBe(readFileSync(new URL("../../LICENSE", import.meta.url), "utf8"));
    const loose = new Set(["./pkg/*", "./package.json"]);
    expect(pkg.exports["./package.json"]).toBe("./package.json");
    for (const [key, entry] of Object.entries(pkg.exports)) {
        if (loose.has(key)) continue;
        for (const file of Object.values(entry)) {
            expect(listed.has(file.slice(2))).toBe(true);
            expect(existsSync(new URL(file, import.meta.url))).toBe(true);
        }
    }
    const subpaths = Object.keys(pkg.exports).filter((key) => key.startsWith("./") && !loose.has(key));
    expect(subpaths.map((key) => key.slice(2)).sort()).toEqual(units);
});

for (const unit of units) {
    test(`${unit} exports every ok name`, async () => {
        expect(pkg.exports[`./${unit}`]).toEqual({ types: `./${unit}.d.ts`, node: `./${unit}.node.js`, default: `./${unit}.js` });
        const mod = await import(`./${unit}.js`);
        const bytes = await Bun.file(new URL(`./pkg/${unit}/mrly_js_${unit}_bg.wasm`, import.meta.url)).arrayBuffer();
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
