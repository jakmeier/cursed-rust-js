/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_myrobodetection_free(a: number, b: number): void;
export function __wbg_event_free(a: number, b: number): void;
export function __wbg_get_event_timestamp(a: number): number;
export function __wbg_set_event_timestamp(a: number, b: number): void;
export function __wbg_get_event_x(a: number): number;
export function __wbg_set_event_x(a: number, b: number): void;
export function __wbg_get_event_y(a: number): number;
export function __wbg_set_event_y(a: number, b: number): void;
export function __wbg_robodetectionoutput_free(a: number, b: number): void;
export function __wbg_get_robodetectionoutput_jitter(a: number): number;
export function __wbg_set_robodetectionoutput_jitter(a: number, b: number): void;
export function __wbg_get_robodetectionoutput_humanScore(a: number): number;
export function __wbg_set_robodetectionoutput_humanScore(a: number, b: number): void;
export function myrobodetection_new(): number;
export function myrobodetection_fromEvents(a: number, b: number): number;
export function myrobodetection_addEvent(a: number, b: number, c: number, d: number): void;
export function myrobodetection_saveResult(a: number, b: number): void;
export function myrobodetection_saveBorrowedResult(a: number, b: number): void;
export function myrobodetection_events(a: number, b: number): void;
export function myrobodetection_results(a: number, b: number): void;
export function myrobodetection_isBot(a: number): number;
export function robodetectionoutput_text(a: number, b: number): void;
export function __wbindgen_export_0(a: number, b: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_export_1(a: number, b: number, c: number): void;
