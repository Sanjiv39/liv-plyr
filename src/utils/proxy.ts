import { AxiosHeaderValue } from "axios";
import { ContentType } from "../types/content-types";
import { PROXY } from "../services/apis/config";

type ProxyHeaderKey =
  | "Host"
  | "User-Agent"
  | "Accept"
  | "Accept-Language"
  | "Accept-Encoding"
  | "Authorization"
  | "Origin"
  | "Referer"
  | "Connection"
  | "Upgrade-Insecure-Requests"
  | "If-Modified-Since"
  | "If-None-Match"
  | "Cache-Control"
  | "Content-Length"
  | "Content-Type"
  | "Content-Encoding";

export type ProxyHeaders = {
  [k: string]: AxiosHeaderValue;
  "Content-Type": ContentType;
} & Record<ProxyHeaderKey, AxiosHeaderValue>;

export type GenerateProxyOptions = {
  streamResponse: boolean;
};

export const generateProxyConfig = (
  url: string,
  options?: Partial<GenerateProxyOptions>,
  headers?: Partial<ProxyHeaders>,
  throwError?: boolean
) => {
  try {
    if (
      typeof url !== "string" ||
      !url.trim() ||
      !url.trim().match(/^http(s|)\:\/\/[^.]+[.][^.]+/)
    ) {
      throw new Error(
        `Target URL must be a valid string in format [<http|https>://example.com]`
      );
    }
    url = url.trim();
    if (
      typeof options !== "undefined" &&
      (typeof options !== "object" || Array.isArray(options))
    ) {
      throw new Error(`Options must be a valid object in format [key: value]`);
    }
    if (
      typeof headers !== "undefined" &&
      (typeof headers !== "object" || Array.isArray(headers))
    ) {
      throw new Error(`Headers must be a valid object in format [key: value]`);
    }

    const allOptions: Partial<GenerateProxyOptions> & {
      streamResponse: boolean;
    } = {
      streamResponse: false,
      ...options,
    };

    const query = `?url=${encodeURIComponent(
      url
    )}&stream=${!!allOptions.streamResponse}&headers=${encodeURIComponent(
      JSON.stringify(headers || {})
    )}`;

    const data = {
      targetUrl: url,
      headers: headers || null,
      /**
       * @description The Query URL that can be prepended to proxy request
       * @description It contains encoded `target-url` and encoded `headers`
       */
      queryUrl: query,
      /**
       * @description The Complete Proxy URL that will request
       * @description It is in the format `<proxyBaseUrl>?<proxyQuery>`
       */
      fullUrl: `${PROXY.defaults.baseURL}${query}`,
    };
    return data;
  } catch (err) {
    if (throwError) {
      throw err;
    }
    console.error("Error generating proxy config");
    return null;
  }
};
