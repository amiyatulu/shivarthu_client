import { ApiPromise, WsProvider } from '@polkadot/api';

export async function helloworld() {
    const wsProvider = new WsProvider('wss://rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: wsProvider });

    // Do something
    console.log(api.genesisHash.toHex());
}
