
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
*/
export function greet() {
    wasm.greet();
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}
/**
* @param {string} rotor1_map
* @param {string} rotor2_map
* @param {string} rotor3_map
* @param {bigint} rotor1_key
* @param {bigint} rotor2_key
* @param {bigint} rotor3_key
* @param {bigint} pair1_left
* @param {bigint} pair1_right
* @param {bigint} pair2_left
* @param {bigint} pair2_right
* @param {bigint} pair3_left
* @param {bigint} pair3_right
* @param {string} raw
* @returns {string}
*/
export function enigma_encrypt_with_settings_inline(rotor1_map, rotor2_map, rotor3_map, rotor1_key, rotor2_key, rotor3_key, pair1_left, pair1_right, pair2_left, pair2_right, pair3_left, pair3_right, raw) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(rotor1_map, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(rotor2_map, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(rotor3_map, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        uint64CvtShim[0] = rotor1_key;
        const low3 = u32CvtShim[0];
        const high3 = u32CvtShim[1];
        uint64CvtShim[0] = rotor2_key;
        const low4 = u32CvtShim[0];
        const high4 = u32CvtShim[1];
        uint64CvtShim[0] = rotor3_key;
        const low5 = u32CvtShim[0];
        const high5 = u32CvtShim[1];
        uint64CvtShim[0] = pair1_left;
        const low6 = u32CvtShim[0];
        const high6 = u32CvtShim[1];
        uint64CvtShim[0] = pair1_right;
        const low7 = u32CvtShim[0];
        const high7 = u32CvtShim[1];
        uint64CvtShim[0] = pair2_left;
        const low8 = u32CvtShim[0];
        const high8 = u32CvtShim[1];
        uint64CvtShim[0] = pair2_right;
        const low9 = u32CvtShim[0];
        const high9 = u32CvtShim[1];
        uint64CvtShim[0] = pair3_left;
        const low10 = u32CvtShim[0];
        const high10 = u32CvtShim[1];
        uint64CvtShim[0] = pair3_right;
        const low11 = u32CvtShim[0];
        const high11 = u32CvtShim[1];
        const ptr12 = passStringToWasm0(raw, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len12 = WASM_VECTOR_LEN;
        wasm.enigma_encrypt_with_settings_inline(retptr, ptr0, len0, ptr1, len1, ptr2, len2, low3, high3, low4, high4, low5, high5, low6, high6, low7, high7, low8, high8, low9, high9, low10, high10, low11, high11, ptr12, len12);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_alert_55a28e00e09821a5 = function(arg0, arg1) {
        alert(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();


    return wasm;
}

function initSync(bytes) {
    const imports = getImports();

    initMemory(imports);

    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('enigma_core_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
