// The smallest drawing: a pen, one disc, one file.
import { raster } from "@mrlyprod/draw";
import { save } from "./save.ts";

// A color is [red, green, blue, alpha], each from 0 to 255.
const pen = raster(240, 240, [0, 0, 0, 255]);
pen.disc(120, 120, 60, [255, 255, 255, 255]);

save(import.meta.file, pen);
