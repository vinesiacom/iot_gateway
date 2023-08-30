const mqtt = require('mqtt')

const { Actor, HttpAgent } = require('@dfinity/agent');
const { Principal } = require('@dfinity/principal');
const { Ed25519KeyIdentity } = require('@dfinity/identity');

const { idlFactory } = require('./src/service.did.js')

const fs = require('fs');

var keyData = fs.readFileSync('./key.json', 'utf8');
var key = Ed25519KeyIdentity.fromJSON(keyData);
console.log("Loaded principal: " + key.getPrincipal().toString())

//specify localhost endpoint or ic endpoint;
const canister_id = 'bkyz2-fmaaa-aaaaa-qaaaq-cai'
// const host = "https://ic0.app/"; //ic
const host = "http://127.0.0.1:4943/"; //ic
const http = new HttpAgent({ identity: key, host: host });

async function run() {
  await http.fetchRootKey();
  const actor = Actor.createActor(idlFactory, {
    agent: http,
    canisterId: canister_id,
  });

  let settings = await actor.getSettings();

  console.log("Interval: " , settings.Ok.interval);
  console.log("Owner: " , settings.Ok.owner.toText());

  let items = await actor.getMessages(0n);
  console.log(items.Ok.length);
}

run();