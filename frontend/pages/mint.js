import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Header from "@/components/Header";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { randomAsU8a } from "@polkadot/util-crypto";
import React, { useEffect, useState, useRef } from "react";
import { useWallet } from "useink";
import { formatWallet, CallContract, mint, getMintedNftCount } from "../functions/index";
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
    }, 6000);

    return () => {
      // Destroy Typed instance during cleanup to stop animation
      typed.destroy();
      clearTimeout(timing);
    };
  }, []);

  useEffect(() => {
    
  }, []);

  const [api, setAPI] = useState(null);

  const el = React.useRef(null);

  const handleMint = async (api, account, type) => {
    await mint(api, account, type);
    const nftCountValue = await getMintedNftCount(api);
    console.log(nftCountValue + " nftCountValue")
    setNftLeft(12000 - nftCountValue);
  };

  useEffect(() => {
    let connect = async () => {
      let wsProvider = new WsProvider("wss://ws.test.azero.dev");
      let _api = await ApiPromise.create({ provider: wsProvider });
      setAPI(_api);
      const nftCountValue = await getMintedNftCount(_api);
      console.log(nftCountValue + " nftCountValue")
      setNftLeft(12000 - nftCountValue);
    };
    connect();
  }, []);

  return (
    <>
      <Head>
        <title>Foxies</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className="w-full font-VT323" style={{fontSize:12}}>
        <div className={styles.pageBackground}></div>

        <div className="w-full  h-[100dvh] sm:h-[100vh] fixed bottom-[0%] z-50">

          <Header />


            <div className="farmer">
                <div className="flex items-center justify-start lg:left-1/2 left-[30%] top-10 lg:top-20 ml-5">
                  <div
                    style={{padding:30, fontSize:20}}
                    className={`max-w-[650px] px-10 py-6 mr-2 lg:mr-0
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
                  <h1 className="text-white text-4xl pt-8">{nftLeft} NFTs left</h1>
                  <div className="flex z-10">
                    <button  onClick={()=>handleMint(api, account, "random")} className="mx-2  border-[2px] border-black bg-white rounded-lg text-2xl lg:text-4xl  text-black px-4 flex items-center">
                      <span className=" font-VT323">Random mint</span>
                    </button>
                    <button  onClick={()=>handleMint(api, account, "fox")} className="mx-2  border-[2px] border-black bg-white rounded-lg text-2xl lg:text-4xl  text-black px-4 flex items-center">
                      <span className=" font-VT323">Fox mint</span>
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
