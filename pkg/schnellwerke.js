import { Jsaudiosource, Jsaudioctx } from './snippets/schnellwerke-ed63dda5e8b90d9a/src/engine/audio/audio.js';
import { Jskeyboard, Jsmouse, Jstouch, Jsgamepad } from './snippets/schnellwerke-ed63dda5e8b90d9a/src/engine/input/input.js';
import { Gfxrender, Gfxmesh, Jsloop, snlll } from './snippets/schnellwerke-ed63dda5e8b90d9a/src/engine/render/gfx.js';
import { Jsrelod, Jsloadsdf, get_text_iframe } from './snippets/schnellwerke-ed63dda5e8b90d9a/src/engine/resourceloader/resloader.js';

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
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1b9b59625f4c3dc3(arg0, arg1);
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
    imports.wbg.__wbg_log_317895bacf54b2c2 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_gettextiframe_d9c2b9f2a26573b3 = function(arg0, arg1) {
        const ret = get_text_iframe(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_dce8990958dc098e = function(arg0, arg1, arg2) {
        const ret = new Jsaudiosource(getObject(arg0), getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_jgetuse_b2367a2fbc7cd4cd = function(arg0) {
        const ret = getObject(arg0).jgetuse();
        return ret;
    };
    imports.wbg.__wbg_jgettx_a36ea2edacf2c7de = function(arg0) {
        const ret = getObject(arg0).jgettx();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizex_f04fe36b4cb8f74a = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizex();
        return ret;
    };
    imports.wbg.__wbg_jgetty_55d047a4b756a9bb = function(arg0) {
        const ret = getObject(arg0).jgetty();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizey_d4439ec9439134c5 = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizey();
        return ret;
    };
    imports.wbg.__wbg_jgety_ad535a94ab759a8f = function(arg0) {
        const ret = getObject(arg0).jgety();
        return ret;
    };
    imports.wbg.__wbg_jgetx_e839f46b3423674b = function(arg0) {
        const ret = getObject(arg0).jgetx();
        return ret;
    };
    imports.wbg.__wbg_getkey_e5428afc406f732e = function(arg0, arg1) {
        const ret = getObject(arg0).getkey(arg1);
        return ret;
    };
    imports.wbg.__wbg_getmrc_8146631d372fd51a = function(arg0) {
        const ret = getObject(arg0).getmrc();
        return ret;
    };
    imports.wbg.__wbg_getmmc_62f1c5a106f80686 = function(arg0) {
        const ret = getObject(arg0).getmmc();
        return ret;
    };
    imports.wbg.__wbg_willrender_f3b5cf0762b55a73 = function(arg0, arg1) {
        getObject(arg0).will_render(arg1 !== 0);
    };
    imports.wbg.__wbg_setubo_52def3822cd560c5 = function(arg0, arg1) {
        getObject(arg0).set_ubo(getObject(arg1));
    };
    imports.wbg.__wbg_queuepipeline_10fba820203e5ffd = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).queuepipeline(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getStringFromWasm0(arg5, arg6), getStringFromWasm0(arg7, arg8), getStringFromWasm0(arg9, arg10));
    };
    imports.wbg.__wbg_new_9c64988031ce378d = function(arg0, arg1, arg2, arg3) {
        const ret = new Gfxrender(getStringFromWasm0(arg0, arg1), arg2, arg3);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_d6c2c92dfc7893b6 = function(arg0) {
        const ret = new Jsloop(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_857c1f281e8a1af9 = function() {
        const ret = new Jskeyboard();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_f7edebfa2f59b5fd = function() {
        const ret = new Jsmouse();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3373bc622a4b5b66 = function() {
        const ret = new Jstouch();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_836307f2f8909e62 = function() {
        const ret = new Jsgamepad();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_2665722fa7eb83ad = function() {
        const ret = new Jsaudioctx();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_drawloop_2b149d4be94d54fd = function(arg0) {
        getObject(arg0).drawloop();
    };
    imports.wbg.__wbg_gfxsetrenderscale_b8e9b2e9c6279ed6 = function(arg0, arg1, arg2) {
        getObject(arg0).gfxsetrenderscale(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_gfxsetshadowmapres_9c39b6ea5100a8d8 = function(arg0, arg1, arg2) {
        getObject(arg0).gfxsetshadowmapres(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_setrelxy_3bb0f57e0d9122e3 = function(arg0, arg1) {
        getObject(arg0).setrelxy(arg1);
    };
    imports.wbg.__wbg_setvolume_85e664b1bb30318b = function(arg0, arg1) {
        getObject(arg0).setvolume(arg1);
    };
    imports.wbg.__wbg_play_da923e35f933d64f = function(arg0) {
        getObject(arg0).play();
    };
    imports.wbg.__wbg_create_0b9fe6d5be161aee = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24, arg25, arg26, arg27, arg28) {
        const ret = new Gfxmesh(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4), getObject(arg5), arg6 >>> 0, getStringFromWasm0(arg7, arg8), getStringFromWasm0(arg9, arg10), getStringFromWasm0(arg11, arg12), arg13, getStringFromWasm0(arg14, arg15), getStringFromWasm0(arg16, arg17), getStringFromWasm0(arg18, arg19), getStringFromWasm0(arg20, arg21), getStringFromWasm0(arg22, arg23), getStringFromWasm0(arg24, arg25), getStringFromWasm0(arg26, arg27), arg28 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_pushmesh_d57554b8216cec15 = function(arg0, arg1, arg2) {
        getObject(arg0).push_mesh(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_snlll_ec2e0ecc8bf72e86 = function(arg0, arg1) {
        snlll(getObject(arg0), arg1 >>> 0);
    };
    imports.wbg.__wbg_new_766710a7adac36ef = function(arg0, arg1) {
        const ret = new Jsrelod(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getvert_9500836d62f83059 = function(arg0) {
        const ret = getObject(arg0).getvert();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getuv_09af66668a50310d = function(arg0) {
        const ret = getObject(arg0).getuv();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getnorm_a7000ee84d120079 = function(arg0) {
        const ret = getObject(arg0).getnorm();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlen_70e067c36215dcd7 = function(arg0) {
        const ret = getObject(arg0).getlen();
        return ret;
    };
    imports.wbg.__wbg_new_7ec2b988e8a69200 = function(arg0, arg1) {
        const ret = new Jsloadsdf(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getmd_ff6739cbd153ec31 = function(arg0) {
        const ret = getObject(arg0).getmd();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getmat_4e41d8baa3daf857 = function(arg0) {
        const ret = getObject(arg0).getmat();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlight_50043bafea0602b3 = function(arg0) {
        const ret = getObject(arg0).getlight();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getspeaker_ab9154f7d40e4acc = function(arg0) {
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
