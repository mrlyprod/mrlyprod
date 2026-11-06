import type { Glyphs, IconName } from "@mrlyprod/ui/shell/design.ts";
import { BOX } from "../own/icon.ts";
import add from "./add.ts";
import calendar from "./calendar.ts";
import cart from "./cart.ts";
import check from "./check.ts";
import clock from "./clock.ts";
import close from "./close.ts";
import code from "./code.ts";
import command from "./command.ts";
import data from "./data.ts";
import dice from "./dice.ts";
import down from "./down.ts";
import download from "./download.ts";
import edit from "./edit.ts";
import file from "./file.ts";
import folder from "./folder.ts";
import grid from "./grid.ts";
import image from "./image.ts";
import info from "./info.ts";
import keyboard from "./keyboard.ts";
import left from "./left.ts";
import levels from "./levels.ts";
import link from "./link.ts";
import lock from "./lock.ts";
import minus from "./minus.ts";
import more from "./more.ts";
import note from "./note.ts";
import paneLeft from "./pane-left.ts";
import paneRight from "./pane-right.ts";
import right from "./right.ts";
import search from "./search.ts";
import share from "./share.ts";
import shift from "./shift.ts";
import shrink from "./shrink.ts";
import star from "./star.ts";
import starFill from "./star-fill.ts";
import trash from "./trash.ts";
import tune from "./tune.ts";
import up from "./up.ts";
import volume from "./volume.ts";
import warn from "./warn.ts";

export const icons: Glyphs<IconName> = {
  box: BOX,
  draw: {
    add,
    minus,
    check,
    close,
    left,
    right,
    up,
    down,
    search,
    more,
    grid,
    "pane-left": paneLeft,
    "pane-right": paneRight,
    shrink,
    share,
    download,
    link,
    info,
    warn,
    lock,
    cart,
    star,
    "star-fill": starFill,
    dice,
    tune,
    levels,
    note,
    volume,
    keyboard,
    command,
    shift,
    calendar,
    clock,
    trash,
    edit,
    folder,
    file,
    code,
    image,
    data,
  },
};
