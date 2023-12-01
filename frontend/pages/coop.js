import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Image from "next/image";
import Header from "@/components/Header";
import { ApiPromise, WsProvider } from '@polkadot/api';
import { randomAsU8a } from '@polkadot/util-crypto';
import React, { useEffect, useState } from "react";
import { useWallet } from "useink";
import { formatWallet, CallContract, getBalances, getTokenIdsForBoth} from "../functions/index";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

export default function Coop() {

  const { account, connect, disconnect } = useWallet();
  const [balances, setBalances] = useState(['X', 'X', 'X']);
  const [IDs, setIDs] = useState([]);
  var api;
  var wsProvider;

  useEffect(() => {
    const call = async () => {
      wsProvider = new WsProvider('wss://ws.test.azero.dev');
      api = await ApiPromise.create({ provider: wsProvider });
      let result = await getBalances(api, account);
      let balancesParam = [result[0], result[1]];
      let result2 = await getTokenIdsForBoth(api, account, balancesParam);
      setBalances(result)
      setIDs(result2);
    };
  
    console.log(account);
    if (account) {
      call();
    }
  }, [account]);
  

  return (
    <>
      <Head>
        <title>Crypto</title>
        <meta name="description" content="Generated by create next app" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div>
        <div className={styles.pageBackground}></div>
        <Header />
        <div className="absolute z-40 w-full h-full top-0">
         
            <h1 className="pt-20 font-VT323 text-white text-5xl lg:text-7xl text-center">
            You're in the coop !
            </h1>
         
          <div className="pt-14 lg:28 grid grid-cols-1 md:grid-cols-3 lg:grid-flow-row gap-6 font-VT323 text-white text-2xl lg:text-4xl mx-4 lg:mx-16 lg:leading-10">
            <div className="p-4">
            You own {balances[0]} chickens.
            <p className="pt-12">Stake your NFTs to earn delicious $EGGS rewards.</p>
            <button  className="relative mx-auto mt-8 border-2  border-black bg-white rounded-full text-2xl lg:text-4xl text-black px-4 flex items-center">
                <span className="relative font-VT323">Stake Chickens</span>
              </button>
            </div>
            <div className=" p-4 text-center	">
             You own {balances[1]} foxes.
             <p className="pt-12">Stake your NFTs to try to steal the precious $EGGS</p>
          {/*   <button className=" border-2 border-black bg-white rounded-full text-xl text-black px-12 flex items-center">
                MINT
              </button> */}  
              <button  className="relative mx-auto mt-8 border-2  border-black bg-white rounded-full text-2xl lg:text-4xl text-black px-4 flex items-center">
                <span className="relative font-VT323">Stake Foxes</span>
              </button>
            </div>
            <div className="p-4 text-center">
            Must read before staking !
            <Image className="mx-auto" src="/book.png" width={160} height={160} />
            <p className="py-8">Your $EGGS balance : <br />{balances[2]} $EGGS</p>
            <Image className="mx-auto" src="/egg.png" width={140} height={140} />
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
