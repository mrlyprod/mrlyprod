/* CONSTANTS */

const GL = {
  NONE: 0x0, NO_ERROR: 0x0, POINTS: 0x0, ZERO: 0x0, LINES: 0x1, ONE: 0x1, LINE_LOOP: 0x2, LINE_STRIP: 0x3, TRIANGLES: 0x4, TRIANGLE_STRIP: 0x5,
  TRIANGLE_FAN: 0x6, DEPTH_BUFFER_BIT: 0x100, NEVER: 0x200, LESS: 0x201, EQUAL: 0x202, LEQUAL: 0x203, GREATER: 0x204, NOTEQUAL: 0x205, GEQUAL: 0x206,
  ALWAYS: 0x207, SRC_COLOR: 0x300, ONE_MINUS_SRC_COLOR: 0x301, SRC_ALPHA: 0x302, ONE_MINUS_SRC_ALPHA: 0x303, DST_ALPHA: 0x304,
  ONE_MINUS_DST_ALPHA: 0x305, DST_COLOR: 0x306, ONE_MINUS_DST_COLOR: 0x307, SRC_ALPHA_SATURATE: 0x308, STENCIL_BUFFER_BIT: 0x400, FRONT: 0x404,
  BACK: 0x405, FRONT_AND_BACK: 0x408, INVALID_ENUM: 0x500, INVALID_VALUE: 0x501, INVALID_OPERATION: 0x502, OUT_OF_MEMORY: 0x505,
  INVALID_FRAMEBUFFER_OPERATION: 0x506, CW: 0x900, CCW: 0x901, LINE_WIDTH: 0xb21, CULL_FACE: 0xb44, CULL_FACE_MODE: 0xb45, FRONT_FACE: 0xb46,
  DEPTH_RANGE: 0xb70, DEPTH_TEST: 0xb71, DEPTH_WRITEMASK: 0xb72, DEPTH_CLEAR_VALUE: 0xb73, DEPTH_FUNC: 0xb74, STENCIL_TEST: 0xb90,
  STENCIL_CLEAR_VALUE: 0xb91, STENCIL_FUNC: 0xb92, STENCIL_VALUE_MASK: 0xb93, STENCIL_FAIL: 0xb94, STENCIL_PASS_DEPTH_FAIL: 0xb95,
  STENCIL_PASS_DEPTH_PASS: 0xb96, STENCIL_REF: 0xb97, STENCIL_WRITEMASK: 0xb98, VIEWPORT: 0xba2, DITHER: 0xbd0, BLEND: 0xbe2, READ_BUFFER: 0xc02,
  SCISSOR_BOX: 0xc10, SCISSOR_TEST: 0xc11, COLOR_CLEAR_VALUE: 0xc22, COLOR_WRITEMASK: 0xc23, UNPACK_ROW_LENGTH: 0xcf2, UNPACK_SKIP_ROWS: 0xcf3,
  UNPACK_SKIP_PIXELS: 0xcf4, UNPACK_ALIGNMENT: 0xcf5, PACK_ROW_LENGTH: 0xd02, PACK_SKIP_ROWS: 0xd03, PACK_SKIP_PIXELS: 0xd04, PACK_ALIGNMENT: 0xd05,
  MAX_TEXTURE_SIZE: 0xd33, MAX_VIEWPORT_DIMS: 0xd3a, SUBPIXEL_BITS: 0xd50, RED_BITS: 0xd52, GREEN_BITS: 0xd53, BLUE_BITS: 0xd54, ALPHA_BITS: 0xd55,
  DEPTH_BITS: 0xd56, STENCIL_BITS: 0xd57, TEXTURE_2D: 0xde1, DONT_CARE: 0x1100, FASTEST: 0x1101, NICEST: 0x1102, BYTE: 0x1400, UNSIGNED_BYTE: 0x1401,
  SHORT: 0x1402, UNSIGNED_SHORT: 0x1403, INT: 0x1404, UNSIGNED_INT: 0x1405, FLOAT: 0x1406, HALF_FLOAT: 0x140b, INVERT: 0x150a, TEXTURE: 0x1702,
  COLOR: 0x1800, DEPTH: 0x1801, STENCIL: 0x1802, DEPTH_COMPONENT: 0x1902, RED: 0x1903, ALPHA: 0x1906, RGB: 0x1907, RGBA: 0x1908, LUMINANCE: 0x1909,
  LUMINANCE_ALPHA: 0x190a, KEEP: 0x1e00, REPLACE: 0x1e01, INCR: 0x1e02, DECR: 0x1e03, VENDOR: 0x1f00, RENDERER: 0x1f01, VERSION: 0x1f02,
  NEAREST: 0x2600, LINEAR: 0x2601, NEAREST_MIPMAP_NEAREST: 0x2700, LINEAR_MIPMAP_NEAREST: 0x2701, NEAREST_MIPMAP_LINEAR: 0x2702,
  LINEAR_MIPMAP_LINEAR: 0x2703, TEXTURE_MAG_FILTER: 0x2800, TEXTURE_MIN_FILTER: 0x2801, TEXTURE_WRAP_S: 0x2802, TEXTURE_WRAP_T: 0x2803,
  REPEAT: 0x2901, POLYGON_OFFSET_UNITS: 0x2a00, COLOR_BUFFER_BIT: 0x4000, CONSTANT_COLOR: 0x8001, ONE_MINUS_CONSTANT_COLOR: 0x8002,
  CONSTANT_ALPHA: 0x8003, ONE_MINUS_CONSTANT_ALPHA: 0x8004, BLEND_COLOR: 0x8005, FUNC_ADD: 0x8006, MIN: 0x8007, MAX: 0x8008, BLEND_EQUATION: 0x8009,
  BLEND_EQUATION_RGB: 0x8009, FUNC_SUBTRACT: 0x800a, FUNC_REVERSE_SUBTRACT: 0x800b, UNSIGNED_SHORT_4_4_4_4: 0x8033, UNSIGNED_SHORT_5_5_5_1: 0x8034,
  POLYGON_OFFSET_FILL: 0x8037, POLYGON_OFFSET_FACTOR: 0x8038, RGB8: 0x8051, RGBA4: 0x8056, RGB5_A1: 0x8057, RGBA8: 0x8058, RGB10_A2: 0x8059,
  TEXTURE_BINDING_2D: 0x8069, TEXTURE_BINDING_3D: 0x806a, UNPACK_SKIP_IMAGES: 0x806d, UNPACK_IMAGE_HEIGHT: 0x806e, TEXTURE_3D: 0x806f,
  TEXTURE_WRAP_R: 0x8072, MAX_3D_TEXTURE_SIZE: 0x8073, SAMPLE_ALPHA_TO_COVERAGE: 0x809e, SAMPLE_COVERAGE: 0x80a0, SAMPLE_BUFFERS: 0x80a8,
  SAMPLES: 0x80a9, SAMPLE_COVERAGE_VALUE: 0x80aa, SAMPLE_COVERAGE_INVERT: 0x80ab, BLEND_DST_RGB: 0x80c8, BLEND_SRC_RGB: 0x80c9,
  BLEND_DST_ALPHA: 0x80ca, BLEND_SRC_ALPHA: 0x80cb, CLAMP_TO_EDGE: 0x812f, TEXTURE_MIN_LOD: 0x813a, TEXTURE_MAX_LOD: 0x813b,
  TEXTURE_BASE_LEVEL: 0x813c, TEXTURE_MAX_LEVEL: 0x813d, GENERATE_MIPMAP_HINT: 0x8192, DEPTH_COMPONENT16: 0x81a5, DEPTH_COMPONENT24: 0x81a6,
  FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: 0x8210, FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: 0x8211, FRAMEBUFFER_ATTACHMENT_RED_SIZE: 0x8212,
  FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: 0x8213, FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: 0x8214, FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: 0x8215,
  FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: 0x8216, FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: 0x8217, FRAMEBUFFER_DEFAULT: 0x8218,
  DEPTH_STENCIL_ATTACHMENT: 0x821a, RG: 0x8227, RG_INTEGER: 0x8228, R8: 0x8229, RG8: 0x822b, R16F: 0x822d, R32F: 0x822e, RG16F: 0x822f,
  RG32F: 0x8230, R8I: 0x8231, R8UI: 0x8232, R16I: 0x8233, R16UI: 0x8234, R32I: 0x8235, R32UI: 0x8236, RG8I: 0x8237, RG8UI: 0x8238, RG16I: 0x8239,
  RG16UI: 0x823a, RG32I: 0x823b, RG32UI: 0x823c, TEXTURE_IMMUTABLE_LEVELS: 0x82df, UNSIGNED_SHORT_5_6_5: 0x8363, UNSIGNED_INT_2_10_10_10_REV: 0x8368,
  MIRRORED_REPEAT: 0x8370, ALIASED_POINT_SIZE_RANGE: 0x846d, ALIASED_LINE_WIDTH_RANGE: 0x846e, TEXTURE0: 0x84c0, TEXTURE1: 0x84c1, TEXTURE2: 0x84c2,
  TEXTURE3: 0x84c3, TEXTURE4: 0x84c4, TEXTURE5: 0x84c5, TEXTURE6: 0x84c6, TEXTURE7: 0x84c7, TEXTURE8: 0x84c8, TEXTURE9: 0x84c9, TEXTURE10: 0x84ca,
  TEXTURE11: 0x84cb, TEXTURE12: 0x84cc, TEXTURE13: 0x84cd, TEXTURE14: 0x84ce, TEXTURE15: 0x84cf, TEXTURE16: 0x84d0, TEXTURE17: 0x84d1,
  TEXTURE18: 0x84d2, TEXTURE19: 0x84d3, TEXTURE20: 0x84d4, TEXTURE21: 0x84d5, TEXTURE22: 0x84d6, TEXTURE23: 0x84d7, TEXTURE24: 0x84d8,
  TEXTURE25: 0x84d9, TEXTURE26: 0x84da, TEXTURE27: 0x84db, TEXTURE28: 0x84dc, TEXTURE29: 0x84dd, TEXTURE30: 0x84de, TEXTURE31: 0x84df,
  ACTIVE_TEXTURE: 0x84e0, MAX_RENDERBUFFER_SIZE: 0x84e8, DEPTH_STENCIL: 0x84f9, UNSIGNED_INT_24_8: 0x84fa, MAX_TEXTURE_LOD_BIAS: 0x84fd,
  INCR_WRAP: 0x8507, DECR_WRAP: 0x8508, TEXTURE_CUBE_MAP: 0x8513, TEXTURE_BINDING_CUBE_MAP: 0x8514, TEXTURE_CUBE_MAP_POSITIVE_X: 0x8515,
  TEXTURE_CUBE_MAP_NEGATIVE_X: 0x8516, TEXTURE_CUBE_MAP_POSITIVE_Y: 0x8517, TEXTURE_CUBE_MAP_NEGATIVE_Y: 0x8518, TEXTURE_CUBE_MAP_POSITIVE_Z: 0x8519,
  TEXTURE_CUBE_MAP_NEGATIVE_Z: 0x851a, MAX_CUBE_MAP_TEXTURE_SIZE: 0x851c, VERTEX_ARRAY_BINDING: 0x85b5, CURRENT_VERTEX_ATTRIB: 0x8626,
  COMPRESSED_TEXTURE_FORMATS: 0x86a3, BUFFER_SIZE: 0x8764, BUFFER_USAGE: 0x8765, STENCIL_BACK_FUNC: 0x8800, STENCIL_BACK_FAIL: 0x8801,
  STENCIL_BACK_PASS_DEPTH_FAIL: 0x8802, STENCIL_BACK_PASS_DEPTH_PASS: 0x8803, RGBA32F: 0x8814, RGB32F: 0x8815, RGBA16F: 0x881a, RGB16F: 0x881b,
  MAX_DRAW_BUFFERS: 0x8824, DRAW_BUFFER0: 0x8825, DRAW_BUFFER1: 0x8826, DRAW_BUFFER2: 0x8827, DRAW_BUFFER3: 0x8828, DRAW_BUFFER4: 0x8829,
  DRAW_BUFFER5: 0x882a, DRAW_BUFFER6: 0x882b, DRAW_BUFFER7: 0x882c, DRAW_BUFFER8: 0x882d, DRAW_BUFFER9: 0x882e, DRAW_BUFFER10: 0x882f,
  DRAW_BUFFER11: 0x8830, DRAW_BUFFER12: 0x8831, DRAW_BUFFER13: 0x8832, DRAW_BUFFER14: 0x8833, DRAW_BUFFER15: 0x8834, BLEND_EQUATION_ALPHA: 0x883d,
  TEXTURE_COMPARE_MODE: 0x884c, TEXTURE_COMPARE_FUNC: 0x884d, COMPARE_REF_TO_TEXTURE: 0x884e, CURRENT_QUERY: 0x8865, MAX_VERTEX_ATTRIBS: 0x8869,
  MAX_TEXTURE_IMAGE_UNITS: 0x8872, ARRAY_BUFFER: 0x8892, ELEMENT_ARRAY_BUFFER: 0x8893, ARRAY_BUFFER_BINDING: 0x8894,
  ELEMENT_ARRAY_BUFFER_BINDING: 0x8895, STREAM_DRAW: 0x88e0, STREAM_READ: 0x88e1, STREAM_COPY: 0x88e2, STATIC_DRAW: 0x88e4, STATIC_READ: 0x88e5,
  STATIC_COPY: 0x88e6, DYNAMIC_DRAW: 0x88e8, DYNAMIC_READ: 0x88e9, DYNAMIC_COPY: 0x88ea, DEPTH24_STENCIL8: 0x88f0, MAX_ARRAY_TEXTURE_LAYERS: 0x88ff,
  SAMPLER_BINDING: 0x8919, FRAGMENT_SHADER: 0x8b30, VERTEX_SHADER: 0x8b31, MAX_FRAGMENT_UNIFORM_COMPONENTS: 0x8b49,
  MAX_VERTEX_UNIFORM_COMPONENTS: 0x8b4a, MAX_VERTEX_TEXTURE_IMAGE_UNITS: 0x8b4c, MAX_COMBINED_TEXTURE_IMAGE_UNITS: 0x8b4d, SHADER_TYPE: 0x8b4f,
  FLOAT_VEC2: 0x8b50, FLOAT_VEC3: 0x8b51, FLOAT_VEC4: 0x8b52, INT_VEC2: 0x8b53, INT_VEC3: 0x8b54, INT_VEC4: 0x8b55, BOOL: 0x8b56, BOOL_VEC2: 0x8b57,
  BOOL_VEC3: 0x8b58, BOOL_VEC4: 0x8b59, FLOAT_MAT2: 0x8b5a, FLOAT_MAT3: 0x8b5b, FLOAT_MAT4: 0x8b5c, SAMPLER_2D: 0x8b5e, SAMPLER_3D: 0x8b5f,
  SAMPLER_CUBE: 0x8b60, SAMPLER_2D_SHADOW: 0x8b62, DELETE_STATUS: 0x8b80, COMPILE_STATUS: 0x8b81, LINK_STATUS: 0x8b82, VALIDATE_STATUS: 0x8b83,
  ATTACHED_SHADERS: 0x8b85, ACTIVE_UNIFORMS: 0x8b86, ACTIVE_ATTRIBUTES: 0x8b89, FRAGMENT_SHADER_DERIVATIVE_HINT: 0x8b8b,
  SHADING_LANGUAGE_VERSION: 0x8b8c, CURRENT_PROGRAM: 0x8b8d, IMPLEMENTATION_COLOR_READ_TYPE: 0x8b9a, IMPLEMENTATION_COLOR_READ_FORMAT: 0x8b9b,
  UNSIGNED_NORMALIZED: 0x8c17, TEXTURE_2D_ARRAY: 0x8c1a, TEXTURE_BINDING_2D_ARRAY: 0x8c1d, R11F_G11F_B10F: 0x8c3a,
  UNSIGNED_INT_10F_11F_11F_REV: 0x8c3b, RGB9_E5: 0x8c3d, UNSIGNED_INT_5_9_9_9_REV: 0x8c3e, SRGB: 0x8c40, SRGB8: 0x8c41, SRGB8_ALPHA8: 0x8c43,
  MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: 0x8c80, MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: 0x8c8a,
  MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: 0x8c8b, STENCIL_BACK_REF: 0x8ca3, STENCIL_BACK_VALUE_MASK: 0x8ca4, STENCIL_BACK_WRITEMASK: 0x8ca5,
  DRAW_FRAMEBUFFER_BINDING: 0x8ca6, FRAMEBUFFER_BINDING: 0x8ca6, RENDERBUFFER_BINDING: 0x8ca7, READ_FRAMEBUFFER: 0x8ca8, DRAW_FRAMEBUFFER: 0x8ca9,
  READ_FRAMEBUFFER_BINDING: 0x8caa, RENDERBUFFER_SAMPLES: 0x8cab, DEPTH_COMPONENT32F: 0x8cac, DEPTH32F_STENCIL8: 0x8cad,
  FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: 0x8cd0, FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: 0x8cd1, FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: 0x8cd2,
  FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: 0x8cd3, FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: 0x8cd4, FRAMEBUFFER_COMPLETE: 0x8cd5,
  FRAMEBUFFER_INCOMPLETE_ATTACHMENT: 0x8cd6, FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: 0x8cd7, FRAMEBUFFER_INCOMPLETE_DIMENSIONS: 0x8cd9,
  FRAMEBUFFER_UNSUPPORTED: 0x8cdd, MAX_COLOR_ATTACHMENTS: 0x8cdf, COLOR_ATTACHMENT0: 0x8ce0, COLOR_ATTACHMENT1: 0x8ce1, COLOR_ATTACHMENT2: 0x8ce2,
  COLOR_ATTACHMENT3: 0x8ce3, COLOR_ATTACHMENT4: 0x8ce4, COLOR_ATTACHMENT5: 0x8ce5, COLOR_ATTACHMENT6: 0x8ce6, COLOR_ATTACHMENT7: 0x8ce7,
  COLOR_ATTACHMENT8: 0x8ce8, COLOR_ATTACHMENT9: 0x8ce9, COLOR_ATTACHMENT10: 0x8cea, COLOR_ATTACHMENT11: 0x8ceb, COLOR_ATTACHMENT12: 0x8cec,
  COLOR_ATTACHMENT13: 0x8ced, COLOR_ATTACHMENT14: 0x8cee, COLOR_ATTACHMENT15: 0x8cef, DEPTH_ATTACHMENT: 0x8d00, STENCIL_ATTACHMENT: 0x8d20,
  FRAMEBUFFER: 0x8d40, RENDERBUFFER: 0x8d41, RENDERBUFFER_WIDTH: 0x8d42, RENDERBUFFER_HEIGHT: 0x8d43, RENDERBUFFER_INTERNAL_FORMAT: 0x8d44,
  STENCIL_INDEX8: 0x8d48, RENDERBUFFER_RED_SIZE: 0x8d50, RENDERBUFFER_GREEN_SIZE: 0x8d51, RENDERBUFFER_BLUE_SIZE: 0x8d52,
  RENDERBUFFER_ALPHA_SIZE: 0x8d53, RENDERBUFFER_DEPTH_SIZE: 0x8d54, RENDERBUFFER_STENCIL_SIZE: 0x8d55, FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: 0x8d56,
  MAX_SAMPLES: 0x8d57, RGB565: 0x8d62, RGBA32UI: 0x8d70, RGB32UI: 0x8d71, RGBA16UI: 0x8d76, RGB16UI: 0x8d77, RGBA8UI: 0x8d7c, RGB8UI: 0x8d7d,
  RGBA32I: 0x8d82, RGB32I: 0x8d83, RGBA16I: 0x8d88, RGB16I: 0x8d89, RGBA8I: 0x8d8e, RGB8I: 0x8d8f, RED_INTEGER: 0x8d94, RGB_INTEGER: 0x8d98,
  RGBA_INTEGER: 0x8d99, INT_2_10_10_10_REV: 0x8d9f, FLOAT_32_UNSIGNED_INT_24_8_REV: 0x8dad, SAMPLER_2D_ARRAY: 0x8dc1,
  SAMPLER_2D_ARRAY_SHADOW: 0x8dc4, SAMPLER_CUBE_SHADOW: 0x8dc5, UNSIGNED_INT_VEC2: 0x8dc6, UNSIGNED_INT_VEC3: 0x8dc7, UNSIGNED_INT_VEC4: 0x8dc8,
  INT_SAMPLER_2D: 0x8dca, INT_SAMPLER_3D: 0x8dcb, INT_SAMPLER_CUBE: 0x8dcc, INT_SAMPLER_2D_ARRAY: 0x8dcf, UNSIGNED_INT_SAMPLER_2D: 0x8dd2,
  UNSIGNED_INT_SAMPLER_3D: 0x8dd3, UNSIGNED_INT_SAMPLER_CUBE: 0x8dd4, UNSIGNED_INT_SAMPLER_2D_ARRAY: 0x8dd7, LOW_FLOAT: 0x8df0, MEDIUM_FLOAT: 0x8df1,
  HIGH_FLOAT: 0x8df2, LOW_INT: 0x8df3, MEDIUM_INT: 0x8df4, HIGH_INT: 0x8df5, MAX_VERTEX_UNIFORM_VECTORS: 0x8dfb,
  MAX_FRAGMENT_UNIFORM_VECTORS: 0x8dfd, R8_SNORM: 0x8f94, RG8_SNORM: 0x8f95, RGB8_SNORM: 0x8f96, RGBA8_SNORM: 0x8f97, SIGNED_NORMALIZED: 0x8f9c,
  RGB10_A2UI: 0x906f, TEXTURE_IMMUTABLE_FORMAT: 0x912f, UNPACK_FLIP_Y_WEBGL: 0x9240, UNPACK_PREMULTIPLY_ALPHA_WEBGL: 0x9241,
};

const TYPES = {
  float: GL.FLOAT, vec2: GL.FLOAT_VEC2, vec3: GL.FLOAT_VEC3, vec4: GL.FLOAT_VEC4, int: GL.INT, uint: GL.UNSIGNED_INT,
  ivec2: GL.INT_VEC2, ivec3: GL.INT_VEC3, ivec4: GL.INT_VEC4, bool: GL.BOOL, mat2: GL.FLOAT_MAT2, mat3: GL.FLOAT_MAT3,
  mat4: GL.FLOAT_MAT4, sampler2D: GL.SAMPLER_2D, sampler3D: GL.SAMPLER_3D, samplerCube: GL.SAMPLER_CUBE,
};

const EXTENSIONS = ['EXT_color_buffer_float', 'WEBGL_lose_context'];

const declared = (source, word) => {
  const found = [];
  const pattern = new RegExp(`${word}\\s+(?:(?:lowp|mediump|highp)\\s+)?(\\w+)\\s+(\\w+)\\s*(\\[\\s*(\\d+)\\s*\\])?\\s*;`, 'g');
  for (const [, type, name, list, size] of source.matchAll(pattern)) found.push({ type, name: list ? `${name}[0]` : name, size: list ? Number(size) : 1 });
  return found;
};

const flat = (value) => (ArrayBuffer.isView(value) || Array.isArray(value) ? Array.from(value) : value);

const DRAWS = /^draw(Arrays|Elements)(Instanced)?$/;

/* GL */

export function gl({ bad = null, ext = EXTENSIONS } = {}) {
  const log = [];
  let made = 0;
  let port = [0, 0, 1, 1];
  let used = null;
  let framed = null;
  let unit = 0;
  const shaders = new Map();
  const programs = new Map();
  const values = new Map();
  const bound = {};
  const drawn = [];
  const own = {
    log,
    draws: () => drawn.map((one) => ({ ...one })),
    useProgram: (prog) => {
      log.push(['useProgram', prog]);
      used = prog;
    },
    bindFramebuffer: (target, buffer) => {
      log.push(['bindFramebuffer', target, buffer]);
      framed = buffer;
    },
    activeTexture: (slot) => {
      log.push(['activeTexture', slot]);
      unit = slot - GL.TEXTURE0;
    },
    bindTexture: (target, tex) => {
      log.push(['bindTexture', target, tex]);
      bound[unit] = tex?.id ?? null;
    },
    canvas: { width: 1, height: 1 },
    drawingBufferWidth: 1,
    drawingBufferHeight: 1,
    shaderSource: (shader, source) => {
      log.push(['shaderSource', shader, source]);
      shaders.set(shader, source);
    },
    getShaderParameter: (shader) => !(bad && shaders.get(shader)?.includes(bad)),
    getShaderInfoLog: (shader) => (bad && shaders.get(shader)?.includes(bad) ? `ERROR: 0:1: '${bad}' : syntax error` : ''),
    attachShader: (prog, shader) => {
      log.push(['attachShader', prog, shader]);
      programs.get(prog).push(shaders.get(shader) ?? '');
    },
    getProgramParameter: (prog, key) => {
      if (key !== GL.ACTIVE_UNIFORMS) return true;
      return own.uniforms(prog).length;
    },
    uniforms: (prog) => {
      const seen = new Map();
      for (const source of programs.get(prog) ?? []) for (const u of declared(source, 'uniform')) seen.set(u.name, u);
      return [...seen.values()];
    },
    getActiveUniform: (prog, i) => {
      const u = own.uniforms(prog)[i];
      return u ? { name: u.name, size: u.size, type: TYPES[u.type] ?? GL.FLOAT } : null;
    },
    getUniformLocation: (prog, name) => ({ name: name.replace('[0]', '') }),
    getAttribLocation: (prog, name) => {
      const names = (programs.get(prog) ?? []).flatMap((source) => declared(source, '\\bin').map((a) => a.name));
      return names.indexOf(name);
    },
    getProgramInfoLog: () => '',
    getExtension: (name) => {
      log.push(['getExtension', name]);
      return ext.includes(name) ? { loseContext: () => log.push(['loseContext']) } : null;
    },
    checkFramebufferStatus: () => GL.FRAMEBUFFER_COMPLETE,
    viewport: (...args) => {
      log.push(['viewport', ...args]);
      port = args;
    },
    getParameter: (key) => (key === GL.VIEWPORT ? Int32Array.from(port) : null),
    isContextLost: () => false,
  };
  const note = (key) => (...args) => {
    log.push([key, ...args]);
    const value = key.startsWith('uniformMatrix') ? args[2] : args.length > 2 ? args.slice(1) : args[1];
    if (!values.has(used)) values.set(used, {});
    values.get(used)[args[0].name] = flat(value);
  };
  const paint = (key) => (...args) => {
    log.push([key, ...args]);
    const [mode, first, count, instances] = key.includes('Arrays') ? args : [args[0], 0, args[1], args[4]];
    const [vs = '', fs = ''] = programs.get(used) ?? [];
    drawn.push({ vs, fs, uniforms: { ...values.get(used) }, units: { ...bound }, fb: framed, viewport: [...port], mode, count, instances: instances ?? 1 });
  };
  const call = (key) => (...args) => {
    log.push([key, ...args]);
    if (!key.startsWith('create')) return undefined;
    const token = { kind: key.slice(6), id: ++made };
    if (key === 'createProgram') programs.set(token, []);
    return token;
  };
  return new Proxy(own, {
    get: (target, key) => {
      if (key in target) return target[key];
      if (key in GL) return GL[key];
      if (typeof key !== 'string') return undefined;
      if (/^[A-Z][A-Z0-9_]*$/.test(key)) throw new Error(`fake gl: no constant ${key}`);
      if (/^uniform(Matrix)?\d/.test(key)) return note(key);
      if (DRAWS.test(key)) return paint(key);
      return call(key);
    },
  });
}

const PARAMS = {
  Oscillator: { frequency: 440, detune: 0 },
  Gain: { gain: 1 },
  BiquadFilter: { frequency: 350, Q: 1, gain: 0, detune: 0 },
  BufferSource: { playbackRate: 1, detune: 0 },
  Delay: { delayTime: 0 },
  DynamicsCompressor: { threshold: -24, knee: 30, ratio: 12, attack: 0.003, release: 0.25 },
  StereoPanner: { pan: 0 },
  ConstantSource: { offset: 1 },
};

const NODES = ['Oscillator', 'Gain', 'BiquadFilter', 'BufferSource', 'Convolver', 'Delay', 'DynamicsCompressor', 'MediaStreamDestination', 'WaveShaper', 'StereoPanner', 'ConstantSource'];

const VERBS = ['setValueAtTime', 'linearRampToValueAtTime', 'exponentialRampToValueAtTime', 'setTargetAtTime', 'setValueCurveAtTime', 'cancelScheduledValues', 'cancelAndHoldAtTime'];

export function audio({ rate = 44100, time = 0, length = 0 } = {}) {
  const log = [];
  const nodes = [];
  const buffer = (channels, frames, sampleRate) => {
    const data = Array.from({ length: channels }, () => new Float32Array(frames));
    return { numberOfChannels: channels, length: frames, sampleRate, duration: frames / sampleRate, getChannelData: (i) => data[i] };
  };
  const param = (node, name, value) => {
    const p = { node, name, value, calls: [] };
    for (const verb of VERBS) {
      p[verb] = (...args) => {
        p.calls.push([verb, ...args]);
        log.push([verb, node.id, name, ...args]);
        return p;
      };
    }
    return p;
  };
  const make = (kind, extra = {}) => {
    const node = { id: nodes.length, kind, outputs: [], started: null, stopped: null, ...extra };
    for (const [name, value] of Object.entries(PARAMS[kind] ?? {})) node[name] = param(node, name, value);
    node.connect = (to) => {
      node.outputs.push(to);
      log.push(['connect', node.id, to.kind ? to.id : `${to.node.id}.${to.name}`]);
      return to.kind ? to : undefined;
    };
    node.disconnect = () => {
      node.outputs = [];
      log.push(['disconnect', node.id]);
    };
    node.start = (...args) => {
      node.started = args;
      log.push(['start', node.id, ...args]);
    };
    node.stop = (...args) => {
      node.stopped = args;
      log.push(['stop', node.id, ...args]);
    };
    nodes.push(node);
    log.push(['create', node.id, kind]);
    return node;
  };
  const ctx = {
    log,
    nodes,
    sampleRate: rate,
    currentTime: time,
    length,
    state: 'running',
    createBuffer: buffer,
    resume: () => {
      ctx.state = 'running';
      log.push(['resume']);
      return Promise.resolve();
    },
    suspend: () => {
      ctx.state = 'suspended';
      log.push(['suspend']);
      return Promise.resolve();
    },
    close: () => {
      ctx.state = 'closed';
      log.push(['close']);
      return Promise.resolve();
    },
    startRendering: () => Promise.resolve(buffer(2, ctx.length, rate)),
    find: (kind) => nodes.filter((node) => node.kind === kind),
  };
  for (const kind of NODES) ctx[`create${kind}`] = (...args) => make(kind, kind === 'MediaStreamDestination' ? { stream: { getAudioTracks: () => [] } } : kind === 'Delay' ? { max: args[0] } : {});
  ctx.destination = make('Destination');
  return ctx;
}
