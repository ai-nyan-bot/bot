import {FC, ReactNode, useEffect} from "react";
import useLocalStorageState from "use-local-storage-state";
import {LOCAL_STORAGE_KEY} from "@states/local";

export const CURRENT_VERSION = "0";
const VERSION_KEY = 'version'

type VersionProps = {
    children?: ReactNode;
}

const EnsureVersion: FC<VersionProps> = ({children}) => {
    const [version, setVersion] = useVersion();

    useEffect(() => {
        if (version !== CURRENT_VERSION) {
            localStorage.removeItem(VERSION_KEY);
            localStorage.removeItem(LOCAL_STORAGE_KEY);
            setVersion(CURRENT_VERSION);
        }
    }, [version, setVersion]);

    if (version !== CURRENT_VERSION) {
        return (
            <p>
                The bot was upgraded for you
            </p>
        );
    }

    return (children)
}

export default EnsureVersion;


export const useVersion = () => {
    return useLocalStorageState<string>(VERSION_KEY, {
        defaultValue: CURRENT_VERSION
    })
}