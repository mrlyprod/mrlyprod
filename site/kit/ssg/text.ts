/* TEXT */

const ESC: Record<string, string> = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" };

export const escape = (text: unknown) => String(text).replace(/[&<>"]/g, (c) => ESC[c]!);
