import { Jsaudiosource, Jsaudioctx } from './snippets/schnellwerke-7d12bd7ef09e858d/src/engine/audio/audio.js';
import { Jskeyboard, Jsmouse, Jstouch, Jsgamepad } from './snippets/schnellwerke-7d12bd7ef09e858d/src/engine/input/input.js';
import { Gfxrender, Gfxmesh, Jsloop, snlll } from './snippets/schnellwerke-7d12bd7ef09e858d/src/engine/render/gfx.js';
import { Jsrelod, Jsloadsdf, get_text_iframe } from './snippets/schnellwerke-7d12bd7ef09e858d/src/engine/resourceloader/resloader.js';

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
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8dc6586a1d9c2c05(arg0, arg1);
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
    imports.wbg.__wbg_log_fad38ffe6d8d7d0a = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_gettextiframe_b14ee82d9ca27fff = function(arg0, arg1) {
        const ret = get_text_iframe(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_11adb938c78cb567 = function(arg0, arg1, arg2) {
        const ret = new Jsaudiosource(getObject(arg0), getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_jgetuse_50bf11999c22c78a = function(arg0) {
        const ret = getObject(arg0).jgetuse();
        return ret;
    };
    imports.wbg.__wbg_jgettx_3a87bef0f9ea80ee = function(arg0) {
        const ret = getObject(arg0).jgettx();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizex_0e310e284b4e73e1 = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizex();
        return ret;
    };
    imports.wbg.__wbg_jgetty_0708dac3bfbb5a24 = function(arg0) {
        const ret = getObject(arg0).jgetty();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizey_1d80c8cc2c41994c = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizey();
        return ret;
    };
    imports.wbg.__wbg_jgety_a2f70f5cb4ff299a = function(arg0) {
        const ret = getObject(arg0).jgety();
        return ret;
    };
    imports.wbg.__wbg_jgetx_8a6e78abb74fab01 = function(arg0) {
        const ret = getObject(arg0).jgetx();
        return ret;
    };
    imports.wbg.__wbg_getkey_1c913f78145f1ba4 = function(arg0, arg1) {
        const ret = getObject(arg0).getkey(arg1);
        return ret;
    };
    imports.wbg.__wbg_getmrc_0758a73ed44f23ba = function(arg0) {
        const ret = getObject(arg0).getmrc();
        return ret;
    };
    imports.wbg.__wbg_getmmc_28b485ce3e7272e7 = function(arg0) {
        const ret = getObject(arg0).getmmc();
        return ret;
    };
    imports.wbg.__wbg_willrender_c5573944bef2f3a7 = function(arg0, arg1) {
        getObject(arg0).will_render(arg1 !== 0);
    };
    imports.wbg.__wbg_setubo_3abead0484481228 = function(arg0, arg1) {
        getObject(arg0).set_ubo(getObject(arg1));
    };
    imports.wbg.__wbg_queuepipeline_fc167171f2707196 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).queuepipeline(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getStringFromWasm0(arg5, arg6), getStringFromWasm0(arg7, arg8), getStringFromWasm0(arg9, arg10));
    };
    imports.wbg.__wbg_new_c605eb13ca52206a = function(arg0, arg1, arg2, arg3) {
        const ret = new Gfxrender(getStringFromWasm0(arg0, arg1), arg2, arg3);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3e63a8270c870e71 = function(arg0) {
        const ret = new Jsloop(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_483697bb6348c815 = function() {
        const ret = new Jskeyboard();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_c988dbc775a1bdd7 = function() {
        const ret = new Jsmouse();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_2361fb1ce79d09b7 = function() {
        const ret = new Jstouch();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_14046d3b1f5120fe = function() {
        const ret = new Jsgamepad();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_5bbe347c13b8ae34 = function() {
        const ret = new Jsaudioctx();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_drawloop_59b637166abe1b6a = function(arg0) {
        getObject(arg0).drawloop();
    };
    imports.wbg.__wbg_gfxsetrenderscale_88ed085be5a5f9cc = function(arg0, arg1, arg2) {
        getObject(arg0).gfxsetrenderscale(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_gfxsetshadowmapres_9165941d066b0ca2 = function(arg0, arg1, arg2) {
        getObject(arg0).gfxsetshadowmapres(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_setrelxy_ca870ed3a7a99f99 = function(arg0, arg1) {
        getObject(arg0).setrelxy(arg1);
    };
    imports.wbg.__wbg_setvolume_c34bbbb7473bf848 = function(arg0, arg1) {
        getObject(arg0).setvolume(arg1);
    };
    imports.wbg.__wbg_play_bb284d22e26a2411 = function(arg0) {
        getObject(arg0).play();
    };
    imports.wbg.__wbg_create_28dded1652d8b013 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24, arg25, arg26, arg27, arg28) {
        const ret = new Gfxmesh(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4), getObject(arg5), arg6 >>> 0, getStringFromWasm0(arg7, arg8), getStringFromWasm0(arg9, arg10), getStringFromWasm0(arg11, arg12), arg13, getStringFromWasm0(arg14, arg15), getStringFromWasm0(arg16, arg17), getStringFromWasm0(arg18, arg19), getStringFromWasm0(arg20, arg21), getStringFromWasm0(arg22, arg23), getStringFromWasm0(arg24, arg25), getStringFromWasm0(arg26, arg27), arg28 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_pushmesh_95ea16fcf0dee3e2 = function(arg0, arg1, arg2) {
        getObject(arg0).push_mesh(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_snlll_57aff4f717936dc1 = function(arg0, arg1) {
        snlll(getObject(arg0), arg1 >>> 0);
    };
    imports.wbg.__wbg_new_52b83b453b797fd4 = function(arg0, arg1) {
        const ret = new Jsrelod(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getvert_44b609cc83a60e6e = function(arg0) {
        const ret = getObject(arg0).getvert();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getuv_612fe6b4eb30fc58 = function(arg0) {
        const ret = getObject(arg0).getuv();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getnorm_51768ba2ce7c20f3 = function(arg0) {
        const ret = getObject(arg0).getnorm();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlen_220bcc266cebadd8 = function(arg0) {
        const ret = getObject(arg0).getlen();
        return ret;
    };
    imports.wbg.__wbg_new_e9fddbc35543bca5 = function(arg0, arg1) {
        const ret = new Jsloadsdf(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getmd_06bc39eab7bf1eb9 = function(arg0) {
        const ret = getObject(arg0).getmd();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getmat_e33de8982b2767b4 = function(arg0) {
        const ret = getObject(arg0).getmat();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlight_cfba1bff995c21de = function(arg0) {
        const ret = getObject(arg0).getlight();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getspeaker_0ebffb34fa3cdd5a = function(arg0) {
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
