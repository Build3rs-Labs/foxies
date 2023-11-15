import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise, WsProvider } from '@polkadot/api';
import fetch from 'node-fetch';
import { Keyring } from '@polkadot/keyring';
import fs from 'fs';

const columns = [];

const contract_abi = JSON.parse(fs.readFileSync("./outputs/factory.json"));
const contract_address = "FACTORY_ADDRESS_HERE";

const mnemonic = "MNEMONIC_HERE";

var wsProvider;
var api;

var account;

var contract;

var callOptions;

const doTxn = async ()=> {

  wsProvider = new WsProvider('wss://ws.test.azero.dev');
  api = await ApiPromise.create({ provider: wsProvider });

  callOptions = {
    gasLimit: api.registry.createType('WeightV2', {
        refTime:99999999999,
        proofSize:99999999999,
    }),
    storageDepositLimit:99999999999
  };

  const keyring = new Keyring({type:"sr25519"});

  account = keyring.addFromUri(mnemonic);

  contract = new ContractPromise(api, contract_abi, contract_address);

  distribute();

}

const distribute = async ()=> {
    
    for (let i = 0; i < 1000; i++) {
        let unsub = await contract.tx["generateRandomNft"](callOptions).signAndSend(account, 
            ({ events = [], status }) => {
                if (status.isInBlock) {
                    console.log(`pending mint`);
                } else if (status.isFinalized) {
                    let failed = false;
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        if (method == "ExtrinsicFailed") {
                            failed = true;
                        }
                    });
                    if (failed == true) {
                        console.log(`failed mint`);
                    }
                    else {
                        console.log(`confirmed mint: ${i + 1} minted`);
                    }
                    unsub();
                }
            }
        );
        await promise();
    }
}

const promise = (time=3000)=> {
    return new Promise((resolve, reject)=> {
        setTimeout(()=> {
            resolve(true);
        }, time);
    });
}

doTxn();
