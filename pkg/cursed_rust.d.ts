/* tslint:disable */
/* eslint-disable */
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
   * @param {RoboDetectionOutput} result
   */
  saveResult(result: RoboDetectionOutput): void;
  /**
   * @param {RoboDetectionOutput} result
   */
  saveBorrowedResult(result: RoboDetectionOutput): void;
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
   * @returns {RoboDetectionOutput}
   */
  isBot(): RoboDetectionOutput;
  readonly results: (RoboDetectionOutput)[];
}
export class RoboDetectionOutput {
  free(): void;
  /**
   * @returns {string}
   */
  text(): string;
  humanScore: number;
  jitter: number;
  timestamp: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_myrobodetection_free: (a: number, b: number) => void;
  readonly __wbg_event_free: (a: number, b: number) => void;
  readonly __wbg_get_event_timestamp: (a: number) => number;
  readonly __wbg_set_event_timestamp: (a: number, b: number) => void;
  readonly __wbg_get_event_coordinate: (a: number) => number;
  readonly __wbg_set_event_coordinate: (a: number, b: number) => void;
  readonly __wbg_coordinate_free: (a: number, b: number) => void;
  readonly __wbg_get_coordinate_x: (a: number) => number;
  readonly __wbg_set_coordinate_x: (a: number, b: number) => void;
  readonly __wbg_get_coordinate_y: (a: number) => number;
  readonly __wbg_set_coordinate_y: (a: number, b: number) => void;
  readonly __wbg_robodetectionoutput_free: (a: number, b: number) => void;
  readonly __wbg_get_robodetectionoutput_jitter: (a: number) => number;
  readonly __wbg_set_robodetectionoutput_jitter: (a: number, b: number) => void;
  readonly __wbg_get_robodetectionoutput_humanScore: (a: number) => number;
  readonly __wbg_set_robodetectionoutput_humanScore: (a: number, b: number) => void;
  readonly myrobodetection_new: () => number;
  readonly myrobodetection_fromEvents: (a: number, b: number) => number;
  readonly myrobodetection_addEvent: (a: number, b: number, c: number, d: number) => void;
  readonly myrobodetection_saveResult: (a: number, b: number) => void;
  readonly myrobodetection_saveBorrowedResult: (a: number, b: number) => void;
  readonly myrobodetection_allEvents: (a: number, b: number) => void;
  readonly myrobodetection_events: (a: number, b: number, c: number, d: number) => void;
  readonly myrobodetection_num_events: (a: number) => number;
  readonly myrobodetection_results: (a: number, b: number) => void;
  readonly myrobodetection_isBot: (a: number) => number;
  readonly robodetectionoutput_text: (a: number, b: number) => void;
  readonly coordinate_new: (a: number, b: number) => number;
  readonly __wbg_set_robodetectionoutput_timestamp: (a: number, b: number) => void;
  readonly __wbg_get_robodetectionoutput_timestamp: (a: number) => number;
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
