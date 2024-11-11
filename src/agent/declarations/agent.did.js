export const idlFactory = ({ IDL }) => {
  const CanisterSettingsDto = IDL.Record({
    'fee' : IDL.Nat64,
    'interval' : IDL.Nat64,
    'token_in_address' : IDL.Text,
    'owner' : IDL.Text,
    'token_out_address' : IDL.Text,
    'amount_in' : IDL.Nat64,
    'token_in_name' : IDL.Text,
    'token_out_name' : IDL.Text,
    'slippage' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const LogItemEvent = IDL.Variant({
    'Start' : IDL.Null,
    'Approve' : IDL.Null,
    'Stop' : IDL.Null,
    'Swap' : IDL.Null,
    'Transfer' : IDL.Null,
    'SavePoolAddress' : IDL.Null,
  });
  const LogItem = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Opt(IDL.Text),
    'event' : LogItemEvent,
    'timestamp' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({
    'Ok' : CanisterSettingsDto,
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'get_balance_in' : IDL.Func([], [Result], []),
    'get_balance_out' : IDL.Func([], [Result], []),
    'get_canister_address' : IDL.Func([], [Result], []),
    'get_log' : IDL.Func([], [IDL.Vec(LogItem)], ['query']),
    'get_settings' : IDL.Func([], [Result_1], ['query']),
    'start' : IDL.Func([], [Result], []),
    'stop' : IDL.Func([], [Result], []),
    'transfer_in_token' : IDL.Func([IDL.Text, IDL.Nat64], [Result], []),
    'transfer_out_token' : IDL.Func([IDL.Text, IDL.Nat64], [Result], []),
  });
};
export const init = ({ IDL }) => {
  const CanisterSettingsDto = IDL.Record({
    'fee' : IDL.Nat64,
    'interval' : IDL.Nat64,
    'token_in_address' : IDL.Text,
    'owner' : IDL.Text,
    'token_out_address' : IDL.Text,
    'amount_in' : IDL.Nat64,
    'token_in_name' : IDL.Text,
    'token_out_name' : IDL.Text,
    'slippage' : IDL.Nat64,
  });
  return [CanisterSettingsDto];
};
