import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Header from "@/components/Header";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { randomAsU8a } from "@polkadot/util-crypto";
import React, { useEffect, useState, useRef } from "react";
import { useWallet } from "useink";
import { mint, getMintedNftCount, getFoxMints, getMintPrices, gameStats } from "../functions/index";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import Image from "next/image";

import Typed from 'typed.js';

export default function Mint() {
  const { account } = useWallet();
  const scrollSections = [0, 1];
  const backgroundStyle = {
    backgroundSize: "cover",
    backgroundPosition: "center center",
    backgroundAttachment: "fixed",
    backgroundImage: 'url("/mint.jpg")', // Replace with the actual image path

    minHeight: "100vh",
  };
  
  const [nftLeft, setNftLeft] = useState('X');

  const [foxMints, setFoxMints] = useState(0);

  const [ stats, setGameStats ] = useState([0, 0, 0, 0, 0]);

  useEffect(() => {
    const typed = new Typed(el.current, {
      strings: [`Welcome to the world of Foxies! 🌎
      <br/>
      Chickens peacefully produce $AZERO 🥚
      <br/>
      Foxes are sneaky, they steal $AZERO 🥚`],
      typeSpeed: 10,
      showCursor: false
    });

    const timing = setTimeout(()=>{
      try {
        document.getElementsByClassName("farmer")[0].style.animation = 'exit 2s ease-in-out forwards';
        document.getElementsByClassName("farmer-right")[0].style.animation = 'exit 2s ease-in-out forwards';
        document.getElementsByClassName("minter")[0].style.animation = 'entrance 2s ease-in-out forwards';
        document.getElementsByClassName("learn")[0].style.animation = 'exit 2s ease-in-out forwards';
        let timeout = setTimeout(()=>{
          document.getElementsByClassName("learn")[0].remove();
          clearTimeout(timeout);
        }, 2000);
      }
      catch (error) {}
    }, 10000);

    return () => {
      // Destroy Typed instance during cleanup to stop animation
      typed.destroy();
      clearTimeout(timing);
    };
  }, [account]);

  useEffect(() => {
    
  }, [account]);

  const [api, setAPI] = useState(null);

  const [ mintPrice, setMintPrice ] = useState([0, 0]);

  const el = React.useRef(null);

  const handleMint = async (api, account, type) => {
    if (nftLeft == 0) {
      return false;
    }
    await mint(api, account, type, (type == "random")?mintPrice[0] * (10 ** 12):mintPrice[1] * (10 ** 12));
    const nftCountValue = await getMintedNftCount(api);
    setNftLeft(15000 - nftCountValue);
    let mints = await getFoxMints(api, account);
    setFoxMints(mints);
  };

  useEffect(() => {
    let connect = async () => {

      let wsProvider = new WsProvider("wss://ws.test.azero.dev");
      let _api = await ApiPromise.create({ provider: wsProvider });

      setAPI(_api);

      const status = await gameStats(_api);

      setGameStats(status);

      const nftCountValue = await getMintedNftCount(_api);

      setNftLeft(15000 - nftCountValue);

      let mintCosts = await getMintPrices(_api);

      setMintPrice(mintCosts);

      let mints = await getFoxMints(_api, account);
      setFoxMints(mints);
      
    };
    connect();
  }, [account]);

  return (
    <>
      <Head>
        <title>Foxies</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Header />

      <div className="font-VT323" style={{fontSize:12}}>
        <div className={styles.pageBackground}></div>

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
                    src="/farmer1.png"
                    width={150}
                    height={600}
                    alt="logo"
                    className="mx-4"
                  />
                </div>
            </div>

            <div className="farmer-right">
                <div className="">
                  <div
                    style={{padding:'20px 30px 20px 30px', fontSize:20}}
                    className={`game-stats`}
                  >
                    <center>
                      <div className="game-stats-text centralized underline w-100">Game Status:</div>
                      <div className="mt-3">
                        <div className="game-stats-text">
                          <span className="underline me-2">{stats[0].toLocaleString()}</span> NFTs Minted
                        </div>
                        <div className="game-stats-text mt-1">
                          <span className="underline me-2">{stats[1].toLocaleString()}</span> Foxes Minted
                        </div>
                        <div className="game-stats-text mt-1">
                          <span className="underline me-2">{stats[2].toLocaleString()}</span> Chickens Minted
                        </div>
                        <div className="game-stats-text mt-1">
                          <span className="underline me-2">{stats[3].toLocaleString()}</span> $AZERO <img src="/azero.png" className=" mx-1 azero-ticker-small"/> Traded
                        </div>
                        <div className="game-stats-text mt-1">
                          <span className="underline me-2">{stats[4].toLocaleString()}</span> $AZERO <img src="/azero.png" className=" mx-1 azero-ticker-small"/> Claimed
                        </div>
                      </div>
                    </center>
                  </div>
                </div>
                <div className="bottom-20 lg:right-1/2">
                  <Image
                    src="/farmer3.png"
                    width={150}
                    height={600}
                    alt="logo"
                    className="mx-4 farmer-img"
                  />
                </div>
            </div>

            <div className="minter">
              <div
                  className={`w-full h-full flex items-center justify-center flex-col	 transition-opacity duration-1000`}
                >
                  <div className="flex mx-6">
                    <div className="mx-2">
                      <img src="/chicken.png" className={"nft-icons"}  alt="logo"/>
                    </div>
                    <div className="mx-2">
                      <img src="/fox.png" className={"nft-icons"} alt="logo"/>
                    </div>
                  </div>

                  <div className="loader mt-5 pt-4">
                    <img src="/heart.png" className="heart"/>
                    <div className="loader-bar pixel-corners--wrapper">
                      <div className="loader-progress pixel-corners--wrapper"
                      style={{width:`${((15000 - nftLeft) / 15000) * 100}%`}}></div>
                    </div>
                  </div>

                  <h1 className="text-white text-2xl mt-1 pb-3">{nftLeft.toLocaleString()} NFTs left</h1>

                  <span className="d-block mt-4 mb-3" style={{color:'#FFFFFF', fontSize:16}}>
                    ⓘ Random mints cost {mintPrice[0]} AZERO and Direct fox mints cost {mintPrice[1]} AZERO
                  </span>
                  <div className="flex z-10">
                    <button onClick={()=>handleMint(api, account, "random")} className="mx-2  border-[2px] border-black bg-white rounded-full text-2xl lg:text-3xl  text-black px-4 py-1 flex items-center">
                      <span className=" font-VT323">
                        Random mint!
                        <span className="ml-1">
                          &gt;
                        </span>
                      </span>
                    </button>
                    <button style={(foxMints < 2 && foxMints != null)?{backgroundColor:"#EDBF8E", cursor:"pointer"}:{backgroundColor:"#EDBF8E99", cursor:"not-allowed"}} onClick={()=>(foxMints < 2)?handleMint(api, account, "foxes"):null} className="mx-2  border-[2px] border-black bg-white rounded-full text-2xl lg:text-3xl  text-black px-4 py-1 flex items-center">
                      <span className=" font-VT323">
                        Mint Fox!
                        <span className="ml-1">
                          &gt;
                        </span>
                      </span>
                    </button>
                  </div>
                </div>
            </div>

            <div style={{position:"fixed", width:"100vw", bottom:0, left:0, zIndex: 1000}} className="learn">
              <a target="_blank" rel="noreferrer noopener" href="https://medium.com/@foxiesgame/the-foxies-game-on-aleph-zero-3aed13c1b8b5">
                <button className="mx-auto border-[2px] my-5 border-black secondary rounded-full text-2xl text-black px-4 flex items-center">
                  <span className=" font-VT323">
                    Learn more about the game mechanics
                  </span>
                </button>
              </a>
            </div>
          </div>

        <ToastContainer />
      </div>
    </>
  );
}
