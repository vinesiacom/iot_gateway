export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Settings = IDL.Record({
    'interval' : IDL.Nat64,
    'owner' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Settings, 'Err' : IDL.Text });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'status_code' : IDL.Nat16,
  });
  const Entry = IDL.Record({
    'tags' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'fields' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'timestamp' : IDL.Nat64,
  });
  return IDL.Service({
    'filter' : IDL.Func([], [Result], ['query']),
    'getSettings' : IDL.Func([], [Result_1], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'insert' : IDL.Func([IDL.Text, Entry], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
