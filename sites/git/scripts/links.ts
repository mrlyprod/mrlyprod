import { scan as walk } from "./scan"
import { scan } from "../src/lib/md"
import type { Miss } from "../src/lib/md"

// MAIN

const { ctx, text } = walk()
const docs = new Set<string>()
for (const dir of ctx.dirs.values()) if (dir.readme !== undefined) docs.add(dir.readme)
for (const file of ctx.files.values()) if (file.doc) docs.add(file.path)

const misses: Miss[] = []
for (const path of [...docs].sort()) misses.push(...(await scan(ctx, path, text(path))))

for (const miss of misses) console.log(`${miss.path}: broken ${miss.kind} ${miss.value}`)
console.log(`links: ${docs.size} docs, ${misses.length} broken`)
if (misses.length > 0) process.exit(1)
