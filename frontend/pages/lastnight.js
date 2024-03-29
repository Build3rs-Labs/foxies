import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Image from "next/image";
import HeaderCoop from "@/components/HeaderCoop";
import { ApiPromise, WsProvider } from '@polkadot/api';
import React, { useEffect, useState } from "react";
import { useWallet } from "useink";
import { getBalances, getLastStolenFromChicken, getStaked, getLastStolenForFox  } from "../functions/index";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

export default function Coop() {

  const { account, connect, disconnect } = useWallet();

  var api;
  var wsProvider;

  const [ loading, setIsLoading ] = useState(true);

  const [balances, setBalances] = useState([0, 0, 0, 0, 0]);

  const [staked, setStaked] = useState([0, 0]);

  const [stolenForFox, setStolenForFox] = useState([0, 0]);

  const [stolenFromChicken, setStolenFromChicken] = useState([0, 0]);

  var api;
  var wsProvider;

  useEffect(() => {
    if (account) {

      setIsLoading(true);

      const call = async () => {

        wsProvider = new WsProvider('wss://ws.test.azero.dev');
        api = await ApiPromise.create({ provider: wsProvider });

        let result = await getBalances(api, account);

        let staked = await getStaked(api, account);

        let laststolenfromchicken = await getLastStolenFromChicken(api, account);

        setStolenFromChicken(laststolenfromchicken);

        let laststolenforfox = await getLastStolenForFox(api, account);

        setStolenForFox(laststolenforfox);

        setStaked(staked);
        setBalances(result);

      };
      call();
    }
  }, [account, stolenForFox, stolenFromChicken]);

  return (
    <>
      <Head>
        <title>Foxies</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <HeaderCoop />

      <div className="font-VT323" style={{fontSize:12}}>
        <div className={styles.pageBackground2}></div>

        <div className="">

          <div className="lastnight">
            <div className="flex p-5 mt-5 block-parent">
              <div className="w-1/3 block-small">
                <center>
                  <span className="relative text-white text-2xl mt-4 font-VT323">You own {balances[0] + staked[0]} {(balances[0] + staked[0] === 1) ? "chicken" : "chickens"}</span>
                  <img src="/chicken.png" alt="logo" className="nft-icons mt-5 not-mobile"/>
                  <div className="msg text-xl mt-5" style={{marginBottom:100}}>
                    Last night, the chickens staked in your farm produced {balances[4].toLocaleString(undefined, {maximumFractionDigits:12})} $AZERO.
                    {(stolenFromChicken[1] > 0)?
                    <>
                    <br/>
                    Unfortunately, the sneaky foxes stole {stolenFromChicken[1].toLocaleString(undefined, {maximumFractionDigits:12})} $AZERO from you.
                    </>
                    :null
                    }
                  </div>
                </center>
              </div>
              <div style={{marginBottom:80}} className="w-1/3 block-small">
                <center>
                  <img src="/recap.png" style={{width:300, maxWidth:"90%"}}/>
                  <button className="relative mx-auto mt-3 mb-3 w-100 border-2 border-black bg-white rounded-full text-2xl lg:text-2xl text-black px-4 flex items-center">
                    <span className="relative text-xl flexy">Your balance is {balances[2].toLocaleString()} $AZERO <img src="/azero.png" className=" mx-1 azero-ticker-large"/></span>
                  </button>
                </center>
              </div>
              <div className="w-1/3 block-small">
                <center>
                  <span className="relative text-white text-2xl mt-4 font-VT323">You own {balances[1] + staked[1]} {(balances[1] + staked[1] === 1) ? "fox" : "foxes"}</span>
                  <img src="/fox.png" alt="logo" className="nft-icons mt-5 not-mobile"/>
                  <div className="msg text-xl mt-5" style={{marginBottom:100}}>
                    Last night, the foxes staked in your farm produced {balances[3].toLocaleString(undefined, {maximumFractionDigits:12})} $AZERO.
                    {(stolenForFox[1] > 0)?
                    <>
                    <br/>
                    Unfortunately, the sneaky foxes stole {stolenForFox[1].toLocaleString(undefined, {maximumFractionDigits:12})} $AZERO from you.
                    </>
                    :null
                    }
                  </div>
                </center>
              </div>
            </div>
          </div>
        </div>

        <ToastContainer />
      </div>
    </>
  );
}
