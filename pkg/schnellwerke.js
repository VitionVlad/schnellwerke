import { Jsaudio } from './snippets/schnellwerke-e759e1fb4adfa39e/src/engine/audio/audio.js';
import { Gfxrender, Gfxmesh, Gpucompute } from './snippets/schnellwerke-e759e1fb4adfa39e/src/engine/render/gfx.js';
import { Jsrelod } from './snippets/schnellwerke-e759e1fb4adfa39e/src/engine/resourceloader/resloader.js';
import * as __wbg_star0 from './snippets/schnellwerke-e759e1fb4adfa39e/src/engine/input/keyboard.js';
import * as __wbg_star1 from './snippets/schnellwerke-e759e1fb4adfa39e/src/engine/input/mouse.js';
import * as __wbg_star2 from './snippets/schnellwerke-e759e1fb4adfa39e/src/engine/input/touch.js';

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
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
function __wbg_adapter_16(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h26d9e3d64390110b(arg0, arg1);
}

/**
*/
export function main() {
    wasm.main();
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
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
    imports.wbg.__wbg_create_f873514de99ec5fb = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24, arg25, arg26, arg27) {
        const ret = new Gfxmesh(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4), arg5, getStringFromWasm0(arg6, arg7), getStringFromWasm0(arg8, arg9), getStringFromWasm0(arg10, arg11), arg12, getStringFromWasm0(arg13, arg14), getStringFromWasm0(arg15, arg16), getStringFromWasm0(arg17, arg18), getStringFromWasm0(arg19, arg20), getStringFromWasm0(arg21, arg22), getStringFromWasm0(arg23, arg24), getStringFromWasm0(arg25, arg26), arg27 !== 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createcompute_1a36a44654b64c41 = function(arg0, arg1, arg2, arg3) {
        const ret = new Gpucompute(arg0, arg1, getStringFromWasm0(arg2, arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_gfxgetcanvassizex_31ac7aa4985b79bf = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizex();
        return ret;
    };
    imports.wbg.__wbg_gfxgetcanvassizey_54757510270b9052 = function(arg0) {
        const ret = getObject(arg0).gfxgetcanvassizey();
        return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_draw_bd517220103f8d8b = function(arg0, arg1, arg2) {
        getObject(arg0).draw(getObject(arg1), takeObject(arg2));
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_setvolume_7f21eccdd9cb28f9 = function(arg0, arg1) {
        getObject(arg0).setvolume(arg1);
    };
    imports.wbg.__wbg_play_3e42ef0ae142ae4c = function(arg0) {
        getObject(arg0).play();
    };
    imports.wbg.__wbg_gfxendpass_e3a35797f3160ceb = function(arg0) {
        getObject(arg0).gfxendpass();
    };
    imports.wbg.__wbg_gfxbeginpass_2b29c2f95af0c3f8 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).gfxbeginpass(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_new_943b55775f2795a5 = function(arg0, arg1, arg2, arg3) {
        const ret = new Gfxrender(getStringFromWasm0(arg0, arg1), arg2, arg3);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_gfxsetrenderscale_51e8f91a849757b5 = function(arg0, arg1) {
        getObject(arg0).gfxsetrenderscale(arg1);
    };
    imports.wbg.__wbg_gfxsetshadowmapres_61a9fff32f748324 = function(arg0, arg1) {
        getObject(arg0).gfxsetshadowmapres(arg1);
    };
    imports.wbg.__wbg_gfxbeginshadowpass_35c183267a3563df = function(arg0, arg1, arg2) {
        getObject(arg0).gfxbeginshadowpass(getStringFromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_gfxbeginmainpass_5d38487a9a6e7dce = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).gfxbeginmainpass(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_gfxfinishrender_099b41adb9135d44 = function(arg0) {
        getObject(arg0).gfxfinishrender();
    };
    imports.wbg.__wbg_gfxgetexectime_03b0cc6689db0c90 = function(arg0) {
        const ret = getObject(arg0).gfxgetexectime();
        return ret;
    };
    imports.wbg.__wbg_execute_3897141733aeb5f2 = function(arg0, arg1, arg2) {
        getObject(arg0).execute(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_getstate_77e69f7d26059bf6 = function(arg0) {
        const ret = getObject(arg0).getstate();
        return ret;
    };
    imports.wbg.__wbg_getresult_610848d13deba563 = function(arg0) {
        const ret = getObject(arg0).getresult();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3605d442df340829 = function(arg0, arg1) {
        const ret = new Jsaudio(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3c8c35e22f4f2a88 = function(arg0, arg1) {
        const ret = new Jsrelod(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getvert_2c4ca0a81cddd564 = function(arg0) {
        const ret = getObject(arg0).getvert();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getuv_ce3406d9c3a4050a = function(arg0) {
        const ret = getObject(arg0).getuv();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getnorm_567ef3c0d7630d3c = function(arg0) {
        const ret = getObject(arg0).getnorm();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlen_53911ceff327ea9a = function(arg0) {
        const ret = getObject(arg0).getlen();
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_cee7a886d55e7df5 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_requestAnimationFrame_fdbeaff9e8f3f77d = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newnoargs_cfecb3965268594c = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_3f093dd26d5569f8 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_05040bd9523805b9 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_adc720039f2cb14f = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_622105db80c1457d = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_f56b013ed9bcf359 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_buffer_b914fb8b50ebbc3e = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_5fd0a60d38f47fa6 = function(arg0, arg1, arg2) {
        const ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_d5be849e30054b65 = function(arg0) {
        const ret = new Float32Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_c07e61b3625bced8 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_f2871b6ecd8bc3e4 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_fa1ac68ba10e0036 = function(arg0) {
        const ret = new Float32Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setindex_0bd00d2b6ce7b82b = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = arg2;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper67 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 3, __wbg_adapter_16);
        return addHeapObject(ret);
    };
    imports['./snippets/schnellwerke-e759e1fb4adfa39e/src/engine/input/keyboard.js'] = __wbg_star0;
    imports['./snippets/schnellwerke-e759e1fb4adfa39e/src/engine/input/mouse.js'] = __wbg_star1;
    imports['./snippets/schnellwerke-e759e1fb4adfa39e/src/engine/input/touch.js'] = __wbg_star2;

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
