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
  appendPathIfStream: boolean;
};

const isValidUrl = (url: string) => {
  try {
    new URL(url);
    return true;
  } catch (err) {
    return false;
  }
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
      !url.trim().match(/^http(s|)\:\/\/[^.]+[.][^.]+/) ||
      !isValidUrl(url.trim())
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
      appender: string;
    } = {
      appender: "",
      streamResponse: false,
      appendPathIfStream: true,
      ...options,
    };

    if (allOptions.appendPathIfStream) {
      const match = url.match(/\/[^\/]+[.](m3u|m3u8|ts|m4s|mpd|xml|dash)$/);
      url = match ? url.replace(match[0], "") : url;
      allOptions.appender = `/${match?.[0] || ""}`.replace(/\$/, "");
    }

    const query = `?url=${encodeURIComponent(
      url
    )}&stream=${!!allOptions.streamResponse}&headers=${encodeURIComponent(
      JSON.stringify(headers || {})
    )}`;

    const params = {
      url: url,
      headers: headers || {},
      stream: !!allOptions.streamResponse,
    };
    const encodedParams = encodeURIComponent(JSON.stringify(params));

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
       * @description It is in the format `<proxy-base-url>?<proxyQuery>`
       */
      fullUrl: `${PROXY.defaults.baseURL}${query}`,
      /**
       * @description The encoded path parameter, to support relative-URI too from `XHR`
       * @description It can be used in the proxy as `<proxy-base-url>/<encodedConfigParams>...more?`
       */
      encodedConfigPath: encodedParams + allOptions.appender,
      /**
       * @description The complete url with encoded path parameter.
       * @description Has the `proxy-base-url` and `encoded-params` joined as paths
       */
      fullEncodedConfigUrl: `${PROXY.defaults.baseURL}/${encodedParams}${allOptions.appender}`,
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
