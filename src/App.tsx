import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Hls from "hls.js";
import reactLogo from "./assets/react.svg";
// import axios from "axios";
import { proxyRequest } from "./services/apis/proxy";
import { generateProxyConfig } from "./utils/proxy";
import "./App.css";

const headers = {};
const url = "https://movie.tg-iptv.site/movies/939243/master.m3u8";
proxyRequest("get", { url: url, headers: headers });

const hls = new Hls();
const proxyData = generateProxyConfig(
  "https://movie.tg-iptv.site/movies/939243/master.m3u8",
  undefined,
  headers
);
// hls.trigger;
hls.loadSource(proxyData?.fullEncodedConfigUrl || "");

const res: { status: number; statusText?: string; data: string } = await invoke(
  "proxy_request",
  {
    req: {
      url: url,
      method: "GET",
      headers: {
        "User-Agent": "Smarters IPTV",
      },
    },
  },
  { headers: headers }
);

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <div>Status : {res?.status}</div>
      <div>Status Text : {res?.statusText}</div>
      <div>RESPONSE : {res?.data?.slice(0, 100)}</div>
    </main>
  );
}

export default App;
