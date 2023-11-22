module.exports.idlFactory = ({ IDL }) => {
  const Message = IDL.Record({
    'topic' : IDL.Text,
    'message' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'index' : IDL.Nat64,
  });
  const PagedResult = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Vec(Message),
    'skip' : IDL.Nat64,
    'limit' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : PagedResult, 'Err' : IDL.Text });
  const Settings = IDL.Record({
    'interval' : IDL.Nat64,
    'owner' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Settings, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    'getInMessages' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'getMessages' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'getSettings' : IDL.Func([], [Result_1], ['query']),
    'onMessage' : IDL.Func([IDL.Text, IDL.Text], [Result_2], []),
  });
};
module.exports.init = ({ IDL }) => { return []; };
