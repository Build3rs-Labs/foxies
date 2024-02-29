import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Image from "next/image";
import HeaderCoop from "@/components/HeaderCoop";
import { ApiPromise, WsProvider } from '@polkadot/api';
import React, { useEffect, useState } from "react";
import { useWallet } from "useink";
import { getBalances, getTokenIdsForBoth, PSP34_approve, PSP34_allowance, getStaked, stake, unstake } from "../functions/index";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

import Typed from 'typed.js';

export default function Coop() {
  const el = React.useRef(null);
  const scrollSections = [0, 1, 2];
  const { account, connect, disconnect } = useWallet();
  const [balances, setBalances] = useState([0, 0, 0, 0, 0]);

  const [staked, setStaked] = useState([0, 0]);

  const [IDs, setIDs] = useState([]);
  const [isApproved, setIsApproved] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [isFoxApproved, setIsFoxApproved] = useState(false);

  useEffect(() => {
    const typed = new Typed(el.current, {
      strings: [`Here, you can stake your NFTs to earn $EGGS rewards!<br/>
      $EGGS can be used to mint more NFTs<br/>and increase your chances of getting a fox!<br/>
      They can also be sold for profit.`],
      typeSpeed: 10,
      showCursor: false
    });

    const timing = setTimeout(()=>{
      document.getElementsByClassName("farmer")[0].style.animation = 'exit 2s ease-in-out forwards';
      document.getElementsByClassName("coop-chicken")[0].style.animation = 'entrance 2s ease-in-out forwards';
    }, 8000);
  }, []);

  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 1000);

    return () => clearTimeout(timer);
  }, []);

  var api;
  var wsProvider;



  const handleApprove = async (animal) => {

    try {

      wsProvider = new WsProvider('wss://ws.test.azero.dev');
      api = await ApiPromise.create({ provider: wsProvider });
      await PSP34_approve(api, account, animal);
      const approvalStatus = await PSP34_allowance(api, account, animal);
      if (animal == "chickens") {
        setIsApproved(approvalStatus);
      }
      else {
        setIsFoxApproved(approvalStatus);
      }

    } catch (error) {
      toast.error("Failed: " + error);
    } 
  };

  const showElements = (type)=> {
    if (type == 0) {
      document.getElementsByClassName("coop-chicken")[0].style.animation = 'exit 0.3s ease-in-out forwards';
      document.getElementsByClassName("coop-fox")[0].style.animation = 'entrance 0.3s ease-in-out forwards';
      setTimeout(()=>{
        document.getElementsByClassName("coop-chicken")[0].style.display = 'none';
        document.getElementsByClassName("inner-body")[0].style.display = 'none';
        document.getElementsByClassName("inner-body")[1].style.display = 'inline-block';
        document.getElementsByClassName("coop-fox")[0].style.display = "flex";
      }, 300);
    }
    else {
      document.getElementsByClassName("coop-fox")[0].style.animation = 'exit 0.3s ease-in-out forwards';
      document.getElementsByClassName("coop-chicken")[0].style.animation = 'entrance 0.3s ease-in-out forwards';
      setTimeout(()=>{
        document.getElementsByClassName("coop-fox")[0].style.display = 'none';
        document.getElementsByClassName("inner-body")[1].style.display = 'none';
        document.getElementsByClassName("inner-body")[0].style.display = 'inline-block';
        document.getElementsByClassName("coop-chicken")[0].style.display = "flex";
      }, 300);
    }
  }

  const handleStake = async (animal) => {
    try {

      wsProvider = new WsProvider('wss://ws.test.azero.dev');
      api = await ApiPromise.create({ provider: wsProvider });
     
      const stakeStatus = await stake(api, account, animal);
     let result = await getBalances(api, account);
     setBalances(result);
     let staked = await getStaked(api, account);
     setStaked(staked);
    } catch (error) {
      toast.error("Failed: " + error);
    } 
  };
  
  const handleUnstake = async (animal) => {
    try {

      wsProvider = new WsProvider('wss://ws.test.azero.dev');
      api = await ApiPromise.create({ provider: wsProvider });
     
      const stakeStatus = await unstake(api, account, animal);
     let result = await getBalances(api, account);
     setBalances(result);
     let staked = await getStaked(api, account);
     setStaked(staked);
    } catch (error) {
      toast.error("Failed: " + error);
    } 
  };
  useEffect(() => {
    if (account) {
      setIsLoading(true);
      const call = async () => {
        wsProvider = new WsProvider('wss://ws.test.azero.dev');
        api = await ApiPromise.create({ provider: wsProvider });

        let result = await getBalances(api, account);

        let staked = await getStaked(api, account);
        setStaked(staked);

        let approvalStatus = await PSP34_allowance(api, account, 'chickens');
        setIsApproved(approvalStatus);
        setBalances(result);
        let foxApprovalStatus = await PSP34_allowance(api, account, 'foxes'); 

        setIsFoxApproved(foxApprovalStatus);

        setIsLoading(false);
      };
      call();
    }
  }, [account]);
  
  const renderStakeButtons = (animalType) => {

    const isAnimalApproved = animalType === "chickens" ? isApproved : isFoxApproved;
  
    if (isLoading === true) {
      return <p className="text-center text-3xl text-white mt-3">Loading...</p>;
    }
  
    if (isAnimalApproved) {
      return (
        <>
          <button onClick={()=>handleStake(animalType)} className="relative mx-auto mt-4 border-2 border-black bg-white rounded-full text-2xl lg:text-3xl text-black px-4 flex items-center">
            <span className="relative font-VT323">
              Stake now 
              <span className="ml-2">
                &gt;
              </span>
            </span>
          </button>
          <button onClick={()=>handleUnstake(animalType)} className="relative mx-auto mt-4 border-2 border-black bg-white rounded-full text-2xl lg:text-3xl text-black px-4 flex items-center">
            <span className="relative font-VT323">
              Unstake
              <span className="ml-2">
                &gt;
              </span>
            </span>
          </button>
        </>
      );
    } else {
      return (
        <button onClick={()=>handleApprove(animalType)} className="relative mx-auto mt-8 border-2 border-black bg-white rounded-lg text-2xl lg:text-3xl text-black px-8 flex items-center">
          <span className="relative font-VT323">Approve</span>
        </button>
      );
    }
  };
  

  
  

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

            <div className="farmer">
                <div className="">
                  <div
                    style={{padding:30, fontSize:20}}
                    className={`max-w-[100%] px-10 py-6 mr-2 lg:mr-0
                    text-center rounded-full bubble-up ${styles.bubble} ${styles["bubble-bottom-left"]}`}
                  >
                    <div ref={el}></div>
                  </div>
                </div>
                <div className="bottom-20 lg:right-1/2">
                  <Image
                    src="/farmer2.png"
                    width={150}
                    height={600}
                    alt="logo"
                    className="mx-4"
                  />
                </div>
            </div>

            <div className="coop-chicken">

              <div className="inner-body">
                <div className={`flex items-center pt-2 flex-col	transition-opacity duration-1000`}
                  >
                    <div className="flex mx-6">
                        <img src="/chicken.png" alt="logo" className="mx-4 nft-icons"/>
                    </div>
                    
                    <span className="relative text-white text-2xl mt-4 font-VT323">You own {balances[0] + staked[0]} {(balances[0] + staked[0] === 1) ? "chicken" : "chickens"}. Chickens staked: {staked[0]}</span>

                    {!account ? <p className="text-white text-3xl pt-2">First, connect your wallet</p> : renderStakeButtons("chickens")}
                    <button className="relative mx-auto mt-3 border-2 border-black bg-white rounded-lg text-2xl lg:text-3xl text-black px-8 flex items-center">
                      <span className="relative font-VT323">Your eggs balance is {balances[2].toLocaleString()} $EGGS.</span>
                    </button>
                </div>
                
              </div>

              <button className="out-of-the-box" onClick={()=>showElements(0)}>
                <img src="/down-arrows.png" className="arrows"/>
              </button>
              
            </div>

            <div className="coop-fox">
              <button className="out-of-the-box" onClick={()=>showElements(1)}>
                <img src="/up-arrows.png" className="arrows"/>
              </button>
              <div className="inner-body">
                <div className={`flex items-center pt-2 flex-col transition-opacity duration-1000`}
                  >
                    <div className="flex mx-6">
                        <img src="/fox.png" alt="logo" className="mx-4 nft-icons"/>
                    </div>
                    
                    <span className="relative text-white text-2xl mt-4 font-VT323">You own {balances[1] + staked[1]} {(balances[1] + staked[1] === 1) ? "fox" : "foxes"}. Foxes staked: {staked[1]}</span>

                    {!account ? <p className="text-white text-3xl pt-2">First, connect your wallet</p> : renderStakeButtons("foxes")}
                    <button className="relative mx-auto mt-3 border-2 border-black bg-white rounded-lg text-2xl lg:text-3xl text-black px-8 flex items-center">
                      <span className="relative font-VT323">Your eggs balance is {balances[2].toLocaleString()} $EGGS.</span>
                    </button>
                </div>
                
              </div>
              
            </div>
          </div>

        <ToastContainer />
      </div>
    </>
  );
}
