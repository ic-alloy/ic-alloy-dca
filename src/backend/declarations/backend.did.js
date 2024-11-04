export const idlFactory = ({ IDL }) => {
  const CanisterSettingsInput = IDL.Record({
    'fee' : IDL.Nat64,
    'interval' : IDL.Nat64,
    'owner' : IDL.Text,
    'swap_token' : IDL.Text,
    'base_token' : IDL.Text,
    'amount_in' : IDL.Nat64,
    'slippage' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const LogItemEvent = IDL.Variant({ 'Approve' : IDL.Null, 'Swap' : IDL.Null });
  const LogItem = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Opt(IDL.Text),
    'event' : LogItemEvent,
  });
  return IDL.Service({
    'get_address' : IDL.Func([], [Result], []),
    'get_log' : IDL.Func([], [IDL.Vec(LogItem)], ['query']),
    'start' : IDL.Func([], [Result], []),
    'stop' : IDL.Func([], [Result], []),
  });
};
export const init = ({ IDL }) => {
  const CanisterSettingsInput = IDL.Record({
    'fee' : IDL.Nat64,
    'interval' : IDL.Nat64,
    'owner' : IDL.Text,
    'swap_token' : IDL.Text,
    'base_token' : IDL.Text,
    'amount_in' : IDL.Nat64,
    'slippage' : IDL.Nat64,
  });
  return [CanisterSettingsInput];
};
