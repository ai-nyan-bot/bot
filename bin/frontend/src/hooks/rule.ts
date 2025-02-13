import {useAuth} from "@hooks/auth.ts";
import {useGet, usePatch, usePost} from "@hooks/http.ts";
import {useCallback} from "react";
import {Condition} from "@types";


export type RuleCreateRequest = {
    name: string,
    sequence: {
        condition: Condition
    }
};

export type RuleCreateResponse = {};

type RuleCreateAction = (req: RuleCreateRequest, abortController?: AbortController) => void
export const useRuleCreate = (): [RuleCreateAction, RuleCreateResponse | null, boolean, Error | null] => {
    const [auth] = useAuth()
    const [post, response, loading, error] = usePost<RuleCreateResponse>()
    const fn = useCallback(async (req: RuleCreateRequest, abortController?: AbortController) =>
        post(`/v1/rules`, req, abortController), [auth]
    )
    return [fn, response, loading, error]
}


export type RuleUpdateRequest = {
    name: string,
    sequence: {
        condition: Condition
    }
};

export type RuleUpdateResponse = {};

type RuleUpdateAction = (id: string, req: RuleUpdateRequest, abortController?: AbortController) => void
export const useRuleUpdate = (): [RuleUpdateAction, RuleUpdateResponse | null, boolean, Error | null] => {
    const [auth] = useAuth()
    const [patch, response, loading, error] = usePatch<RuleUpdateResponse>()
    const fn = useCallback(async (id: string, req: RuleUpdateRequest, abortController?: AbortController) =>
        patch(`/v1/rules/${id}`, req, abortController), [auth]
    )
    return [fn, response, loading, error]
}

export type RuleListResponse = {
    rules: Array<{
        id: number;
        name: string;
    }>
}

type RuleListAction = (abortController?: AbortController) => void
export const useRuleList = (): [RuleListAction, RuleListResponse | null, boolean, Error | null] => {
    const [auth] = useAuth()
    const [get, response, loading, error] = useGet<RuleListResponse>()
    const fn = useCallback(async (abortController?: AbortController) =>
        get(`/v1/rules`, abortController), [auth]
    )
    return [fn, response, loading, error]
}

export type RuleGetResponse = {
    id: number;
    name: string;
}

type RuleGetAction = (id: string, abortController?: AbortController) => void
export const useRuleGet = (): [RuleGetAction, RuleGetResponse | null, boolean, Error | null] => {
    const [auth] = useAuth()
    const [get, response, loading, error] = useGet<RuleGetResponse>()
    const fn = useCallback(async (id: string, abortController?: AbortController) =>
        get(`/v1/rules/${id}`, abortController), [auth]
    )
    return [fn, response, loading, error]
}
