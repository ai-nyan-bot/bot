// import {ExclamationTriangleIcon,} from '@heroicons/react/20/solid'
// import {useWallet} from '@solana/wallet-adapter-react'
// import useOnlineStatus from 'hooks/useOnlineStatus'
// import TopBarStore from 'stores/topBarStore'
// import {IconButton} from './shared/Button'
// import HeroSearch from './HeroSearch'

import {IconButton} from "@components/forms";
import {Bars3Icon, Cog8ToothIcon} from "@heroicons/react/20/solid";
import {useTheme} from "@hooks/theme.ts";
import {LogoDark, LogoLight} from "@components/icons";
import React from "react";
import Button from "@components/forms/button.tsx";

// export const TOPBAR_ICON_BUTTON_CLASSES =
//     'relative flex h-12 w-12 items-center justify-center rounded-full focus-visible:bg-th-bkg-3 md:hover:bg-th-bkg-2'

export const TopBar = () => {
    // const {t} = useTranslation('common')
    // const {connected} = useWallet()
    // const {showSettingsModal, setShowSettingsModal} = TopBarStore()
    // const isOnline = useOnlineStatus()
    const theme = useTheme()
    return (
        <>
            {/*{!isOnline ? (*/}
            {/*    <div*/}
            {/*        className="fixed left-1/2 top-3 z-10 flex h-10 w-max -translate-x-1/2 items-center rounded-full bg-th-down px-4 py-2 md:top-8">*/}
            {/*        <ExclamationTriangleIcon className="h-5 w-5 shrink-0 text-white"/>*/}
            {/*        <p className="ml-2 text-white">*/}
            {/*            Your connection appears to be offline*/}
            {/*        </p>*/}
            {/*    </div>*/}
            {/*) : null}*/}
            <div
                className={`container-shadow container-border flex h-20 items-center !border-x-0 !border-t-0 bg-th-bkg-1 px-4 md:grid md:grid-cols-3 md:px-6`}>
                <div className="w-full">
                    <div className="block h-12 w-max">
                        <Button
                            className={`container-shadow bg-th-primary md:hover:bg-th-primary-dark`}
                            // onClick={() => setShowSettingsModal(true)}
                            size="large"
                        >
                            <Bars3Icon className="h-5 w-5 text-th-fgd-1"/>
                        </Button>
                        {theme === 'DARK' ? (
                            <LogoDark className="h-12 w-auto"/>
                        ) : (
                            <LogoLight className="h-12 w-auto"/>
                        )}
                    </div>
                </div>

                <div className="col-span-1">
                    {/*<HeroSearch/>*/}
                    <h1>Portfolio Name</h1>
                </div>
                <div className="col-span-1 flex items-center justify-end md:space-x-3">
                    <IconButton
                        className={`container-shadow bg-th-primary md:hover:bg-th-primary-dark`}
                        // onClick={() => setShowSettingsModal(true)}
                        size="large"
                    >
                        <Cog8ToothIcon className="h-5 w-5 text-th-fgd-1"/>
                    </IconButton>
                    {/*{connected ? (*/}
                    {/*    <div className="pl-2 md:pl-0">*/}
                    {/*        <ConnectedMenu/>*/}
                    {/*    </div>*/}
                    {/*) : (*/}
                    {/*    <div className="pl-2 md:pl-0">*/}
                    {/*        <ConnectWalletButton/>*/}
                    {/*    </div>*/}
                    {/*)}*/}
                </div>
            </div>
            {/*{showSettingsModal ? (*/}
            {/*    <SettingsModal*/}
            {/*        isOpen={showSettingsModal}*/}
            {/*        onClose={() => setShowSettingsModal(false)}*/}
            {/*    />*/}
            {/*) : null}*/}
        </>
    )
}