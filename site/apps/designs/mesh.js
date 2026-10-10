export const TILT = Math.atan(Math.SQRT1_2);
const SPAN = 2;

export function mesh(quads) {
  const faces = quads.length;
  const position = new Float32Array(faces * 12);
  const normal = new Float32Array(faces * 12);
  const index = new Uint32Array(faces * 6);
  quads.forEach(({ normal: n, verts }, face) => {
    for (let k = 0; k < 4; k++) {
      const at = face * 12 + k * 3;
      const v = verts[k];
      position[at] = v.x;
      position[at + 1] = v.y;
      position[at + 2] = v.z;
      normal[at] = n.x;
      normal[at + 1] = n.y;
      normal[at + 2] = n.z;
    }
    const corner = face * 4;
    index.set([corner, corner + 1, corner + 2, corner, corner + 2, corner + 3], face * 6);
  });
  return { position, normal, index, faces };
}

export function eye(theta, phi, reach) {
  const flat = Math.cos(phi) * reach;
  return [Math.cos(theta) * flat, Math.sin(theta) * flat, Math.sin(phi) * reach];
}

export function frustum(aspect, fit) {
  const top = Math.max((SPAN * Math.SQRT2) / aspect, SPAN * Math.sqrt(3)) / (2 * fit);
  return { left: -top * aspect, right: top * aspect, top, bottom: -top };
}
