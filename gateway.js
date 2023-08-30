const mqtt = require('mqtt')

const { Actor, HttpAgent } = require('@dfinity/agent');
const { Principal } = require('@dfinity/principal');
const { Ed25519KeyIdentity } = require('@dfinity/identity');

const { idlFactory } = require('./src/service.did.js')

const fs = require('fs');
var keyData = fs.readFileSync('./key.json', 'utf8');
var key = Ed25519KeyIdentity.fromJSON(keyData);
console.log("Loaded principal: " + key.getPrincipal().toString())

// const host = "https://ic0.app/"; //ic
const host = "http://127.0.0.1:4943/"; //ic
const http = new HttpAgent({ identity: key, host: host });

const client = mqtt.connect('mqtt://127.0.0.1:1883')
client.on('connect', function () {
  console.log("Connected")
  client.subscribe('/#', function (err) {
    console.log("Subscribed")
  })
})

const canistersConfig = {};
const canisterIntervals = {};
const canisterIndex = {};

async function run() {
  await http.fetchRootKey();

  client.on('message', async function (topic, message) {

    let path = topic.split('/')

    try {
      let principal = Principal.fromText(path[1]);
      const actor = Actor.createActor(idlFactory, {
        agent: http,
        canisterId: principal,
      });

      var settings = canistersConfig[principal.toText()];
      if (settings === undefined) {
        settings = await actor.getSettings();
        canistersConfig[principal.toText()] = settings;

        if (settings.Ok !== undefined) {
          if (settings.Ok.interval > 0) {
            canisterIntervals[principal.toText()] = setInterval(async () => {

              let index = canisterIndex[principal.toText()] ?? 0;
              let data = await actor.getMessages(BigInt(index));

              canisterIndex[principal.toText()] = index + data.Ok.data.length

              if (data.Ok.data.length > 0) {
                for (let msg of data.Ok.data) {
                  console.log('Publishing:', msg);
                  client.publish(msg.index, msg.topic)
                }
              }

            }, Number(settings.Ok.interval * 1000n))
          }
        }
      }

      let result = await actor.onMessage(topic.toString(), message.toString());
      console.log("Result: ", result);
    } catch (e) {
      console.log('Could not process message', e)
    }
  });
}

run();