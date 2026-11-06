import type { EmojiName, Glyphs } from "@mrlyprod/ui/shell/design.ts";
import { BOX } from "../own/stamp.ts";
import cool from "./cool.ts";
import fire from "./fire.ts";
import grin from "./grin.ts";
import heart from "./heart.ts";
import joy from "./joy.ts";
import love from "./love.ts";
import party from "./party.ts";
import sad from "./sad.ts";
import smile from "./smile.ts";
import spark from "./spark.ts";
import thumb from "./thumb.ts";
import wink from "./wink.ts";

export const emojis: Glyphs<EmojiName> = {
  box: BOX,
  crisp: true,
  draw: {
    smile,
    grin,
    joy,
    wink,
    love,
    cool,
    sad,
    heart,
    thumb,
    fire,
    spark,
    party,
  },
};
