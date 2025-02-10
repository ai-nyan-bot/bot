import {useCallback} from "react";
import {usePost} from "@hooks/http.ts";
import {useAuth} from "@hooks/auth.ts";

export type SwapRequest = {
    from: string;
    to: string;
    amount: number;
}

export type SwapResponse = {
    signature: string;
}

type SwapAction = (walletId: string, req: SwapRequest, abortController?: AbortController) => void
export const useSwap = (): [SwapAction, SwapResponse | null, boolean, Error | null] => {
    const [auth] = useAuth()
    const [post, response, loading, error] = usePost<SwapResponse>()
    const fn = useCallback(async (walletId: string, req: SwapRequest, abortController?: AbortController) =>
        post(`/v1/wallets/${walletId}/swap`, req, abortController), [auth]
    )
    return [fn, response, loading, error]
}
