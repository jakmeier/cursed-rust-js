/* tslint:disable */
/* eslint-disable */
export class BotDetectionOutput {
  free(): void;
  /**
   * @returns {string}
   */
  text(): string;
  humanScore: number;
  timestamp: number;
}
export class Coordinate {
  free(): void;
  /**
   * @param {number} x
   * @param {number} y
   */
  constructor(x: number, y: number);
  x: number;
  y: number;
}
export class Event {
  free(): void;
  coordinate: Coordinate;
  timestamp: number;
}
export class MyBotDetection {
  free(): void;
  constructor();
  /**
   * @param {(Event)[]} events
   * @returns {MyBotDetection}
   */
  static fromEvents(events: (Event)[]): MyBotDetection;
  /**
   * @param {number} timestamp
   * @param {MouseEvent} event
   */
  addEvent(timestamp: number, event: MouseEvent): void;
  /**
   * @param {BotDetectionOutput} result
   */
  saveResult(result: BotDetectionOutput): void;
  /**
   * @param {BotDetectionOutput} result
   */
  saveBorrowedResult(result: BotDetectionOutput): void;
  /**
   * @returns {(Event)[]}
   */
  allEvents(): (Event)[];
  /**
   * @param {number} start
   * @param {number} end
   * @returns {(Event)[]}
   */
  events(start: number, end: number): (Event)[];
  /**
   * @returns {number}
   */
  num_events(): number;
  /**
   * @returns {BotDetectionOutput}
   */
  isBot(): BotDetectionOutput;
  readonly results: (BotDetectionOutput)[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mybotdetection_free: (a: number, b: number) => void;
  readonly __wbg_event_free: (a: number, b: number) => void;
  readonly __wbg_get_event_coordinate: (a: number) => number;
  readonly __wbg_set_event_coordinate: (a: number, b: number) => void;
  readonly __wbg_coordinate_free: (a: number, b: number) => void;
  readonly __wbg_get_coordinate_x: (a: number) => number;
  readonly __wbg_set_coordinate_x: (a: number, b: number) => void;
  readonly __wbg_get_coordinate_y: (a: number) => number;
  readonly __wbg_set_coordinate_y: (a: number, b: number) => void;
  readonly __wbg_botdetectionoutput_free: (a: number, b: number) => void;
  readonly __wbg_get_botdetectionoutput_humanScore: (a: number) => number;
  readonly __wbg_set_botdetectionoutput_humanScore: (a: number, b: number) => void;
  readonly __wbg_get_botdetectionoutput_timestamp: (a: number) => number;
  readonly __wbg_set_botdetectionoutput_timestamp: (a: number, b: number) => void;
  readonly mybotdetection_new: () => number;
  readonly mybotdetection_fromEvents: (a: number, b: number) => number;
  readonly mybotdetection_addEvent: (a: number, b: number, c: number, d: number) => void;
  readonly mybotdetection_saveResult: (a: number, b: number) => void;
  readonly mybotdetection_saveBorrowedResult: (a: number, b: number) => void;
  readonly mybotdetection_allEvents: (a: number, b: number) => void;
  readonly mybotdetection_events: (a: number, b: number, c: number, d: number) => void;
  readonly mybotdetection_num_events: (a: number) => number;
  readonly mybotdetection_results: (a: number, b: number) => void;
  readonly mybotdetection_isBot: (a: number) => number;
  readonly botdetectionoutput_text: (a: number, b: number) => void;
  readonly coordinate_new: (a: number, b: number) => number;
  readonly __wbg_set_event_timestamp: (a: number, b: number) => void;
  readonly __wbg_get_event_timestamp: (a: number) => number;
  readonly __wbindgen_export_0: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
