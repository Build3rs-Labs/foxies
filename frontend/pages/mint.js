import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Header from "@/components/Header";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { randomAsU8a } from "@polkadot/util-crypto";
import React, { useEffect, useState, useRef } from "react";
import { useWallet } from "useink";
import { formatWallet, CallContract, mint, getMintedNftCount, getFoxMints, getLastMint } from "../functions/index";
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

  useEffect(() => {
    const typed = new Typed(el.current, {
      strings: [`Welcome to the world of Foxies! 🌎
      <br/>
      Chickens peacefully produce $EGGS 🥚
      <br/>
      Foxes are sneaky, they steal $EGGS 🥚`],
      typeSpeed: 10,
      showCursor: false
    });

    const timing = setTimeout(()=>{
      document.getElementsByClassName("farmer")[0].style.animation = 'exit 2s ease-in-out forwards';
      document.getElementsByClassName("minter")[0].style.animation = 'entrance 2s ease-in-out forwards';
    }, 7000);

    return () => {
      // Destroy Typed instance during cleanup to stop animation
      typed.destroy();
      clearTimeout(timing);
    };
  }, [account]);

  useEffect(() => {
    
  }, [account]);

  const [api, setAPI] = useState(null);

  const el = React.useRef(null);

  const handleMint = async (api, account, type) => {
    await mint(api, account, type);
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
      const nftCountValue = await getMintedNftCount(_api);

      setNftLeft(15000 - nftCountValue);

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
                  <h1 className="text-white text-4xl pt-8 pb-6">{nftLeft.toLocaleString()} NFTs left</h1>
                  <div className="flex z-10">
                    <button  onClick={()=>handleMint(api, account, "random")} className="mx-2  border-[2px] border-black bg-white rounded-lg text-2xl lg:text-3xl  text-black px-4 py-1 flex items-center">
                      <span className=" font-VT323">
                        Random mint!
                        <span className="ml-1">
                          &gt;
                        </span>
                      </span>
                    </button>
                    <button style={(foxMints < 2 && foxMints != null)?{backgroundColor:"#FFFFFF", cursor:"pointer"}:{backgroundColor:"#A2A2A2", cursor:"not-allowed"}} onClick={()=>(foxMints < 2)?handleMint(api, account, "foxes"):null} className="mx-2  border-[2px] border-black bg-white rounded-lg text-2xl lg:text-3xl  text-black px-4 py-1 flex items-center">
                      <span className=" font-VT323">
                        Mint a fox!
                        <span className="ml-1">
                          &gt;
                        </span>
                      </span>
                    </button>
                  </div>
                </div>
            </div>

            <div style={{position:"fixed", width:"100vw", bottom:0, left:0, zIndex: 1000}}>
              <a target="_blank" rel="noreferrer noopener" href="https://medium.com/@foxiesgame/the-foxies-game-on-aleph-zero-3aed13c1b8b5">
                <button className="mx-auto border-[2px] my-5 border-black bg-white rounded-lg text-2xl text-black px-4 flex items-center">
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
