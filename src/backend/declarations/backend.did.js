export const idlFactory = ({ IDL }) => {
  const CanisterSettingsInput = IDL.Record({
    'asset' : IDL.Text,
    'interval' : IDL.Nat64,
    'owner' : IDL.Principal,
    'amount' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  return IDL.Service({
    'get_address' : IDL.Func([IDL.Opt(IDL.Principal)], [Result], []),
    'get_balance' : IDL.Func([IDL.Opt(IDL.Principal)], [Result], []),
    'send_eth' : IDL.Func([IDL.Text, IDL.Nat], [Result], []),
    'update_settings' : IDL.Func([CanisterSettingsInput], [], []),
  });
};
export const init = ({ IDL }) => {
  const CanisterSettingsInput = IDL.Record({
    'asset' : IDL.Text,
    'interval' : IDL.Nat64,
    'owner' : IDL.Principal,
    'amount' : IDL.Nat64,
  });
  return [CanisterSettingsInput];
};
