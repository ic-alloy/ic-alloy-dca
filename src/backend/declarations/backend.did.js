export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const Wei = IDL.Nat;
  return IDL.Service({
    'get_address' : IDL.Func([IDL.Opt(IDL.Principal)], [Result], []),
    'get_balance' : IDL.Func([IDL.Opt(IDL.Principal)], [Result], []),
    'send_eth' : IDL.Func([IDL.Text, Wei], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
