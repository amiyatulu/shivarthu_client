import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { waitReady } from "@polkadot/wasm-crypto";



export async function helloworld() {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    console.log(api.genesisHash.toHex());
}

export async function get_api(wsprovider) {
    const wsProvider = new WsProvider(wsprovider);
    const api = await ApiPromise.create({ provider: wsProvider });
    return api;

}

export function get_account_address_from_seed(mnemonic) {
    // console.log(mnemonic);    
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.createFromUri(mnemonic);
    // console.log("pair address", pair.address);
    return pair.address;
}


export async function transfer_balance(wsprovider, mnemonic, credit_user, value) {
    await waitReady();
    return new Promise((resolve, reject) => {
        const apiPromise = get_api(wsprovider);
        const keyring = new Keyring({ type: 'sr25519' });
        const pair = keyring.createFromUri(mnemonic);
         apiPromise.then((api) => {
            api.tx.balances.transfer(credit_user, value)
                .signAndSend(pair, ({ status, dispatchError }) => {
                    if (dispatchError) {
                        if (dispatchError.isModule) {
                            const decoded = api.registry.findMetaError(dispatchError.asModule);
                            const { docs, name, section } = decoded;
                            reject(`Error ${section}.${name}: ${docs.join(' ')}`);
                        } else {
                            reject(`Error ${dispatchError.toString()}`);
                        }
                    } else if (status.isFinalized) {
                        resolve(`Finalized ${status.asFinalized.toString()}`);
                    }
                })
                .catch(reject);
        }).catch(reject);
    });
}