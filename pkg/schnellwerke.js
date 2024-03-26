import { Jsaudio } from './snippets/schnellwerke-f53e00a8b9817494/src/engine/audio/audio.js';
import { Gfxrender, Gfxmesh, Gpucompute } from './snippets/schnellwerke-f53e00a8b9817494/src/engine/render/gfx.js';
import { Jsrelod } from './snippets/schnellwerke-f53e00a8b9817494/src/engine/resourceloader/resloader.js';

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

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

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
    imports.wbg.__wbg_create_0627302a46a41e3a = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24, arg25, arg26, arg27) {
        const ret = new Gfxmesh(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4), arg5, getStringFromWasm0(arg6, arg7), getStringFromWasm0(arg8, arg9), getStringFromWasm0(arg10, arg11), arg12, getStringFromWasm0(arg13, arg14), getStringFromWasm0(arg15, arg16), getStringFromWasm0(arg17, arg18), getStringFromWasm0(arg19, arg20), getStringFromWasm0(arg21, arg22), getStringFromWasm0(arg23, arg24), getStringFromWasm0(arg25, arg26), arg27 !== 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createcompute_597d2266badf09f4 = function(arg0, arg1, arg2, arg3) {
        const ret = new Gpucompute(arg0, arg1, getStringFromWasm0(arg2, arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_0965c81e343427d9 = function(arg0, arg1, arg2, arg3) {
        const ret = new Gfxrender(getStringFromWasm0(arg0, arg1), arg2, arg3);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_gfxendpass_d1d2106b7bb2c384 = function(arg0) {
        getObject(arg0).gfxendpass();
    };
    imports.wbg.__wbg_gfxbeginpass_8c7b78e5bea55959 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).gfxbeginpass(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_gfxfinishrender_110e9f2902e431f9 = function(arg0) {
        getObject(arg0).gfxfinishrender();
    };
    imports.wbg.__wbg_gfxgetexectime_071a71d97131712a = function(arg0) {
        const ret = getObject(arg0).gfxgetexectime();
        return ret;
    };
    imports.wbg.__wbg_new_5362ca6474d77ec7 = function(arg0, arg1) {
        const ret = new Jsrelod(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getvert_1a31c541b444fab8 = function(arg0) {
        const ret = getObject(arg0).getvert();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_getuv_37a3a23fdcf63157 = function(arg0) {
        const ret = getObject(arg0).getuv();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getnorm_e3aa6b4b4162424e = function(arg0) {
        const ret = getObject(arg0).getnorm();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getlen_7570575e75fc492a = function(arg0) {
        const ret = getObject(arg0).getlen();
        return ret;
    };
    imports.wbg.__wbg_new_6eb7306df2405ec1 = function(arg0, arg1) {
        const ret = new Jsaudio(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
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
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
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
