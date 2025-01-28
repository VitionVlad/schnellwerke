import { Jsaudiosource, Jsaudioctx } from './snippets/schnellwerke-2bcd3832ec3c917d/src/engine/audio/audio.js';
import { Jskeyboard, Jsmouse, Jstouch, Jsgamepad } from './snippets/schnellwerke-2bcd3832ec3c917d/src/engine/input/input.js';
import { Gfxrender, Gfxmesh, Jsloop, snlll } from './snippets/schnellwerke-2bcd3832ec3c917d/src/engine/render/gfx.js';
import { Jsrelod, Jsloadsdf, get_text_iframe } from './snippets/schnellwerke-2bcd3832ec3c917d/src/engine/resourceloader/resloader.js';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

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
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_10(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7655ddb00cd7a299(arg0, arg1);
}

/**
*/
export function main() {
    wasm.main();
}

async function __wbg_load(module, imports) {
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

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_log_326e941fc7b658de = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_gettextiframe_6556e4fbb33fa6b6 = function(arg0, arg1) {
        const ret = get_text_iframe(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_21d5ad0eddbedbe1 = function(arg0, arg1, arg2) {
        const ret = new Jsaudiosource(getObject(arg0), getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_jgetuse_d31ae9ac45a98ad7 = function(arg0) {
        const ret = getObject(arg0).jgetuse();
        return ret;
    };
    imports.wbg.__wbg_jgettx_abeca2286ef9ab4b = function(arg0) {
        const ret = getObject(arg0).jgettx();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizex_5bb48f3eca6ca959 = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizex();
        return ret;
    };
    imports.wbg.__wbg_jgetty_9a47144a3cba86df = function(arg0) {
        const ret = getObject(arg0).jgetty();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizey_eef8b4b6a77bef8f = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizey();
        return ret;
    };
    imports.wbg.__wbg_jgety_b4849ba1b0dd6d0d = function(arg0) {
        const ret = getObject(arg0).jgety();
        return ret;
    };
    imports.wbg.__wbg_jgetx_4dabd79bc7a48c04 = function(arg0) {
        const ret = getObject(arg0).jgetx();
        return ret;
    };
    imports.wbg.__wbg_getkey_b3f86c1cade92ad5 = function(arg0, arg1) {
        const ret = getObject(arg0).getkey(arg1);
        return ret;
    };
    imports.wbg.__wbg_getmrc_0adadfa40248ba08 = function(arg0) {
        const ret = getObject(arg0).getmrc();
        return ret;
    };
    imports.wbg.__wbg_getmmc_9b5764e5a815e2ad = function(arg0) {
        const ret = getObject(arg0).getmmc();
        return ret;
    };
    imports.wbg.__wbg_willrender_423628213aa1dba5 = function(arg0, arg1) {
        getObject(arg0).will_render(arg1 !== 0);
    };
    imports.wbg.__wbg_setubo_08d01444575adab9 = function(arg0, arg1) {
        getObject(arg0).set_ubo(getObject(arg1));
    };
    imports.wbg.__wbg_queuepipeline_07865c42b8351339 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).queuepipeline(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getStringFromWasm0(arg5, arg6), getStringFromWasm0(arg7, arg8), getStringFromWasm0(arg9, arg10));
    };
    imports.wbg.__wbg_new_5ef2b9d8412c2a28 = function(arg0, arg1, arg2, arg3) {
        const ret = new Gfxrender(getStringFromWasm0(arg0, arg1), arg2, arg3);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_c9fa56725105eef4 = function(arg0) {
        const ret = new Jsloop(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_e6f2ea7b5cb56489 = function() {
        const ret = new Jskeyboard();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_af8d702a13537953 = function() {
        const ret = new Jsmouse();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_6cd71009c6ef1ee2 = function() {
        const ret = new Jstouch();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_8ed14b7ba9d1c7bb = function() {
        const ret = new Jsgamepad();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_85f549f3b9f8199b = function() {
        const ret = new Jsaudioctx();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_drawloop_cf169dccb4959aab = function(arg0) {
        getObject(arg0).drawloop();
    };
    imports.wbg.__wbg_gfxsetrenderscale_122692e753f6b75d = function(arg0, arg1, arg2) {
        getObject(arg0).gfxsetrenderscale(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_gfxsetshadowmapres_d830bb091431b497 = function(arg0, arg1, arg2) {
        getObject(arg0).gfxsetshadowmapres(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_setrelxy_cab616f6b591c089 = function(arg0, arg1) {
        getObject(arg0).setrelxy(arg1);
    };
    imports.wbg.__wbg_setvolume_457f039720212f65 = function(arg0, arg1) {
        getObject(arg0).setvolume(arg1);
    };
    imports.wbg.__wbg_play_14cd1c9045eee566 = function(arg0) {
        getObject(arg0).play();
    };
    imports.wbg.__wbg_create_c731154c28ea12be = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24, arg25, arg26, arg27, arg28) {
        const ret = new Gfxmesh(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4), getObject(arg5), arg6 >>> 0, getStringFromWasm0(arg7, arg8), getStringFromWasm0(arg9, arg10), getStringFromWasm0(arg11, arg12), arg13, getStringFromWasm0(arg14, arg15), getStringFromWasm0(arg16, arg17), getStringFromWasm0(arg18, arg19), getStringFromWasm0(arg20, arg21), getStringFromWasm0(arg22, arg23), getStringFromWasm0(arg24, arg25), getStringFromWasm0(arg26, arg27), arg28 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_pushmesh_5e029c701a409612 = function(arg0, arg1, arg2) {
        getObject(arg0).push_mesh(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_snlll_5d78c2260bbbf995 = function(arg0, arg1) {
        snlll(getObject(arg0), arg1 >>> 0);
    };
    imports.wbg.__wbg_new_c8f3f39a49bab0d3 = function(arg0, arg1) {
        const ret = new Jsrelod(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getvert_661392fb24ffe079 = function(arg0) {
        const ret = getObject(arg0).getvert();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getuv_fca313568a679a03 = function(arg0) {
        const ret = getObject(arg0).getuv();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getnorm_b68fc351aa2c73f8 = function(arg0) {
        const ret = getObject(arg0).getnorm();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlen_1f09e3cd9d5c578a = function(arg0) {
        const ret = getObject(arg0).getlen();
        return ret;
    };
    imports.wbg.__wbg_new_d3fdf381211d561d = function(arg0, arg1) {
        const ret = new Jsloadsdf(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getmd_01dc42fa7c94ca3a = function(arg0) {
        const ret = getObject(arg0).getmd();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getmat_c71bd1615a0dd81d = function(arg0) {
        const ret = getObject(arg0).getmat();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlight_97838a1f8be8bdab = function(arg0) {
        const ret = getObject(arg0).getlight();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getspeaker_4cd130c19e14f7e9 = function(arg0) {
        const ret = getObject(arg0).getspeaker();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_buffer_12d079cc21e14bdb = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_4a659d079a1650e0 = function(arg0, arg1, arg2) {
        const ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_9efabd6b6d2ce46d = function(arg0) {
        const ret = new Float32Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_bd975934d1b1fddb = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_d25bbcbc3367f684 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_1e8b839a06de01c5 = function(arg0) {
        const ret = new Float32Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getindex_289b14432d9c89b9 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return ret;
    };
    imports.wbg.__wbg_setindex_e8a148aab2078037 = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = arg2;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper66 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 9, __wbg_adapter_10);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('schnellwerke_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
