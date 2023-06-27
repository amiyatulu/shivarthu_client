import {transfer_balance} from './js_api/balance/balance_tranfer';
import {get_api , get_account_address_from_seed, helloworld} from './js_api/api';
import {add_profile } from './js_api/profile_validation/add_profile';
import {chat_text} from './js_api/ai/chat_hugging_face';
import { waitReady } from "@polkadot/wasm-crypto";


await waitReady();
export {transfer_balance, get_api, get_account_address_from_seed, helloworld, add_profile, chat_text}













