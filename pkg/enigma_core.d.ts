/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
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
export function enigma_encrypt_with_settings_inline(rotor1_map: string, rotor2_map: string, rotor3_map: string, rotor1_key: bigint, rotor2_key: bigint, rotor3_key: bigint, pair1_left: bigint, pair1_right: bigint, pair2_left: bigint, pair2_right: bigint, pair3_left: bigint, pair3_right: bigint, raw: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly enigma_encrypt_with_settings_inline: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number) => void;
  readonly greet: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
