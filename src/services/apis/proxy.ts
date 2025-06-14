import { PROXY } from "./config";
import { generateProxyConfig } from "../../utils/proxy";
import { AxiosRequestConfig, AxiosResponse } from "axios";

export type Method =
  | "get"
  | "post"
  | "postForm"
  | "put"
  | "putForm"
  | "delete"
  | "head"
  | "patch"
  | "patchForm"
  | "options";
export const methods: Method[] = [
  "get",
  "post",
  "postForm",
  "put",
  "putForm",
  "delete",
  "head",
  "patch",
  "patchForm",
];
type GeneratorParams = {
  url: string;
  options?: Parameters<typeof generateProxyConfig>[1];
  headers?: Parameters<typeof generateProxyConfig>[2];
};

export const proxyRequest = async <T = any, R = any, D = any>(
  method: Method = "get",
  generatorParams: GeneratorParams,
  axiosConfig?: AxiosRequestConfig,
  data?: any
) => {
  try {
    method = method.trim().toLowerCase() as Method;
    if (!methods.includes(method)) {
      throw new Error(
        `Invalid method ${method}, required one of [${methods.join(", ")}]`
      );
    }
    if (
      typeof generatorParams !== "object" ||
      !typeof generatorParams ||
      Array.isArray(generatorParams) ||
      typeof generatorParams.url !== "string"
    ) {
      throw new Error(`Invalid generator params, required object with [url]`);
    }

    const proxyConfig = generateProxyConfig(
      generatorParams.url,
      generatorParams.options,
      generatorParams.headers,
      true
    ) as NonNullable<ReturnType<typeof generateProxyConfig>>;

    const url = `/${proxyConfig.encodedConfigPath}`.trim().replace(/^\/+$/, "");

    if (!url.trim()) {
      throw new Error("Invalid encoded config path");
    }

    if (method.match(/(put|post|patch)/)) {
      const res = (await PROXY[method]<T, R, D>(
        url,
        data,
        axiosConfig
      )) as AxiosResponse<T, D>;
      return res;
    }
    const res = (await PROXY[method]<T, R, D>(
      url,
      // @ts-ignore
      axiosConfig
    )) as AxiosResponse<T, D>;
    return res;
  } catch (err) {
    throw err;
  }
};
