/* @ts-self-types="./schnellwerke.d.ts" */
import { cend, mozartsetvolume, newsound, soundplay, soundsetloop, soundsetpos, soundstop } from './snippets/schnellwerke-30ed2f676ded1e7a/src/engine/audio/audio.js';
import { openfs } from './snippets/schnellwerke-30ed2f676ded1e7a/src/engine/loader/loader.js';
import { gamepad_axisnm, gamepad_buttonnm, getKeyPressed, get_axis, get_button, get_frametime, get_mouse_posx, get_mouse_posy, get_resx, get_resy, modifydeffereddata, modifydeffereduniform, modifyshadowdata, modifyshadowuniform, neweng, newmaterial, newmesh, newtexture, quitfullscreen, req_mouse_lock, req_mouse_unlock, rn, setdrawable, setfullscreen, setmeshbuf, setrendercamera, settitle } from './snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js';
import { get_val, set_val } from './snippets/schnellwerke-30ed2f676ded1e7a/src/loop_tick/sav.js';
import * as import1 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import2 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import3 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import4 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import5 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import6 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import7 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js"
import * as import8 from "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/audio/audio.js"


export function main() {
    wasm.main();
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_debug_string_8a447059637473e2: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_acc5528be2b923f2: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_undefined_721f8decd50c87a3: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_throw_ea4887a5f8f9a9db: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_33c39e13d73b25f6: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_cend_73da46e2da7fc397: function(arg0) {
            const ret = cend(arg0 >>> 0);
            return ret;
        },
        __wbg_gamepad_axisnm_a839315269a1c241: function(arg0) {
            const ret = gamepad_axisnm(arg0 >>> 0);
            return ret;
        },
        __wbg_gamepad_buttonnm_a3a2106eb6cf0169: function(arg0) {
            const ret = gamepad_buttonnm(arg0 >>> 0);
            return ret;
        },
        __wbg_getKeyPressed_01d636082838a263: function(arg0) {
            const ret = getKeyPressed(arg0 >>> 0);
            return ret;
        },
        __wbg_get_axis_3de7c8209bffab26: function(arg0, arg1) {
            const ret = get_axis(arg0 >>> 0, arg1 >>> 0);
            return ret;
        },
        __wbg_get_button_8fa328cf3a29c0bb: function(arg0, arg1) {
            const ret = get_button(arg0 >>> 0, arg1 >>> 0);
            return ret;
        },
        __wbg_get_frametime_86e299583da96f16: function(arg0) {
            const ret = get_frametime(arg0 >>> 0);
            return ret;
        },
        __wbg_get_mouse_posx_bb5671eb50d3cd95: function(arg0) {
            const ret = get_mouse_posx(arg0 >>> 0);
            return ret;
        },
        __wbg_get_mouse_posy_9a1e8ef7bebc9329: function(arg0) {
            const ret = get_mouse_posy(arg0 >>> 0);
            return ret;
        },
        __wbg_get_resx_0388b3a77a6b41c7: function(arg0) {
            const ret = get_resx(arg0 >>> 0);
            return ret;
        },
        __wbg_get_resy_b85e676d62790ba1: function(arg0) {
            const ret = get_resy(arg0 >>> 0);
            return ret;
        },
        __wbg_get_val_0641b702b4d2babb: function(arg0, arg1, arg2) {
            const ret = get_val(getStringFromWasm0(arg1, arg2));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_length_589238bdcf171f0e: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_e6bdba0734e5c7f5: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_log_7969bf8351d8aa28: function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        },
        __wbg_modifydeffereddata_a1b180d9d73eb1e3: function(arg0, arg1, arg2) {
            modifydeffereddata(arg0 >>> 0, arg1 >>> 0, arg2);
        },
        __wbg_modifydeffereduniform_e67c6196de18ddcc: function(arg0, arg1, arg2) {
            modifydeffereduniform(arg0 >>> 0, arg1 >>> 0, arg2);
        },
        __wbg_modifyshadowdata_9fa6f2b5fbed3c31: function(arg0, arg1, arg2, arg3) {
            modifyshadowdata(arg0 >>> 0, arg1 >>> 0, arg2 >>> 0, arg3 >>> 0);
        },
        __wbg_modifyshadowuniform_54297261a8343a9e: function(arg0, arg1, arg2) {
            modifyshadowuniform(arg0 >>> 0, arg1 >>> 0, arg2);
        },
        __wbg_mozartsetvolume_b1f8bb66c2d2dc70: function(arg0, arg1) {
            mozartsetvolume(arg0 >>> 0, arg1);
        },
        __wbg_new_with_length_711111a7d776e50c: function(arg0) {
            const ret = new Float32Array(arg0 >>> 0);
            return ret;
        },
        __wbg_new_with_length_9b650f44b5c44a4e: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_neweng_e0131cf2d4f35170: function(arg0, arg1) {
            const ret = neweng(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_newmaterial_376fbe76899291a6: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
            const ret = newmaterial(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), arg6 >>> 0, arg7 >>> 0);
            return ret;
        },
        __wbg_newmesh_7df087f4cb44d0a0: function(arg0, arg1, arg2, arg3, arg4) {
            const ret = newmesh(arg0 >>> 0, arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            return ret;
        },
        __wbg_newsound_6c94340c9fe0ddda: function(arg0, arg1, arg2) {
            const ret = newsound(arg0 >>> 0, getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_newtexture_99c59dcfcc40db89: function(arg0, arg1, arg2, arg3) {
            const ret = newtexture(arg0 >>> 0, arg1 >>> 0, arg2 >>> 0, arg3);
            return ret;
        },
        __wbg_openfs_69d414451c003993: function(arg0, arg1) {
            const ret = openfs(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_prototypesetcall_d721637c7ca66eb8: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_queueMicrotask_1c9b3800e321a967: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_queueMicrotask_311744e534a929a3: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_quitfullscreen_fcc0fa817ebb271f: function(arg0) {
            quitfullscreen(arg0 >>> 0);
        },
        __wbg_req_mouse_lock_85aace2893a80226: function(arg0) {
            req_mouse_lock(arg0 >>> 0);
        },
        __wbg_req_mouse_unlock_1c16941f9f30dbb1: function(arg0) {
            req_mouse_unlock(arg0 >>> 0);
        },
        __wbg_resolve_d82363d90af6928a: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_rn_52e21c9736db47c7: function(arg0) {
            rn(arg0 >>> 0);
        },
        __wbg_set_0bf1fca872bc6d18: function(arg0, arg1, arg2) {
            arg0.set(getArrayU8FromWasm0(arg1, arg2));
        },
        __wbg_set_6f5ddc74972bd54e: function(arg0, arg1, arg2) {
            arg0.set(getArrayF32FromWasm0(arg1, arg2));
        },
        __wbg_set_val_8654864df7388514: function(arg0, arg1, arg2, arg3) {
            set_val(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        },
        __wbg_setdrawable_84872f53f940be57: function(arg0, arg1) {
            setdrawable(arg0 >>> 0, arg1);
        },
        __wbg_setfullscreen_348762ae3c491158: function(arg0) {
            setfullscreen(arg0 >>> 0);
        },
        __wbg_setmeshbuf_ea5375e9a7fd0f93: function(arg0, arg1, arg2) {
            setmeshbuf(arg0 >>> 0, arg1 >>> 0, arg2);
        },
        __wbg_setrendercamera_97fea8d7f28b0ef8: function(arg0, arg1) {
            setrendercamera(arg0 >>> 0, arg1);
        },
        __wbg_settitle_c24ea866cfaef2fa: function(arg0, arg1) {
            settitle(getStringFromWasm0(arg0, arg1));
        },
        __wbg_soundplay_181cd1e07a377d66: function(arg0, arg1, arg2) {
            soundplay(arg0 >>> 0, arg1, arg2);
        },
        __wbg_soundsetloop_ebb69f055de190f0: function(arg0, arg1) {
            soundsetloop(arg0 >>> 0, arg1 !== 0);
        },
        __wbg_soundsetpos_8cef18065b47ee78: function(arg0, arg1) {
            soundsetpos(arg0 >>> 0, arg1);
        },
        __wbg_soundstop_9b3d033781aa35f8: function(arg0) {
            soundstop(arg0 >>> 0);
        },
        __wbg_static_accessor_GLOBAL_THIS_2fee5048bcca5938: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_ce44e66a4935da8c: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_44f6e0cb5e67cdad: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_168f178805d978fe: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_then_05edfc8a4fea5106: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_then_591b6b3a75ee817a: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 777, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h78276179a3dd62be);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 286, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h11019ce1df08ba1c);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./schnellwerke_bg.js": import0,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import1,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import2,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import3,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import4,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import5,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import6,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/render/gauss.js": import7,
        "./snippets/schnellwerke-30ed2f676ded1e7a/src/engine/audio/audio.js": import8,
    };
}

function wasm_bindgen__convert__closures_____invoke__h11019ce1df08ba1c(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h11019ce1df08ba1c(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h78276179a3dd62be(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__h78276179a3dd62be(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

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
    if (builtInMatches && builtInMatches.length > 1) {
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

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
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
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('schnellwerke_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
