/* tslint:disable */
/* eslint-disable */
/**
* @param {Float64Array} in_dist 
* @param {Float64Array} p_merge 
*/
export function set_input(in_dist: Float64Array, p_merge: Float64Array): void;
/**
* @returns {Float64Array} 
*/
export function get_in_dist(): Float64Array;
/**
* @returns {Float64Array} 
*/
export function get_p_merge(): Float64Array;
/**
* @returns {Float64Array} 
*/
export function get_total_chance(): Float64Array;
/**
* @returns {Float64Array} 
*/
export function get_total_chance_no_ones(): Float64Array;
/**
* @returns {Float64Array} 
*/
export function get_output(): Float64Array;
/**
* @returns {Float64Array} 
*/
export function get_remain(): Float64Array;
/**
* @returns {Float64Array} 
*/
export function get_missing(): Float64Array;
/**
*/
export function set_panic_hook(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly set_input: (a: number, b: number, c: number, d: number) => void;
  readonly get_in_dist: (a: number) => void;
  readonly get_p_merge: (a: number) => void;
  readonly get_total_chance: (a: number) => void;
  readonly get_total_chance_no_ones: (a: number) => void;
  readonly get_output: (a: number) => void;
  readonly get_remain: (a: number) => void;
  readonly get_missing: (a: number) => void;
  readonly set_panic_hook: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        