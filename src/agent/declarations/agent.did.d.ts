import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CanisterSettingsDto {
  'fee' : bigint,
  'interval' : bigint,
  'token_in_address' : string,
  'owner' : string,
  'token_out_address' : string,
  'amount_in' : bigint,
  'token_in_name' : string,
  'token_out_name' : string,
  'slippage' : bigint,
}
export interface LogItem {
  'ok' : [] | [string],
  'err' : [] | [string],
  'event' : LogItemEvent,
  'timestamp' : bigint,
}
export type LogItemEvent = { 'Start' : null } |
  { 'Approve' : null } |
  { 'Stop' : null } |
  { 'Swap' : null } |
  { 'Transfer' : null } |
  { 'SavePoolAddress' : null };
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : CanisterSettingsDto } |
  { 'Err' : string };
export interface _SERVICE {
  'get_balance_in' : ActorMethod<[], Result>,
  'get_balance_out' : ActorMethod<[], Result>,
  'get_canister_address' : ActorMethod<[], Result>,
  'get_log' : ActorMethod<[], Array<LogItem>>,
  'get_settings' : ActorMethod<[], Result_1>,
  'start' : ActorMethod<[], Result>,
  'stop' : ActorMethod<[], Result>,
  'transfer_in_token' : ActorMethod<[string, bigint], Result>,
  'transfer_out_token' : ActorMethod<[string, bigint], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
