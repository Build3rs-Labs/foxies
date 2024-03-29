import { ContractPromise } from '@polkadot/api-contract';
import { toastError, toastSuccess } from "@/components/toast";
import { useWallet, useAllWallets } from 'useink';

export const formatWallet = (address) => {
    return `${address.substring(0, 6)}...${address.substring(address.length - 6)}`;
}

const doFormatNumber = (num)=> {
    return Number(num.replace(/\,/g, ""));
}

const ABIs = {
    factory:{"source":{"hash":"0xfc59eefc2b0e5095a89cb86457d9fa9faaf89b1942bf80c0645e6d276dd56c28","language":"ink! 4.3.0","compiler":"rustc 1.78.0-nightly","build_info":{"build_mode":"Release","cargo_contract_version":"3.2.0","rust_toolchain":"nightly-x86_64-unknown-linux-gnu","wasm_opt_settings":{"keep_debug_symbols":false,"optimization_passes":"Z"}}},"contract":{"name":"factory","version":"0.2.1","authors":["Edinyanga Ottoho","Build3rs"],"description":"Factory contract that mints foxes or chickens","repository":"https://github.com/Build3rs-Labs/foxies","homepage":"https://github.com/Build3rs-Labs/foxies","license":"Apache-2.0"},"spec":{"constructors":[{"args":[{"label":"fees_account","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"new","payable":false,"returnType":{"displayName":["ink_primitives","ConstructorResult"],"type":6},"selector":"0x9bae9d5e"}],"docs":[],"environment":{"accountId":{"displayName":["AccountId"],"type":2},"balance":{"displayName":["Balance"],"type":1},"blockNumber":{"displayName":["BlockNumber"],"type":25},"chainExtension":{"displayName":["ChainExtension"],"type":26},"hash":{"displayName":["Hash"],"type":23},"maxEventTopics":4,"timestamp":{"displayName":["Timestamp"],"type":24}},"events":[],"lang_error":{"displayName":["ink","LangError"],"type":8},"messages":[{"args":[],"default":false,"docs":[],"label":"get_account_id","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":9},"selector":"0x79718546"},{"args":[{"label":"address","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"set_staking_address","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0xf719335e"},{"args":[{"label":"address","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"set_chickens_nft_address","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0x6bc4bc7e"},{"args":[{"label":"address","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"set_foxes_nft_address","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0x210e80e5"},{"args":[{"label":"address","type":{"displayName":["AccountId"],"type":2}},{"label":"to_have","type":{"displayName":["bool"],"type":5}}],"default":false,"docs":[],"label":"add_whitelisted","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0xd6cf00f2"},{"args":[{"label":"address","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"remove_whitelisted","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0x5e96698f"},{"args":[{"label":"amount","type":{"displayName":["u128"],"type":1}}],"default":false,"docs":[],"label":"set_azero_for_direct_fox_mints","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0x864d61c0"},{"args":[],"default":false,"docs":[],"label":"get_azero_for_direct_fox_mints","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":14},"selector":"0x49a8680d"},{"args":[],"default":false,"docs":[],"label":"get_platform_status","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":15},"selector":"0x190dca10"},{"args":[],"default":false,"docs":[],"label":"get_azero_for_random_mints","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":14},"selector":"0x5108e1e1"},{"args":[{"label":"mint_type","type":{"displayName":["u8"],"type":4}},{"label":"account","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"mint_by_admin","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0xb3f4e790"},{"args":[],"default":false,"docs":[],"label":"mint_nft","mutates":true,"payable":true,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0x219a113e"},{"args":[],"default":false,"docs":[],"label":"pick_random_fox_holder_with_rarity","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":17},"selector":"0x230aeaa1"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"get_last_mint_by_account","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":19},"selector":"0x1a7d8b69"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":2}}],"default":false,"docs":[],"label":"get_direct_fox_mints","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":22},"selector":"0x81ce1659"},{"args":[{"label":"index","type":{"displayName":["u128"],"type":1}}],"default":false,"docs":[],"label":"get_fox_rarity","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":14},"selector":"0x9f060b73"}]},"storage":{"root":{"layout":{"struct":{"fields":[{"layout":{"root":{"layout":{"leaf":{"key":"0x533e17ba","ty":0}},"root_key":"0x533e17ba"}},"name":"rarities"},{"layout":{"leaf":{"key":"0x00000000","ty":0}},"name":"nfts"},{"layout":{"root":{"layout":{"leaf":{"key":"0x2fc38212","ty":1}},"root_key":"0x2fc38212"}},"name":"nfts_rarity"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":2}},"name":"0"}],"name":"Some"}}}},"name":"chickens_nft_address"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":2}},"name":"0"}],"name":"Some"}}}},"name":"foxes_nft_address"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":2}},"name":"0"}],"name":"Some"}}}},"name":"owner"},{"layout":{"root":{"layout":{"enum":{"dispatchKey":"0x334323a4","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"struct":{"fields":[{"layout":{"leaf":{"key":"0x334323a4","ty":4}},"name":"0"},{"layout":{"leaf":{"key":"0x334323a4","ty":1}},"name":"1"}],"name":"(A, B)"}},"name":"0"}],"name":"Some"}}}},"root_key":"0x334323a4"}},"name":"last_mint"},{"layout":{"leaf":{"key":"0x00000000","ty":1}},"name":"chickens_minted"},{"layout":{"root":{"layout":{"leaf":{"key":"0x3f710c0b","ty":4}},"root_key":"0x3f710c0b"}},"name":"direct_fox_mints"},{"layout":{"leaf":{"key":"0x00000000","ty":1}},"name":"azero_for_direct_fox_mints"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":2}},"name":"0"}],"name":"Some"}}}},"name":"fees_account"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":2}},"name":"0"}],"name":"Some"}}}},"name":"rewards_pool"},{"layout":{"root":{"layout":{"leaf":{"key":"0xdd339047","ty":5}},"root_key":"0xdd339047"}},"name":"whitelisted"},{"layout":{"leaf":{"key":"0x00000000","ty":1}},"name":"azero_traded"},{"layout":{"leaf":{"key":"0x00000000","ty":1}},"name":"azero_claimed"}],"name":"Factory"}},"root_key":"0x00000000"}},"types":[{"id":0,"type":{"def":{"sequence":{"type":1}}}},{"id":1,"type":{"def":{"primitive":"u128"}}},{"id":2,"type":{"def":{"composite":{"fields":[{"type":3,"typeName":"[u8; 32]"}]}},"path":["ink_primitives","types","AccountId"]}},{"id":3,"type":{"def":{"array":{"len":32,"type":4}}}},{"id":4,"type":{"def":{"primitive":"u8"}}},{"id":5,"type":{"def":{"primitive":"bool"}}},{"id":6,"type":{"def":{"variant":{"variants":[{"fields":[{"type":7}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":7},{"name":"E","type":8}],"path":["Result"]}},{"id":7,"type":{"def":{"tuple":[]}}},{"id":8,"type":{"def":{"variant":{"variants":[{"index":1,"name":"CouldNotReadInput"}]}},"path":["ink_primitives","LangError"]}},{"id":9,"type":{"def":{"variant":{"variants":[{"fields":[{"type":2}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":2},{"name":"E","type":8}],"path":["Result"]}},{"id":10,"type":{"def":{"variant":{"variants":[{"fields":[{"type":11}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":11},{"name":"E","type":8}],"path":["Result"]}},{"id":11,"type":{"def":{"variant":{"variants":[{"fields":[{"type":7}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":7},{"name":"E","type":12}],"path":["Result"]}},{"id":12,"type":{"def":{"variant":{"variants":[{"fields":[{"type":13,"typeName":"String"}],"index":0,"name":"Custom"},{"index":1,"name":"FailedMint"},{"index":2,"name":"OnlyOwnerAllowed"},{"index":3,"name":"InvalidMintPayment"},{"index":4,"name":"ExceededDirectFoxMintAllowance"},{"index":5,"name":"FailedAZEROTransfer"},{"index":6,"name":"InvalidMintType"}]}},"path":["factory","factory","FactoryError"]}},{"id":13,"type":{"def":{"primitive":"str"}}},{"id":14,"type":{"def":{"variant":{"variants":[{"fields":[{"type":1}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":1},{"name":"E","type":8}],"path":["Result"]}},{"id":15,"type":{"def":{"variant":{"variants":[{"fields":[{"type":16}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":16},{"name":"E","type":8}],"path":["Result"]}},{"id":16,"type":{"def":{"tuple":[1,1,1,1,1]}}},{"id":17,"type":{"def":{"variant":{"variants":[{"fields":[{"type":18}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":18},{"name":"E","type":8}],"path":["Result"]}},{"id":18,"type":{"def":{"tuple":[2,1]}}},{"id":19,"type":{"def":{"variant":{"variants":[{"fields":[{"type":20}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":20},{"name":"E","type":8}],"path":["Result"]}},{"id":20,"type":{"def":{"variant":{"variants":[{"index":0,"name":"None"},{"fields":[{"type":21}],"index":1,"name":"Some"}]}},"params":[{"name":"T","type":21}],"path":["Option"]}},{"id":21,"type":{"def":{"tuple":[4,1]}}},{"id":22,"type":{"def":{"variant":{"variants":[{"fields":[{"type":4}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":4},{"name":"E","type":8}],"path":["Result"]}},{"id":23,"type":{"def":{"composite":{"fields":[{"type":3,"typeName":"[u8; 32]"}]}},"path":["ink_primitives","types","Hash"]}},{"id":24,"type":{"def":{"primitive":"u64"}}},{"id":25,"type":{"def":{"primitive":"u32"}}},{"id":26,"type":{"def":{"variant":{}},"path":["ink_env","types","NoChainExtension"]}}],"version":"4"},
    staking:{"source":{"hash":"0x9270b90580393648e66be3b542b739c14bbd1e3696d974960c1b6da2bfe1af69","language":"ink! 4.3.0","compiler":"rustc 1.78.0-nightly","build_info":{"build_mode":"Release","cargo_contract_version":"3.2.0","rust_toolchain":"nightly-x86_64-unknown-linux-gnu","wasm_opt_settings":{"keep_debug_symbols":false,"optimization_passes":"Z"}}},"contract":{"name":"staking","version":"0.2.1","authors":["Edinyanga Ottoho","Build3rs"],"description":"Staking contract to stake chicken and earn $AZERO","repository":"https://github.com/Build3rs-Labs/foxies","homepage":"https://github.com/Build3rs-Labs/foxies","license":"Apache-2.0"},"spec":{"constructors":[{"args":[{"label":"factory","type":{"displayName":["AccountId"],"type":0}},{"label":"foxes","type":{"displayName":["AccountId"],"type":0}},{"label":"chickens","type":{"displayName":["AccountId"],"type":0}},{"label":"daily_azero_per_chicken","type":{"displayName":["u128"],"type":3}},{"label":"cap_per_account","type":{"displayName":["u128"],"type":3}}],"default":false,"docs":[],"label":"new","payable":false,"returnType":{"displayName":["ink_primitives","ConstructorResult"],"type":6},"selector":"0x9bae9d5e"}],"docs":[],"environment":{"accountId":{"displayName":["AccountId"],"type":0},"balance":{"displayName":["Balance"],"type":3},"blockNumber":{"displayName":["BlockNumber"],"type":19},"chainExtension":{"displayName":["ChainExtension"],"type":20},"hash":{"displayName":["Hash"],"type":18},"maxEventTopics":4,"timestamp":{"displayName":["Timestamp"],"type":4}},"events":[],"lang_error":{"displayName":["ink","LangError"],"type":8},"messages":[{"args":[],"default":false,"docs":[],"label":"get_account_id","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":9},"selector":"0x79718546"},{"args":[{"label":"id","type":{"displayName":["u128"],"type":3}}],"default":false,"docs":[],"label":"stake_chicken","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0x51642ef3"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_staked_chickens","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":14},"selector":"0xbe5cc8f7"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_staked_foxes","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":14},"selector":"0xa63f2aff"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_last_fox_for_stolen_azero","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":15},"selector":"0xa185da18"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_last_time_for_stolen_azero","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":17},"selector":"0x30baf03f"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_last_steal","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":14},"selector":"0xeddc0753"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_claimable_azero","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":17},"selector":"0x1f16e5dd"},{"args":[],"default":false,"docs":[],"label":"unstake_chickens","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0xfb97b871"},{"args":[],"default":false,"docs":[],"label":"get_azero_claimed","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":17},"selector":"0xbd9c65d0"},{"args":[],"default":false,"docs":[],"label":"get_azero_in_pool","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":17},"selector":"0xed732ac1"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"get_claimable_for_fox","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":17},"selector":"0x73336d30"},{"args":[{"label":"id","type":{"displayName":["u128"],"type":3}}],"default":false,"docs":[],"label":"stake_fox","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0xc7738e66"},{"args":[],"default":false,"docs":[],"label":"unstake_foxes","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":10},"selector":"0xd786eb86"}]},"storage":{"root":{"layout":{"struct":{"fields":[{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":0}},"name":"0"}],"name":"Some"}}}},"name":"factory"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":0}},"name":"0"}],"name":"Some"}}}},"name":"foxes"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":0}},"name":"0"}],"name":"Some"}}}},"name":"chickens"},{"layout":{"root":{"layout":{"leaf":{"key":"0x783f1d8b","ty":3}},"root_key":"0x783f1d8b"}},"name":"staked_chickens"},{"layout":{"root":{"layout":{"leaf":{"key":"0x27ad14a5","ty":3}},"root_key":"0x27ad14a5"}},"name":"number_of_chickens_staked"},{"layout":{"root":{"layout":{"leaf":{"key":"0x17c755b3","ty":4}},"root_key":"0x17c755b3"}},"name":"last_chickens_stake_time"},{"layout":{"root":{"layout":{"leaf":{"key":"0xc4e2fc5f","ty":3}},"root_key":"0xc4e2fc5f"}},"name":"staked_foxes"},{"layout":{"root":{"layout":{"leaf":{"key":"0x5cad1138","ty":3}},"root_key":"0x5cad1138"}},"name":"number_of_foxes_staked"},{"layout":{"root":{"layout":{"leaf":{"key":"0x80988a54","ty":4}},"root_key":"0x80988a54"}},"name":"last_foxes_stake_time"},{"layout":{"leaf":{"key":"0x00000000","ty":3}},"name":"daily_azero_per_chicken"},{"layout":{"leaf":{"key":"0x00000000","ty":3}},"name":"cap_per_account"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":0}},"name":"0"}],"name":"Some"}}}},"name":"owner"},{"layout":{"root":{"layout":{"leaf":{"key":"0xca629984","ty":0}},"root_key":"0xca629984"}},"name":"fox_staked_by"},{"layout":{"root":{"layout":{"enum":{"dispatchKey":"0x8226a561","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x8226a561","ty":0}},"name":"0"}],"name":"Some"}}}},"root_key":"0x8226a561"}},"name":"azero_last_stolen_by"},{"layout":{"root":{"layout":{"leaf":{"key":"0x2fbf3bf1","ty":5}},"root_key":"0x2fbf3bf1"}},"name":"last_steal"},{"layout":{"root":{"layout":{"leaf":{"key":"0xea1bcfc1","ty":3}},"root_key":"0xea1bcfc1"}},"name":"azero_last_stolen_time"},{"layout":{"leaf":{"key":"0x00000000","ty":3}},"name":"azero_claimed"}],"name":"Staking"}},"root_key":"0x00000000"}},"types":[{"id":0,"type":{"def":{"composite":{"fields":[{"type":1,"typeName":"[u8; 32]"}]}},"path":["ink_primitives","types","AccountId"]}},{"id":1,"type":{"def":{"array":{"len":32,"type":2}}}},{"id":2,"type":{"def":{"primitive":"u8"}}},{"id":3,"type":{"def":{"primitive":"u128"}}},{"id":4,"type":{"def":{"primitive":"u64"}}},{"id":5,"type":{"def":{"sequence":{"type":3}}}},{"id":6,"type":{"def":{"variant":{"variants":[{"fields":[{"type":7}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":7},{"name":"E","type":8}],"path":["Result"]}},{"id":7,"type":{"def":{"tuple":[]}}},{"id":8,"type":{"def":{"variant":{"variants":[{"index":1,"name":"CouldNotReadInput"}]}},"path":["ink_primitives","LangError"]}},{"id":9,"type":{"def":{"variant":{"variants":[{"fields":[{"type":0}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":0},{"name":"E","type":8}],"path":["Result"]}},{"id":10,"type":{"def":{"variant":{"variants":[{"fields":[{"type":11}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":11},{"name":"E","type":8}],"path":["Result"]}},{"id":11,"type":{"def":{"variant":{"variants":[{"fields":[{"type":7}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":7},{"name":"E","type":12}],"path":["Result"]}},{"id":12,"type":{"def":{"variant":{"variants":[{"fields":[{"type":13,"typeName":"String"}],"index":0,"name":"Custom"},{"index":1,"name":"ChickenNotStaked"},{"index":2,"name":"TokenNotExists"},{"index":3,"name":"TokenNotOwnedByCaller"},{"index":4,"name":"AllowanceInexistent"},{"index":5,"name":"ExceededMaxStakes"},{"index":6,"name":"TransferFailed"},{"index":7,"name":"FailedUnstake"},{"index":8,"name":"OnlyOwnerAllowed"},{"index":9,"name":"NotAFoxHolder"},{"index":10,"name":"HasNotStaked"},{"index":11,"name":"UnableToClaimAzero"},{"index":12,"name":"MintFailed"}]}},"path":["staking","staking","StakingError"]}},{"id":13,"type":{"def":{"primitive":"str"}}},{"id":14,"type":{"def":{"variant":{"variants":[{"fields":[{"type":5}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":5},{"name":"E","type":8}],"path":["Result"]}},{"id":15,"type":{"def":{"variant":{"variants":[{"fields":[{"type":16}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":16},{"name":"E","type":8}],"path":["Result"]}},{"id":16,"type":{"def":{"variant":{"variants":[{"index":0,"name":"None"},{"fields":[{"type":0}],"index":1,"name":"Some"}]}},"params":[{"name":"T","type":0}],"path":["Option"]}},{"id":17,"type":{"def":{"variant":{"variants":[{"fields":[{"type":3}],"index":0,"name":"Ok"},{"fields":[{"type":8}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":3},{"name":"E","type":8}],"path":["Result"]}},{"id":18,"type":{"def":{"composite":{"fields":[{"type":1,"typeName":"[u8; 32]"}]}},"path":["ink_primitives","types","Hash"]}},{"id":19,"type":{"def":{"primitive":"u32"}}},{"id":20,"type":{"def":{"variant":{}},"path":["ink_env","types","NoChainExtension"]}}],"version":"4"},
    PSP34:{"source":{"hash":"0xfdd93036f665c312bacedabffde399f3d98fb0608f8dbe0532e810f88a26e372","language":"ink! 4.3.0","compiler":"rustc 1.75.0-nightly","build_info":{"build_mode":"Release","cargo_contract_version":"3.2.0","rust_toolchain":"nightly-x86_64-unknown-linux-gnu","wasm_opt_settings":{"keep_debug_symbols":false,"optimization_passes":"Z"}}},"contract":{"name":"chickens","version":"0.2.1","authors":["Edinyanga Ottoho","Build3rs"],"description":"Chickens collection","repository":"https://github.com/Build3rs-Labs/foxies","homepage":"https://github.com/Build3rs-Labs/foxies","license":"Apache-2.0"},"spec":{"constructors":[{"args":[{"label":"max_supply","type":{"displayName":["Balance"],"type":5}},{"label":"owner","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"new","payable":false,"returnType":{"displayName":["ink_primitives","ConstructorResult"],"type":10},"selector":"0x9bae9d5e"}],"docs":[],"environment":{"accountId":{"displayName":["AccountId"],"type":0},"balance":{"displayName":["Balance"],"type":5},"blockNumber":{"displayName":["BlockNumber"],"type":3},"chainExtension":{"displayName":["ChainExtension"],"type":31},"hash":{"displayName":["Hash"],"type":30},"maxEventTopics":4,"timestamp":{"displayName":["Timestamp"],"type":9}},"events":[{"args":[{"docs":[],"indexed":false,"label":"owner","type":{"displayName":["AccountId"],"type":0}},{"docs":[],"indexed":false,"label":"operator","type":{"displayName":["AccountId"],"type":0}},{"docs":[],"indexed":false,"label":"id","type":{"displayName":["Option"],"type":18}},{"docs":[],"indexed":false,"label":"approved","type":{"displayName":["bool"],"type":4}}],"docs":[],"label":"Approval"},{"args":[{"docs":[],"indexed":false,"label":"from","type":{"displayName":["Option"],"type":17}},{"docs":[],"indexed":false,"label":"to","type":{"displayName":["Option"],"type":17}},{"docs":[],"indexed":false,"label":"id","type":{"displayName":["Id"],"type":14}}],"docs":[],"label":"Transfer"},{"args":[{"docs":[],"indexed":false,"label":"id","type":{"displayName":["Id"],"type":14}},{"docs":[],"indexed":false,"label":"key","type":{"displayName":["Vec"],"type":6}},{"docs":[],"indexed":false,"label":"data","type":{"displayName":["Vec"],"type":6}}],"docs":[],"label":"AttributeSet"}],"lang_error":{"displayName":["ink","LangError"],"type":12},"messages":[{"args":[],"default":false,"docs":[],"label":"PSP34::collection_id","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":13},"selector":"0xffa27a5f"},{"args":[{"label":"owner","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"PSP34::balance_of","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":15},"selector":"0xcde7e55f"},{"args":[{"label":"id","type":{"displayName":["Id"],"type":14}}],"default":false,"docs":[],"label":"PSP34::owner_of","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":16},"selector":"0x1168624d"},{"args":[{"label":"owner","type":{"displayName":["AccountId"],"type":0}},{"label":"operator","type":{"displayName":["AccountId"],"type":0}},{"label":"id","type":{"displayName":["Option"],"type":18}}],"default":false,"docs":[],"label":"PSP34::allowance","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":19},"selector":"0x4790f55a"},{"args":[{"label":"operator","type":{"displayName":["AccountId"],"type":0}},{"label":"id","type":{"displayName":["Option"],"type":18}},{"label":"approved","type":{"displayName":["bool"],"type":4}}],"default":false,"docs":[],"label":"PSP34::approve","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":20},"selector":"0x1932a8b0"},{"args":[{"label":"to","type":{"displayName":["AccountId"],"type":0}},{"label":"id","type":{"displayName":["Id"],"type":14}},{"label":"data","type":{"displayName":["Vec"],"type":6}}],"default":false,"docs":[],"label":"PSP34::transfer","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":20},"selector":"0x3128d61b"},{"args":[{"label":"from","type":{"displayName":["AccountId"],"type":0}},{"label":"to","type":{"displayName":["AccountId"],"type":0}},{"label":"id","type":{"displayName":["Id"],"type":14}},{"label":"data","type":{"displayName":["Vec"],"type":6}}],"default":false,"docs":[],"label":"PSP34::transfer_from","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":20},"selector":"0x718fd38b"},{"args":[],"default":false,"docs":[],"label":"PSP34::total_supply","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":24},"selector":"0x628413fe"},{"args":[],"default":false,"docs":[],"label":"PSP34::max_supply","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":24},"selector":"0x2b7fbd2c"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}}],"default":false,"docs":[],"label":"PSP34Mintable::mint","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":20},"selector":"0x6c41f2ec"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}},{"label":"attributes","type":{"displayName":["Vec"],"type":25}}],"default":false,"docs":[],"label":"PSP34Mintable::mint_with_attributes","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":20},"selector":"0x9d2918b6"},{"args":[{"label":"account","type":{"displayName":["AccountId"],"type":0}},{"label":"id","type":{"displayName":["Id"],"type":14}}],"default":false,"docs":[],"label":"PSP34Burnable::burn","mutates":true,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":20},"selector":"0x63c9877a"},{"args":[{"label":"id","type":{"displayName":["Id"],"type":14}},{"label":"key","type":{"displayName":["Vec"],"type":6}}],"default":false,"docs":[],"label":"PSP34Metadata::get_attribute","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":27},"selector":"0xf19d48d1"},{"args":[{"label":"index","type":{"displayName":["u128"],"type":5}}],"default":false,"docs":[],"label":"PSP34Enumerable::token_by_index","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":29},"selector":"0xcd0340d0"},{"args":[{"label":"owner","type":{"displayName":["AccountId"],"type":0}},{"label":"index","type":{"displayName":["u128"],"type":5}}],"default":false,"docs":[],"label":"PSP34Enumerable::owners_token_by_index","mutates":false,"payable":false,"returnType":{"displayName":["ink","MessageResult"],"type":29},"selector":"0x3bcfb511"}]},"storage":{"root":{"layout":{"struct":{"fields":[{"layout":{"struct":{"fields":[{"layout":{"root":{"layout":{"leaf":{"key":"0x201f277a","ty":0}},"root_key":"0x201f277a"}},"name":"tokens_owner"},{"layout":{"root":{"layout":{"leaf":{"key":"0xa37f2b9b","ty":3}},"root_key":"0xa37f2b9b"}},"name":"tokens_per_owner"},{"layout":{"root":{"layout":{"leaf":{"key":"0x271b5b42","ty":4}},"root_key":"0x271b5b42"}},"name":"allowances"},{"layout":{"root":{"layout":{"leaf":{"key":"0x75a7d14d","ty":4}},"root_key":"0x75a7d14d"}},"name":"allowances_all"},{"layout":{"leaf":{"key":"0x00000000","ty":5}},"name":"total_supply"},{"layout":{"leaf":{"key":"0x00000000","ty":5}},"name":"max_supply"},{"layout":{"root":{"layout":{"leaf":{"key":"0x807b4b97","ty":6}},"root_key":"0x807b4b97"}},"name":"attributes"},{"layout":{"leaf":{"key":"0x00000000","ty":7}},"name":"all_tokens"},{"layout":{"root":{"layout":{"leaf":{"key":"0xde9c56e6","ty":5}},"root_key":"0xde9c56e6"}},"name":"all_tokens_index"},{"layout":{"root":{"layout":{"enum":{"dispatchKey":"0xbbea6ddb","name":"Id","variants":{"0":{"fields":[{"layout":{"leaf":{"key":"0xbbea6ddb","ty":2}},"name":"0"}],"name":"U8"},"1":{"fields":[{"layout":{"leaf":{"key":"0xbbea6ddb","ty":8}},"name":"0"}],"name":"U16"},"2":{"fields":[{"layout":{"leaf":{"key":"0xbbea6ddb","ty":3}},"name":"0"}],"name":"U32"},"3":{"fields":[{"layout":{"leaf":{"key":"0xbbea6ddb","ty":9}},"name":"0"}],"name":"U64"},"4":{"fields":[{"layout":{"leaf":{"key":"0xbbea6ddb","ty":5}},"name":"0"}],"name":"U128"},"5":{"fields":[{"layout":{"leaf":{"key":"0xbbea6ddb","ty":6}},"name":"0"}],"name":"Bytes"}}}},"root_key":"0xbbea6ddb"}},"name":"owned_tokens"},{"layout":{"root":{"layout":{"leaf":{"key":"0x2887308b","ty":5}},"root_key":"0x2887308b"}},"name":"owned_tokens_index"}],"name":"PSP34Data"}},"name":"data"},{"layout":{"enum":{"dispatchKey":"0x00000000","name":"Option","variants":{"0":{"fields":[],"name":"None"},"1":{"fields":[{"layout":{"leaf":{"key":"0x00000000","ty":0}},"name":"0"}],"name":"Some"}}}},"name":"owner"}],"name":"Chickens"}},"root_key":"0x00000000"}},"types":[{"id":0,"type":{"def":{"composite":{"fields":[{"type":1,"typeName":"[u8; 32]"}]}},"path":["ink_primitives","types","AccountId"]}},{"id":1,"type":{"def":{"array":{"len":32,"type":2}}}},{"id":2,"type":{"def":{"primitive":"u8"}}},{"id":3,"type":{"def":{"primitive":"u32"}}},{"id":4,"type":{"def":{"primitive":"bool"}}},{"id":5,"type":{"def":{"primitive":"u128"}}},{"id":6,"type":{"def":{"sequence":{"type":2}}}},{"id":7,"type":{"def":{"sequence":{"type":5}}}},{"id":8,"type":{"def":{"primitive":"u16"}}},{"id":9,"type":{"def":{"primitive":"u64"}}},{"id":10,"type":{"def":{"variant":{"variants":[{"fields":[{"type":11}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":11},{"name":"E","type":12}],"path":["Result"]}},{"id":11,"type":{"def":{"tuple":[]}}},{"id":12,"type":{"def":{"variant":{"variants":[{"index":1,"name":"CouldNotReadInput"}]}},"path":["ink_primitives","LangError"]}},{"id":13,"type":{"def":{"variant":{"variants":[{"fields":[{"type":14}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":14},{"name":"E","type":12}],"path":["Result"]}},{"id":14,"type":{"def":{"variant":{"variants":[{"fields":[{"type":2,"typeName":"u8"}],"index":0,"name":"U8"},{"fields":[{"type":8,"typeName":"u16"}],"index":1,"name":"U16"},{"fields":[{"type":3,"typeName":"u32"}],"index":2,"name":"U32"},{"fields":[{"type":9,"typeName":"u64"}],"index":3,"name":"U64"},{"fields":[{"type":5,"typeName":"u128"}],"index":4,"name":"U128"},{"fields":[{"type":6,"typeName":"Vec<u8>"}],"index":5,"name":"Bytes"}]}},"path":["psp34","types","Id"]}},{"id":15,"type":{"def":{"variant":{"variants":[{"fields":[{"type":3}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":3},{"name":"E","type":12}],"path":["Result"]}},{"id":16,"type":{"def":{"variant":{"variants":[{"fields":[{"type":17}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":17},{"name":"E","type":12}],"path":["Result"]}},{"id":17,"type":{"def":{"variant":{"variants":[{"index":0,"name":"None"},{"fields":[{"type":0}],"index":1,"name":"Some"}]}},"params":[{"name":"T","type":0}],"path":["Option"]}},{"id":18,"type":{"def":{"variant":{"variants":[{"index":0,"name":"None"},{"fields":[{"type":14}],"index":1,"name":"Some"}]}},"params":[{"name":"T","type":14}],"path":["Option"]}},{"id":19,"type":{"def":{"variant":{"variants":[{"fields":[{"type":4}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":4},{"name":"E","type":12}],"path":["Result"]}},{"id":20,"type":{"def":{"variant":{"variants":[{"fields":[{"type":21}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":21},{"name":"E","type":12}],"path":["Result"]}},{"id":21,"type":{"def":{"variant":{"variants":[{"fields":[{"type":11}],"index":0,"name":"Ok"},{"fields":[{"type":22}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":11},{"name":"E","type":22}],"path":["Result"]}},{"id":22,"type":{"def":{"variant":{"variants":[{"fields":[{"type":23,"typeName":"String"}],"index":0,"name":"Custom"},{"index":1,"name":"SelfApprove"},{"index":2,"name":"NotApproved"},{"index":3,"name":"TokenExists"},{"index":4,"name":"TokenNotExists"},{"index":5,"name":"ReachedMaxSupply"},{"fields":[{"type":23,"typeName":"String"}],"index":6,"name":"SafeTransferCheckFailed"},{"index":7,"name":"OutOfBoundsIndex"},{"index":8,"name":"NotAllowedToApprove"}]}},"path":["psp34","errors","PSP34Error"]}},{"id":23,"type":{"def":{"primitive":"str"}}},{"id":24,"type":{"def":{"variant":{"variants":[{"fields":[{"type":5}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":5},{"name":"E","type":12}],"path":["Result"]}},{"id":25,"type":{"def":{"sequence":{"type":26}}}},{"id":26,"type":{"def":{"tuple":[6,6]}}},{"id":27,"type":{"def":{"variant":{"variants":[{"fields":[{"type":28}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":28},{"name":"E","type":12}],"path":["Result"]}},{"id":28,"type":{"def":{"variant":{"variants":[{"index":0,"name":"None"},{"fields":[{"type":6}],"index":1,"name":"Some"}]}},"params":[{"name":"T","type":6}],"path":["Option"]}},{"id":29,"type":{"def":{"variant":{"variants":[{"fields":[{"type":18}],"index":0,"name":"Ok"},{"fields":[{"type":12}],"index":1,"name":"Err"}]}},"params":[{"name":"T","type":18},{"name":"E","type":12}],"path":["Result"]}},{"id":30,"type":{"def":{"composite":{"fields":[{"type":1,"typeName":"[u8; 32]"}]}},"path":["ink_primitives","types","Hash"]}},{"id":31,"type":{"def":{"variant":{}},"path":["ink_env","types","NoChainExtension"]}}],"version":"4"}
};

const query_address = "5GLdt2aR25oSS74wruX8UhzSktKoTtkprHaeUSCEty5qmuKf";

const CAs = {
    factory: "5GLdt2aR25oSS74wruX8UhzSktKoTtkprHaeUSCEty5qmuKf",
    staking: "5FDkRvAdRftKC36TkS3bSwjrXJYj6q1ic7oicvsRFTtGtgWE",
    chickens: "5GDZpX3CgL3PeKakoaWuAiqGbBPSqqigZs8NTXRxzaq2874T",
    foxes: "5GEbthiPsoYGUjcz2RbCknBj7ypSjCRmDoRmMTKtK89HsgRa"
};

export const getGas = (api) => { 
    return { 
        gasLimit: api.registry.createType('WeightV2', { 
            refTime:120000000000, 
            proofSize:99999999999, 
        }), 
        storageDepositLimit:99999999999 
    }; 
}

export const getFoxMints = async (api, account) => {
    if (!api || !account) {
        return 2;
    }
    let gas = getGas(api);
    let factory = new ContractPromise(api, ABIs.factory, CAs.factory);
    const mints_ = await factory.query["getDirectFoxMints"](query_address, gas, account.address);
    const numberReturn = doFormatNumber(mints_.output.toHuman().Ok);
    return numberReturn;
};

export const getMintPrices = async (api) => {
    if (!api) {
        return false;
    }
    let gas = getGas(api);
    let factory = new ContractPromise(api, ABIs.factory, CAs.factory);
    const direct_mints_ = await factory.query["getAzeroForDirectFoxMints"](query_address, gas);
    const random_mints_ = await factory.query["getAzeroForRandomMints"](query_address, gas);
    return [doFormatNumber(random_mints_.output.toHuman().Ok) / (10 ** 12), doFormatNumber(direct_mints_.output.toHuman().Ok) / (10 ** 12)];
};

export const getMintedNftCount = async (api) => {
    if (!api) {
        return;
    }
    let gas = getGas(api);
    let chickens_ = new ContractPromise(api, ABIs.PSP34, CAs.chickens);
    let foxes_ = new ContractPromise(api, ABIs.PSP34, CAs.foxes);
    const chickens = await chickens_.query["psp34::totalSupply"](query_address , gas);
    const foxes = await foxes_.query["psp34::totalSupply"](query_address , gas);
    const numberReturn = doFormatNumber(chickens.output.toHuman().Ok) + doFormatNumber(foxes.output.toHuman().Ok);
    return numberReturn;
};

export const getLastMint = async (api, account)=> {
    if (!api || !account) {
        return;
    }
    let gas = getGas(api);
    let factory = new ContractPromise(api, ABIs.factory, CAs.factory);
    const last_mint_ = await factory.query["getLastMintByAccount"](query_address, gas, account.address);
    const numberReturn = doFormatNumber(last_mint_.output.toHuman().Ok[0]);
    return numberReturn;
}

export const getTokenIdsForBoth = async (api, account, balances) => {
    if (!api || !account) {
        return;
    }
    let tokenIds = {
        foxes: [],
        chickens: []
    };
    let gas = getGas(api);
    let psp34ContractChickens = new ContractPromise(api, ABIs.PSP34, CAs.chickens);
    for (let i = 0; i < balances[0]; i++) {
        const tokenIdResponseChickens = await psp34ContractChickens.query["psp34Enumerable::ownersTokenByIndex"](query_address, gas, account.address, i.toLocaleString("fullwide", {useGrouping:false}));
        const tokenIdChickens = tokenIdResponseChickens.output.toHuman().Ok;
        tokenIds.chickens.push(tokenIdChickens);
    }
    let psp34ContractFoxes = new ContractPromise(api, ABIs.PSP34, CAs.foxes);
    for (let i = 0; i < balances[1]; i++) {
        const tokenIdResponseFoxes = await psp34ContractFoxes.query["psp34Enumerable::ownersTokenByIndex"](query_address, gas, account.address, i.toLocaleString("fullwide", {useGrouping:false}));
        const tokenIdFoxes = tokenIdResponseFoxes.output.toHuman().Ok;
        tokenIds.foxes.push(tokenIdFoxes);
    }
    return tokenIds;
};


export const stake = async (api, account, token_type) => {
    return new Promise(async (resolve, reject) => {
        if (!api || !account || !token_type) {
            return;
        }

        let gas = getGas(api);
        let contractAddress = token_type === 'foxes' ? CAs.foxes : CAs.chickens;
        let contract = new ContractPromise(api, ABIs.PSP34, contractAddress);
        let tokenIdToStake;
        try {
            const tokenIdResponse = await contract.query["psp34Enumerable::ownersTokenByIndex"](
                query_address, gas, account.address, "0"
            );
            tokenIdToStake = tokenIdResponse.output.toHuman().Ok;

            if (!tokenIdToStake) {
                
                throw new Error(`No ${token_type} token found to stake.`);
            }
        } catch (error) {
            reject(error.message);
            return;
        }

        let stakingContract = new ContractPromise(api, ABIs.staking, CAs.staking);
        let txName = token_type === 'foxes' ? "stakeFox" : "stakeChicken";

        await stakingContract.tx[txName](gas, tokenIdToStake).signAndSend(
            account.address,
            { signer: account.signer },
            async ({ events = [], status }) => {
                if (status.isInBlock) {
                    // Handle isInBlock status
                } else if (status.isFinalized) {
                    let failed = false;
                    events.forEach(({ event: { method } }) => {
                        if (method === "ExtrinsicFailed") {
                            failed = true;
                        }
                    });
                    if (failed) {
                        reject("Staking failed");
                    } else {
                        toastSuccess("Successful Stake!");
                        resolve("Staking successful");
                    }
                }
            }
        );
    });
};

export const unstake = async (api, account, token_type) => {
    return new Promise(async (resolve, reject) => {
        if (!api || !account || !token_type) {
            return;
        }

        let gas = getGas(api);
        let stakingContract = new ContractPromise(api, ABIs.staking, CAs.staking);
       
        let txName = token_type === 'foxes' ? "unstakeFoxes" : "unstakeChickens";

        let balancesBefore = await getBalances(api, account);
        let balanceBefore = balancesBefore[2];
        
        await stakingContract.tx[txName](gas).signAndSend(
            account.address,
            { signer: account.signer },
            async ({ events = [], status }) => {
                if (status.isInBlock) {
                    // Handle isInBlock status
                } else if (status.isFinalized) {
                    let failed = false;
                    events.forEach(({ event: { method } }) => {
                        if (method === "ExtrinsicFailed") {
                            failed = true;
                        }
                    });
                    if (failed) {
                        reject("Unstaking failed");
                    } else {
                        if (token_type == "foxes") {
                            toastSuccess("You have successfully unstaked your fox(es)!");
                            resolve("Unstaking successful");
                        }
                        else {
                            let balancesAfter = await getBalances(api, account);
                            let balanceAfter = balancesAfter[2];
                            if (balanceAfter > balanceBefore) {
                                toastSuccess(`Hurray! You have unstaked and claimed ${(balanceAfter - balanceBefore).toLocaleString(undefined, {maximumFractionDigits:18})} $AZERO`);
                            }
                            else {
                                if (balancesBefore[4] > 0) {
                                    toastError(`Ouch! All your layed $AZERO have been stolen!`);
                                }
                                else {
                                    toastSuccess("You've unstaked your chickens successfully!");
                                }
                            }
                            resolve("Unstaking successful");
                        }
                    }
                }
            }
        );
    });
};


export const PSP34_approve = (api, account,  token_type) => {
    return new Promise(async (resolve, reject)=> {
        if (!api || !account) {
            return;
        }
    
        let gas = getGas(api);
    
        let contract = new ContractPromise(api, ABIs.PSP34, CAs[token_type]);
    
        await contract.tx["psp34::approve"](gas, CAs.staking, null, true).signAndSend(
            account.address,
            { signer: account.signer },
            async ({ events = [], status }) => {
                if (status.isInBlock) {
                    //in block
                } else if (status.isFinalized) {
                    let failed = false;
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        if (method == "ExtrinsicFailed") {
                            failed = true;
                        }
                    });
                    if (failed == true) {
                        toastError();
                        reject("error");
                    }
                    else {
                        toastSuccess("Approval successful!");
                        resolve("success")
                    }
                }
            }
        );
    });
};

export const PSP34_allowance = async (api, account,  token_type) => {
    if (!api || !account) {
        return;
    }

    let gas = getGas(api);

    let contract = new ContractPromise(api, ABIs.PSP34, CAs[token_type]);
    
    let query_ = await contract.query["psp34::allowance"](query_address, gas, account.address, CAs.staking, null);
    let query = query_.output.toHuman().Ok;
    return query;
};

export const mint = async (api, account, type="random", amount)=> {
    if (!api || !account) {
        return; //Wallet and/or API not connected
    }

    return new Promise(async (resolve, reject)=> {
        if (type != "random") {
            let mints = await getFoxMints(api, account);
    
            if (mints == 2) {
                return;
            }
        }
    
        let gas = getGas(api);
        let contract = new ContractPromise(api, ABIs.factory, CAs.factory);
    
        amount = api.createType("Balance", amount.toLocaleString("fullwide", {useGrouping:false}));
        gas.value = amount;
    
        await contract.tx["mintNft"](gas).signAndSend(
            account.address,
            { signer: account.signer },
            async ({ events = [], status }) => {
                if (status.isInBlock) {
                    //in block
                } else if (status.isFinalized) {
                    let failed = false;
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        if (method == "ExtrinsicFailed") {
                            failed = true;
                        }
                    });
                    if (failed == true) {
                        toastError();
                        reject("Mint failed");
                    }
                    else {
                        let last_mint = await getLastMint(api, account);
                        let msg;
                        if (last_mint == 0) {
                            msg = "Kudos! You have successfully minted a chicken!";
                        }
                        else {
                            msg = "Way to go! You have successfully minted a fox!";
                        }
                        toastSuccess(msg);
                        resolve('Minted');
                    }
                }
            }
        );
    });
    
}

export const getStaked = async (api, account)=> {
    if (!api || !account) {
        return; //Wallet and/or API not connected
    }

    let gas = getGas(api);

    let contract = new ContractPromise(api, ABIs.staking, CAs.staking);
    let staked_chickens_ = await contract.query["getStakedChickens"](query_address, gas, account.address);
    let staked_chickens = staked_chickens_.output.toHuman().Ok.length;

    let staked_foxes_ = await contract.query["getStakedFoxes"](query_address, gas, account.address);
    let staked_foxes = staked_foxes_.output.toHuman().Ok.length;

    return [staked_chickens, staked_foxes];

}

export const getBalances = async (api, account)=> {
    if (!api || !account) {
        return; //Wallet and/or API not connected
    }

    let gas = getGas(api);
    let contract = new ContractPromise(api, ABIs.PSP34, CAs.chickens);
    let balance_ = await contract.query["psp34::balanceOf"](query_address, gas, account.address);
    let balance = doFormatNumber(balance_.output.toHuman().Ok);

    let contract2 = new ContractPromise(api, ABIs.PSP34, CAs.foxes);
    let balance_2 = await contract2.query["psp34::balanceOf"](query_address, gas, account.address);
    let balance2 = doFormatNumber(balance_2.output.toHuman().Ok);

    let { data: balance_3 } = await api.query.system.account(account.address);
    let balance3Raw = doFormatNumber(balance_3.free.toHuman());
    let balance3 = balance3Raw / 1e12;

    let contract4 = new ContractPromise(api, ABIs.staking, CAs.staking);
    let balance_4 = await contract4.query["getClaimableForFox"](query_address, gas, account.address);
    let balance4Raw = doFormatNumber(balance_4.output.toHuman().Ok);
    let balance4 = balance4Raw / 1e12;

    let contract5 = new ContractPromise(api, ABIs.staking, CAs.staking);
    let balance_5 = await contract5.query["getClaimableAzero"](query_address, gas, account.address);
    let balance5Raw = doFormatNumber(balance_5.output.toHuman().Ok);
    let balance5 = balance5Raw / 1e12;

    let balances = [balance, balance2, balance3, balance4, balance5];

    return balances;

}