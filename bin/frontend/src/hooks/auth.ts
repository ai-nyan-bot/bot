import {useCallback, useContext, useState} from "react";
import {useNavigate} from "react-router-dom";
import {ContextAppDispatch, ContextAppState} from "@app/context";
import {useLocalStorage} from "@states/local.ts";

export const useAuth = () => {
    const [localStorage, setLocalStorage] = useLocalStorage();
    return [localStorage.auth];
}


type LogoutAction = (abortController?: AbortController) => void
export const useLogout = (): [LogoutAction, boolean, Error | null] => {
    const {auth} = useContext(ContextAppState);
    const appDispatch = useContext(ContextAppDispatch);
    const navigate = useNavigate()
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<Error | null>(null);

    const fn = useCallback((abortController?: AbortController) => {

        fetch(`${import.meta.env.VITE_BASE_URL}/v1/logout`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${auth.token}`
            },
            signal: abortController?.signal,
        })
            .then(() => {
                setLoading(false)
                appDispatch({type: 'APP_LOGOUT'})
                navigate("/", {replace: true})
            })
            .catch(error => {
                if (error.name !== 'AbortError') {
                    // FIXME NETWORK ERROR
                    setError(error)
                    setLoading(false)
                }

                if (error.message === 'NetworkError when attempting to fetch resource.') {
                    window.location.href = '/'
                }
            })

    }, [appDispatch, auth.token, navigate])


    return [fn, loading, error]
}

type MetaMaskAction = (address: string, signature: string, redirect: string, abortController?: AbortController) => void
export const useMetaMask = (): [MetaMaskAction, string | null, boolean, Error | null] => {
    const navigate = useNavigate()
    const appDispatch = useContext(ContextAppDispatch);
    const [token, setToken] = useState<string | null>(null)
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<Error | null>(null);

    const fn = useCallback((address: string, signature: string, redirect: string, abortController?: AbortController) => {
        fetch(`${import.meta.env.VITE_BASE_URL}/v1/auth/metamask`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                address,
                signature,
            }),
            signal: abortController?.signal,
        })
            .then(response => {
                if (!response.ok) {
                    setError(Error(`Request submission failed: ${response.status} - ${response.statusText}`))
                    setLoading(false)
                }

                response.json().then(data => {
                    setToken(data.token)

                    appDispatch({
                        type: 'APP_LOGIN_METAMASK',
                        user: data.user,
                        token: data.token,
                        wallet: data.wallet,
                    })

                    navigate(redirect)

                })
                setLoading(false)
            })
            .catch(error => {
                if (error.name !== 'AbortError') {
                    // FIXME NETWORK ERROR
                    setError(error)
                    setLoading(false)
                }

                if (error.message === 'NetworkError when attempting to fetch resource.') {
                    appDispatch({type: 'APP_LOGOUT'})
                    window.location.href = '/'
                }
            })
    }, [appDispatch, navigate])

    return [fn, token, loading, error]
}

type TelegramAction = (query: string, redirect: string, abortController?: AbortController) => void
export const useTelegram = (): [TelegramAction, string | null, boolean, Error | null] => {

    const dispatch = useContext(ContextAppDispatch);
    const navigate = useNavigate();
    const [token, setToken] = useState<string | null>(null)
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<Error | null>(null);

    const fn = useCallback((query: string, redirect: string, abortController?: AbortController) => {
        fetch(`${import.meta.env.VITE_BASE_URL}/v1/auth/telegram`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({query}),
            signal: abortController?.signal,
        })
            .then(response => {
                if (!response.ok) {
                    setError(Error(`Request submission failed: ${response.status} - ${response.statusText}`))
                    setLoading(false)
                }

                response
                    .json()
                    .then(data => {
                        setToken(data.token)

                        dispatch({
                            type: 'APP_LOGIN_TELEGRAM',
                            user: data.user,
                            token: data.token,
                            telegram: data.telegram,
                            wallet: data.wallet,
                        })

                        setLoading(false)

                        navigate(redirect)
                    })

            })
            .catch(error => {
                if (error.name !== 'AbortError') {
                    // FIXME NETWORK ERROR
                    setError(error)
                    setLoading(false)
                }

                if (error.message === 'NetworkError when attempting to fetch resource.') {
                    dispatch({type: 'APP_LOGOUT'})
                    window.location.href = '/'
                }
            })
    }, [dispatch])

    return [fn, token, loading, error]
}