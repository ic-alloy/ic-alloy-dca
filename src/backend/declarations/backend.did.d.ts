import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CanisterSettingsInput {
  'asset' : string,
  'interval' : bigint,
  'owner' : Principal,
  'amount' : bigint,
}
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export interface _SERVICE {
  'get_address' : ActorMethod<[[] | [Principal]], Result>,
  'get_balance' : ActorMethod<[[] | [Principal]], Result>,
  'send_eth' : ActorMethod<[string, bigint], Result>,
  'update_settings' : ActorMethod<[CanisterSettingsInput], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
