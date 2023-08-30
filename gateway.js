const mqtt = require('mqtt')

const { Actor, HttpAgent } = require('@dfinity/agent');
const { Principal } = require('@dfinity/principal');
const { Ed25519KeyIdentity } = require('@dfinity/identity');

const { idlFactory } = require('./.dfx/local/canisters/storage/storage.did.js')

const fs = require('fs');
var keyData = fs.readFileSync('./key.json', 'utf8');
var key = Ed25519KeyIdentity.fromJSON(keyData);
console.log("Loaded principal: " + key.getPrincipal().toString())

//specify localhost endpoint or ic endpoint;
const canister_id = 'bkyz2-fmaaa-aaaaa-qaaaq-cai'
// const host = "https://ic0.app/"; //ic
const host = "http://127.0.0.1:4943/"; //ic
const http = new HttpAgent({ identity: key, host: host });

const client = mqtt.connect('mqtt://127.0.0.1:1883')
client.on('connect', function () {
  console.log("Connected")
  client.subscribe('/#', function (err) {
    console.log("Subscribed")
    if (!err) {
      client.publish('presence', 'Hello mqtt')
    }
  })
})

client.on('message', function (topic, message) {
  // message is Buffer
  console.log(topic.toString() + ": " + message.toString())
  //   client.end()
})

async function run() {
  await http.fetchRootKey();
  const actor = Actor.createActor(idlFactory, {
    agent: http,
    canisterId: canister_id,
  });

  let settings = await actor.getSettings();

  console.log("Interval: " , settings.Ok.interval);
  console.log("Owner: " , settings.Ok.owner.toText());


  client.on('message', async function (topic, message) {
    let result = await actor.onMessage(topic.toString(), message.toString());
    console.log("Result: " , result);
  });
}

run();