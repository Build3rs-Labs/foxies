import "/styles/globals.css";
import { UseInkProvider } from "useink";
import { AlephTestnet } from "useink/chains";

export default function App({ Component, pageProps }) {
  return (
    <UseInkProvider
      config={{
        dappName: "Foxies",
        chains: [AlephTestnet],
      }}
    >
      <Component {...pageProps} />
    </UseInkProvider>
  );
}
