/* TEXT */

const ESC: Record<string, string> = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" };

export const esc = (text: unknown) => String(text).replace(/[&<>"]/g, (c) => ESC[c]!);
