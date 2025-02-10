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
                navigate("/", {replace: true})
                appDispatch({type: 'APP_LOGOUT'})
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

type MetaMaskAction = (address: string, signature: string, code: string | null, abortController?: AbortController) => void
export const useMetaMask = (): [MetaMaskAction, string | null, boolean, Error | null] => {
    const navigate = useNavigate()
    const appDispatch = useContext(ContextAppDispatch);
    const [token, setToken] = useState<string | null>(null)
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<Error | null>(null);

    const fn = useCallback((address: string, signature: string, code: string | null, abortController?: AbortController) => {
        fetch(`${import.meta.env.VITE_BASE_URL}/v1/auth/metamask`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                address,
                signature,
                code
            }),
            signal: abortController?.signal,
        })
            .then(response => {
                if (response.status > 201) {
                    // FIXME
                    setError(Error("Login failed"));
                    setLoading(false)
                } else {
                    response.json().then(data => {
                        setToken(data.token)

                        appDispatch({
                            type: 'APP_LOGIN_METAMASK',
                            userId: data.userId,
                            token: data.token,
                            wallet: data.wallet,
                        })

                        navigate('/wallet')

                    })
                    setLoading(false)
                }
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

type TelegramAction = (query: string, code: string | null, abortController?: AbortController) => void
export const useTelegram = (): [TelegramAction, string | null, boolean, Error | null] => {
    const navigate = useNavigate()
    const appDispatch = useContext(ContextAppDispatch);
    const [token, setToken] = useState<string | null>(null)
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<Error | null>(null);

    const fn = useCallback((query: string, code: string | null, abortController?: AbortController) => {
        fetch(`${import.meta.env.VITE_BASE_URL}/v1/auth/telegram`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({query, friendCode: code}),
            signal: abortController?.signal,
        })
            .then(response => {
                if (response.status > 201) {
                    // FIXME
                    setError(Error("Login failed"));
                    setLoading(false)
                } else {
                    response.json().then(data => {
                        setToken(data.token)

                        appDispatch({
                            type: 'APP_LOGIN_TELEGRAM',
                            userId: data.userId,
                            token: data.token,
                            telegram: data.telegram,
                            wallet: data.wallet,
                        })

                        navigate('/wallet')

                    })
                    setLoading(false)
                }
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