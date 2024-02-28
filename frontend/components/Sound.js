import Head from "next/head";
import React, { useEffect, useState, useRef } from "react";

export default function Sound() {
    
  const [sound, setSound] = useState(0);

  const audioRef = useRef(null);

  const toggleSound = ()=> {
    if (sound == 1) {
        setSound(0);
        sessionStorage.setItem("sound", "0");
        audioRef.current.pause();
        audioRef.current.currentTime = 0;
    }
    else {
        setSound(1);
        sessionStorage.setItem("sound", "1");
        audioRef.current.loop = true;
        audioRef.current.currentTime = 0;
        audioRef.current.play();
    }
  }

  useEffect(() => {
    let playing = sessionStorage.getItem("sound");
    if (playing == null) {
        playing = 0;
        sessionStorage.setItem("sound", "0");
        setSound(0);
        audioRef.current.pause();
        audioRef.current.currentTime = 0;
    }
    else {
        playing = parseFloat(sessionStorage.getItem("sound"));
        setSound(playing);
        if (playing == 1) {
          audioRef.current.loop = true;
          audioRef.current.currentTime = 0;
          audioRef.current.play();
        }
        else {
          audioRef.current.pause();
          audioRef.current.currentTime = 0;
        }
    }
  }, []);

  return (
    <>
      <audio ref={audioRef} src="./foxies.mp3" loop={true}/>
      <button onClick={toggleSound} className="sound-button" style={{marginLeft:(sound == 0)?33:40}}>
        <img src={(sound == 0)?"./speaker-off.png":"./speaker-on.png"} style={(sound == 0)?{width:50, height:50}:{width:35, height:35}}/>
      </button>
    </>
  );
}
