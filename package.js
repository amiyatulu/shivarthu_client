import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const wsProvider = new WsProvider('wss://rpc.polkadot.io');
const api = await ApiPromise.create({ provider: wsProvider });

export async function helloworld() {
    console.log(api.genesisHash.toHex());
}

export function get_account_address_from_seed(mnemonic) {
    // console.log(mnemonic);    
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.createFromUri(mnemonic);
    // console.log("pair address", pair.address);
    return pair.address;
}
