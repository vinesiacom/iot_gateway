import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Entry {
  'tags' : Array<[string, string]>,
  'fields' : Array<[string, string]>,
  'timestamp' : bigint,
}
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'status_code' : number,
}
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Settings } |
  { 'Err' : string };
export interface Settings { 'interval' : bigint, 'owner' : Principal }
export interface _SERVICE {
  'filter' : ActorMethod<[], Result>,
  'getSettings' : ActorMethod<[], Result_1>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'insert' : ActorMethod<[string, Entry], Result>,
}
