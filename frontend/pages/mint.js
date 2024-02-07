import Head from "next/head";
import styles from "@/styles/Home.module.css";
import Header from "@/components/Header";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { randomAsU8a } from "@polkadot/util-crypto";
import React, { useEffect, useState } from "react";
import { useWallet } from "useink";
import { formatWallet, CallContract, mint, getMintedNftCount } from "../functions/index";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import Image from "next/image";

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
  const [scrollPos, setScrollPos] = useState(0);
  const [currentSection, setCurrentSection] = useState(scrollSections[0]);
  const [sectionIndex, setSectionIndex] = useState(0);
  const [opacity, setOpacity] = useState(1);
  const [nftLeft, setNftLeft] = useState('X');

  const handleScroll = () => {
    if (window.scrollY > 0) {
      const windowHeight = window.innerHeight;
      const scrollHeight = Math.max(
        1,
        document.documentElement.scrollHeight - windowHeight
      );

      const scrollPercentage = (window.scrollY / scrollHeight) * 100;

      const minScrollPos = 0;
      const newScrollPos = Math.max(minScrollPos, scrollHeight);

      setScrollPos(newScrollPos);

      let sectionIndex = Math.floor(scrollPercentage / 25);
      sectionIndex = Math.min(sectionIndex, scrollSections.length - 1);
      console.log(window.scrollY);
      setSectionIndex(sectionIndex);

      setCurrentSection(scrollSections[sectionIndex]);
    }
  };

  useEffect(() => {
    window.addEventListener("scroll", handleScroll);
    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  }, [scrollPos]);

  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 1000);

    return () => clearTimeout(timer);
  }, []);
  const [api, setAPI] = useState(null);

  const handleMint = async (api, account, type) => {
    mint(api, account, type);
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
      <div className="w-full h-[140vh] absolute font-VT323">
        <div className={styles.pageBackground}></div>

        <div className="w-full  h-[100dvh] sm:h-[100vh] fixed bottom-[0%] z-50">
          <Header />

          <div
            className={` w-full transition-opacity duration-1000 ${
              sectionIndex === 0 ? "opacity-100" : "opacity-0"
            }`}
          >
            <div class="flex items-center justify-start absolute lg:left-1/2 left-[20%] top-40 lg:top-48">
              <div
                className={`lg:text-lg xl:text-3xl max-w-[650px] px-10  py-6 mr-2 lg:mr-0
                text-center rounded-full ${styles.bubble} ${styles["bubble-bottom-left"]}`}
              >
                <div className="flex "> Welcome to the world of Foxie! <img src="/earth.png" className=" hidden lg:block w-8 h-8  ml-2 lg:mt-1"></img> </div>
                
                <br className="hidden lg:block " /> 
                <div className="flex">Chickens peacefully produce $EGGS <img src="/egg.png" className=" hidden lg:block w-8 h-8  ml-2 "></img> </div>
                
                <br className="hidden lg:block " /> 
                
                <div className="flex">Foxes are sneaky, they steal $EGGS <img src="/egg.png" className=" hidden lg:block w-8 h-8  ml-2 "></img> </div>
                
              </div>
            </div>
            <div className="absolute bottom-20 lg:right-1/2">
              <Image
                src="/farmer1.png"
                width={150}
                height={600}
                alt="logo"
                className="mx-4"
              />
            </div>
          </div>
          <div
            className={` w-full h-full flex items-center justify-center flex-col	 transition-opacity duration-1000 ${
              sectionIndex === 1 ? "opacity-100" : "opacity-0"
            }`}
          >
            <div className="flex mx-6">
              <div className="mx-2">
                <Image src="/chicken.png" width={350} height={600} alt="logo" />
              </div>
              <div className="mx-2">
                <Image src="/fox.png" width={350} height={600} alt="logo" />
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

        <ToastContainer />
      </div>
    </>
  );
}
