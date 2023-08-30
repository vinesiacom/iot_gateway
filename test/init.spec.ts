import { TestContext, WasmCanister, u64IntoPrincipalId } from 'lightic'
import fs from 'fs'

import { _SERVICE as mqtt } from "../.dfx/local/canisters/storage/service.did"

const context = new TestContext();

describe('test', () => {
    it('test', async () => {
        let mqtt_canister = await context.deploy('./target/wasm32-unknown-unknown/release/mqtt.wasm', {

        })

        let id = u64IntoPrincipalId(0n)
        let agent = context.getAgent(id)
        let actor = agent.getActor(mqtt_canister) as any as mqtt

        let addResult = await actor.onMessage('topic', 'message')
        console.log('addResult', addResult);

        let msgs = await actor.getMessages(1n);
        console.log('getMessages', msgs);

        //test code
    });

    it('generate_candid', async () => {
        let mqtt_canister = await context.deploy('./target/wasm32-unknown-unknown/release/mqtt.wasm') as WasmCanister
        let raw_candid = await mqtt_canister.get_candid();

        fs.writeFileSync('./src/mqtt.did', raw_candid)
    });
});
