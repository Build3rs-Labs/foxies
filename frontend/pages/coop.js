import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Image from "next/image";
import HeaderCoop from "@/components/HeaderCoop";
import { ApiPromise, WsProvider } from '@polkadot/api';
import React, { useEffect, useState } from "react";
import { useWallet } from "useink";
import { getBalances, PSP34_approve, PSP34_allowance, getStaked, stake, unstake, shuffle } from "../functions/index";
import { ToastContainer, toast } from "react-toastify";
import Link from "next/link";
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

  const [ unstakedNFTs, setUnstakedNFTs ] = useState([]);

  const [ stakedNFTs, setStakedNFTs ] = useState([]);

  const dosetBalances = (balances_)=> {
    setBalances(balances_);
    let chickens = Array.from({length:balances_[0]}, ()=>0);
    let foxes = Array.from({length:balances_[1]}, ()=>1);
    let nfts = shuffle(chickens.concat(foxes));
    setUnstakedNFTs(nfts);
  }

  const dosetStaked = (staked_)=> {
    setStaked(staked_);
    let chickens = Array.from({length:staked_[0]}, ()=>0);
    let foxes = Array.from({length:staked_[1]}, ()=>1);
    let nfts = shuffle(chickens.concat(foxes));
    setStakedNFTs(nfts);
  }

  useEffect(() => {
    const typed = new Typed(el.current, {
      strings: [`Here, you can stake your NFTs to earn $AZERO rewards!<br/>
      $AZERO can be used to mint more NFTs<br/>and increase your chances of getting a fox!<br/>
      They can also be sold for profit.`],
      typeSpeed: 10,
      showCursor: false
    });
  }, []);

  const toggleInfo = ()=> {
    document.getElementsByClassName("farmer")[0].style.animation = 'exit 2s ease-in-out forwards';
    document.getElementsByClassName("coop-chicken")[0].style.display = 'flex';
    document.getElementsByClassName("coop-fox")[0].style.display = 'flex';
    document.getElementsByClassName("coop-chicken")[0].style.animation = 'entrance 2s ease-in-out forwards';
    document.getElementsByClassName("farmer-right-alt")[0].style.animation = 'exit 2s ease-in-out forwards';
    document.getElementsByClassName("mobile")[0].style.animation = 'exit 2s ease-in-out forwards';
    setTimeout(() => {
      document.getElementsByClassName("mobile")[0].remove();
    }, 2000);
  }

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

    let allowedToStake = true;

    if (animal == "foxes") {
      if (staked[1] >= 5) {
        allowedToStake = false;
      }
    }
    else {
      if (staked[0] >= 5) {
        allowedToStake = false;
      }
    }
    
    if (allowedToStake == false) {
      return false;
    }

    try {

      wsProvider = new WsProvider('wss://ws.test.azero.dev');
      api = await ApiPromise.create({ provider: wsProvider });
     
      const stakeStatus = await stake(api, account, animal);
     let result = await getBalances(api, account);
     dosetBalances(result);
     let staked = await getStaked(api, account);
     dosetStaked(staked);
    } catch (error) {
      toast.error("Failed: " + error);
    } 
  };
  
  const handleUnstake = async (animal) => {

    let allowedToUnstake = true;

    if (animal == "foxes") {
      if (staked[1] == 0) {
        allowedToUnstake = false;
      }
    }
    else {
      if (staked[0] == 0) {
        allowedToUnstake = false;
      }
    }
    
    if (allowedToUnstake == false) {
      return false;
    }

    try {

      wsProvider = new WsProvider('wss://ws.test.azero.dev');
      api = await ApiPromise.create({ provider: wsProvider });
     
      const stakeStatus = await unstake(api, account, animal);
     let result = await getBalances(api, account);
     dosetBalances(result);
     let staked = await getStaked(api, account);
     dosetStaked(staked);
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
        dosetStaked(staked);

        let approvalStatus = await PSP34_allowance(api, account, 'chickens');
        setIsApproved(approvalStatus);
        dosetBalances(result);
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
        <div style={{display:"flex", justifyContent:"space-around"}}>
          <button onClick={()=>handleStake(animalType)} className="relative mx-2 mt-4 border-2 border-black bg-white rounded-full text-2xl lg:text-3xl text-black px-4 flex items-center">
            <span className="relative font-VT323">
              Stake now 
              <span className="ml-2">
                &gt;
              </span>
            </span>
          </button>
          <button onClick={()=>handleUnstake(animalType)} className="unstake-btn relative mx-2 secondary mt-4 border-2 border-black bg-white rounded-full text-2xl lg:text-3xl text-black px-4 flex items-center">
            <span className="relative font-VT323">
              Unstake{(animalType == "foxes")?
                <>
                  {(balances[3] == 0)?
                  null:
                  <div className="claimable-btn">{balances[3].toLocaleString(undefined, {maximumFractionDigits:12})}</div>
                  }
                </>
                :
                <>
                  {(balances[4] == 0)?
                  null:
                  <div className="claimable-btn">{balances[4].toLocaleString(undefined, {maximumFractionDigits:12})}</div>
                  }
                </>
                }
              <span className="ml-2">
                &gt;
              </span>
            </span>
          </button>
        </div>
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
                  className="mx-4 farmer-img"
                />
              </div>
            </div>

            <div className="farmer-right-alt">
              <div className="bottom-20 lg:right-1">
                <div className="wooden">
                  <span className="text-2xl">STAKED</span>
                  <div>
                    {(stakedNFTs.length == 0)?
                    <div style={{height:30}}>...</div>:
                    <div style={{display:"inline-block"}}>
                      {(stakedNFTs.map((value, index)=>{
                        return <div key={index} style={{padding:5}} className="flexicon">
                          <img src={`${(value == 0)?'/chicken-icon.png':'/fox-icon.png'}`} className="icon-nfts"/>
                        </div>
                      }))}
                    </div>}
                  </div>
                  <span className="text-2xl">UNSTAKED</span>
                  <div>
                    {(unstakedNFTs.length == 0)?
                    <div style={{height:50}}>...</div>:
                    <div style={{display:"inline-block"}}>
                      {(unstakedNFTs.map((value, index)=>{
                        return <div key={index} style={{padding:5}} className="flexicon">
                        <img src={`${(value == 0)?'/chicken-icon.png':'/fox-icon.png'}`} className="icon-nfts"/>
                      </div>
                      }))}
                    </div>}
                  </div>
                </div>
                
                <div className="px-3">
                  <button onClick={()=>toggleInfo()} style={{width:"100%"}} className="border-[2px] my-5 secondary border-black rounded-full text-2xl text-black px-4">
                    <span className=" font-VT323">
                      Stake my NFTs now!
                    </span>
                  </button>
                </div>

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

                    <button className="relative mx-auto mt-3 mb-3 w-100 border-2 border-black bg-white rounded-full text-2xl lg:text-3xl text-black px-8 flex items-center">
                      <span className="relative font-VT323 flexy">Your balance is {balances[2].toLocaleString()} $AZERO <img src="/azero.png" className=" mx-1 azero-ticker-large"/></span>
                    </button>

                    <Link href="/lastnight">
                      <button className="relative mx-auto mt-4 border-2 border-black bluey rounded-full text-2xl lg:text-3xl text-black px-8 flex items-center">
                        <span className="relative font-VT323 flexy">What happened last night?</span>
                      </button>
                    </Link>

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

                    <button className="relative mx-auto mt-3 mb-3 w-100 border-2 border-black bg-white rounded-full text-2xl lg:text-3xl text-black px-8 flex items-center">
                      <span className="relative font-VT323 flexy">Your balance is {balances[2].toLocaleString()} $AZERO <img src="/azero.png" className=" mx-1 azero-ticker-large"/></span>
                    </button>

                    <Link href="/lastnight">
                      <button className="relative mx-auto mt-4 border-2 border-black bluey rounded-full text-2xl lg:text-3xl text-black px-8 flex items-center">
                        <span className="relative font-VT323 flexy">What happened last night?</span>
                      </button>
                    </Link>

                </div>
                
              </div>
              
            </div>

            <div style={{position:"fixed", width:"100vw", bottom:0, left:0, zIndex: 5000}} className="mobile">
              <button onClick={()=>toggleInfo()} className="mx-auto border-[2px] my-5 border-black secondary rounded-full text-2xl text-black px-4 flex items-center">
                <span className=" font-VT323">
                  Stake my NFTs now!
                </span>
              </button>
            </div>

          </div>

        <ToastContainer />
      </div>
    </>
  );
}
