import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CanisterSettingsInput {
  'fee' : bigint,
  'interval' : bigint,
  'owner' : string,
  'swap_token' : string,
  'base_token' : string,
  'amount_in' : bigint,
  'slippage' : bigint,
}
export interface LogItem {
  'ok' : [] | [string],
  'err' : [] | [string],
  'event' : LogItemEvent,
}
export type LogItemEvent = { 'Approve' : null } |
  { 'Swap' : null };
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export interface _SERVICE {
  'get_address' : ActorMethod<[], Result>,
  'get_log' : ActorMethod<[], Array<LogItem>>,
  'start' : ActorMethod<[], Result>,
  'stop' : ActorMethod<[], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
