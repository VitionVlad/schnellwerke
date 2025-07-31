import { mozartsetvolume, newsound, soundplay, soudstop } from './snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/audio/audio.js';
import { openfs } from './snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/loader/loader.js';
import { get_frametime, get_resx, get_resy, settitle, setfullscreen, quitfullscreen, getKeyPressed, req_mouse_lock, req_mouse_unlock, modifyshadowdata, modifydeffereddata, modifyshadowuniform, modifydeffereduniform, neweng, newmaterial, newmodel, setrendercamera, setmeshbuf, setdrawable, newmesh, newtexture, rn, renderloop } from './snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/render/gauss.js';
import { set_val, get_val } from './snippets/schnellwerke-0ccc4b36a7b18b32/src/sav.js';
import * as __wbg_star0 from './snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/audio/audio.js';
import * as __wbg_star1 from './snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/render/gauss.js';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

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

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

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

    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);

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
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
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

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}
function __wbg_adapter_18(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3b2e24d61b3e11a4(arg0, arg1);
}

function __wbg_adapter_21(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7e9d88d1202fb139(arg0, arg1, addHeapObject(arg2));
}

/**
* @returns {Promise<void>}
*/
export function main() {
    const ret = wasm.main();
    return takeObject(ret);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
function __wbg_adapter_109(arg0, arg1, arg2, arg3) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h9245587f06e70754(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
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
    imports.wbg.__wbg_openfs_0c1911196b2a1ace = function() { return logError(function (arg0, arg1) {
        const ret = openfs(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getframetime_07379ade7378a415 = function() { return logError(function (arg0) {
        const ret = get_frametime(arg0 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getresx_e1b7fcf54caeac2c = function() { return logError(function (arg0) {
        const ret = get_resx(arg0 >>> 0);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getresy_799654bbf03cd505 = function() { return logError(function (arg0) {
        const ret = get_resy(arg0 >>> 0);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_settitle_13b229688ffc8a09 = function() { return logError(function (arg0, arg1) {
        settitle(getStringFromWasm0(arg0, arg1));
    }, arguments) };
    imports.wbg.__wbg_setfullscreen_2e33c70217e46eee = function() { return logError(function (arg0) {
        setfullscreen(arg0 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_quitfullscreen_7ba740b88ab86b0d = function() { return logError(function (arg0) {
        quitfullscreen(arg0 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_getKeyPressed_c1aa64b15cb0cd65 = function() { return logError(function (arg0) {
        const ret = getKeyPressed(arg0 >>> 0);
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_reqmouselock_82b736a8e25eab31 = function() { return logError(function (arg0) {
        req_mouse_lock(arg0 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_reqmouseunlock_542d8bfdd039ef53 = function() { return logError(function (arg0) {
        req_mouse_unlock(arg0 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_modifyshadowdata_815d6684de037cfb = function() { return logError(function (arg0, arg1, arg2) {
        modifyshadowdata(arg0 >>> 0, arg1 >>> 0, arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_modifydeffereddata_9d461f559615f05c = function() { return logError(function (arg0, arg1, arg2) {
        modifydeffereddata(arg0 >>> 0, arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_modifyshadowuniform_a850d5376bbbda67 = function() { return logError(function (arg0, arg1, arg2) {
        modifyshadowuniform(arg0 >>> 0, arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_modifydeffereduniform_d7364ff32d37e181 = function() { return logError(function (arg0, arg1, arg2) {
        modifydeffereduniform(arg0 >>> 0, arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_neweng_e295352dab9d02cc = function() { return logError(function (arg0, arg1) {
        const ret = neweng(getStringFromWasm0(arg0, arg1));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newmaterial_d37145a6ca99be69 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        const ret = newmaterial(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), arg6 >>> 0, arg7 >>> 0);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newmodel_6b562fff1e48340b = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = newmodel(takeObject(arg0), takeObject(arg1), takeObject(arg2), takeObject(arg3), takeObject(arg4));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setrendercamera_9ab89d5434963abc = function() { return logError(function (arg0, arg1) {
        setrendercamera(arg0 >>> 0, arg1);
    }, arguments) };
    imports.wbg.__wbg_setmeshbuf_3af491f3507f7a42 = function() { return logError(function (arg0, arg1, arg2) {
        setmeshbuf(arg0 >>> 0, arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_setdrawable_ba80c665f8960f82 = function() { return logError(function (arg0, arg1) {
        setdrawable(arg0 >>> 0, arg1);
    }, arguments) };
    imports.wbg.__wbg_newmesh_2d59a5b19b057cbe = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = newmesh(arg0 >>> 0, arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newtexture_53fe3741ead02e0b = function() { return logError(function (arg0, arg1, arg2, arg3) {
        const ret = newtexture(arg0 >>> 0, arg1 >>> 0, arg2 >>> 0, takeObject(arg3));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_rn_095d983fb5ab67a9 = function() { return logError(function (arg0) {
        rn(arg0 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_renderloop_aae06e968c7a40db = function() { return logError(function (arg0) {
        renderloop(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_setval_aba0ae4e743b5275 = function() { return logError(function (arg0, arg1, arg2) {
        set_val(getStringFromWasm0(arg0, arg1), arg2);
    }, arguments) };
    imports.wbg.__wbg_getval_2cdee7be98ae893d = function() { return logError(function (arg0, arg1) {
        const ret = get_val(getStringFromWasm0(arg0, arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_mozartsetvolume_ff3fc81d3cd8e35b = function() { return logError(function (arg0, arg1) {
        mozartsetvolume(arg0 >>> 0, arg1);
    }, arguments) };
    imports.wbg.__wbg_newsound_9dff7ed5c44a3df3 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = newsound(arg0 >>> 0, getStringFromWasm0(arg1, arg2));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_soundplay_80862b3e5642a414 = function() { return logError(function (arg0, arg1, arg2) {
        soundplay(arg0 >>> 0, arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_soudstop_bcbe174cd14c3a60 = function() { return logError(function (arg0) {
        soudstop(arg0 >>> 0);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_queueMicrotask_481971b0d87f3dd4 = function() { return logError(function (arg0) {
        queueMicrotask(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_e258087cd0daa0ea = function() { return logError(function (arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_27c0f87801dedf93 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_b3ca7c6051f9bec1 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_81740750da40724f = function() { return logError(function (arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_109(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    }, arguments) };
    imports.wbg.__wbg_resolve_b0083a7967828ec8 = function() { return logError(function (arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_0c86a60e8fcfe9f6 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_a73caa9a87991566 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_d1e6af4856ba331b = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_ce0dbfc45cf2f5be = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_c6fb939a7f436783 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_207b558942527489 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_63b92bc8671ed464 = function() { return logError(function (arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithlength_e9b4878cebadb3d3 = function() { return logError(function (arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb = function() { return logError(function (arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_c20a40f15020d68a = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_a47bac70306a19a7 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_newwithlength_1e8b839a06de01c5 = function() { return logError(function (arg0) {
        const ret = new Float32Array(arg0 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithbyteoffsetandlength_4a659d079a1650e0 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_d25bbcbc3367f684 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_bd975934d1b1fddb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_12d079cc21e14bdb = function() { return logError(function (arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper657 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 3, __wbg_adapter_18);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper683 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 37, __wbg_adapter_21);
        return addHeapObject(ret);
    }, arguments) };
    imports['./snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/audio/audio.js'] = __wbg_star0;
    imports['./snippets/schnellwerke-0ccc4b36a7b18b32/src/engine/render/gauss.js'] = __wbg_star1;

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
