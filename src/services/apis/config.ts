import axios from "axios";

const proxy_base = (import.meta.env.VITE_PROXY_BASE || "") as string;

export const PROXY = axios.create({
  baseURL: proxy_base,
  withCredentials: true,
});
