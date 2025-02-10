import {ReactNode, useEffect, useState} from 'react'
import {useTheme} from "@hooks/theme.ts";

export const sideBarAnimationDuration = 300

const Layout = ({children}: { children: ReactNode }) => {
    const [mounted, setMounted] = useState(false)
    const theme = useTheme();
    // const isWhitelisted = useWhitelist()
    // const { connecting } = useWallet()
    // const { asPath } = useRouter()
    // const [termsAccepted] = useLocalStorageState(TERMS_OF_USE_KEY, false)
    // const [showIntroModal, setShowIntroModal] = useLocalStorageState('seenIntroKey-0.1', true,)

    useEffect(() => setMounted(true), [])
    if (!mounted) return null

    // const popIntroModal =
    //     isWhitelisted &&
    //     asPath !== '/terms-of-use' &&
    //     asPath !== '/privacy-policy' &&
    //     (showIntroModal || !termsAccepted)

    return (
        <main
            className={`font-sans`}
        >
            <div
                className={`min-h-screen grow ${
                    theme === "DARK"
                        ? 'bg-th-primary'
                        : ''
                } text-th-fgd-2 transition-all`}
            >
                <div className="fixed bottom-0 h-[148px] w-full bg-th-primary"/>
                {/*<div*/}
                {/*    className={`fixed bottom-[136px] h-24 w-full ${*/}
                {/*        theme === "DARK"*/}
                {/*            ? `bg-[url('/images/water.svg')]`*/}
                {/*            : `bg-[url('/images/water-light.svg')]`*/}
                {/*    } bg-repeat-x opacity-50`}*/}
                {/*/>*/}
                {/*<div*/}
                {/*    className={`fixed bottom-[136px] h-48 w-full ${*/}
                {/*        theme === "DARK"*/}
                {/*            ? `bg-[url('/images/palms.svg')]`*/}
                {/*            : `bg-[url('/images/palms-light.svg')]`*/}
                {/*    } bg-contain bg-bottom bg-repeat-x`}*/}
                {/*/>*/}
                {/* <Transition
          show={isAtBottom}
          enter="transition-opacity duration-500"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="transition-opacity duration-500"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div
            className={`fixed ${
              isWhitelisted ? 'bottom-6' : 'bottom-0'
            } w-full`}
          >
            <Footer />
          </div>
        </Transition> */}

                {/*{connecting ? (*/}
                {/*    <div className="flex h-screen items-center justify-center">*/}
                {/*        <Loading/>*/}
                {/*    </div>*/}
                {/*) : isWhitelisted ? (*/}
                {/*<div className="relative">*/}
                {/*    <div className="mb-4">*/}
                {/*        <TopBar/>*/}
                {/*        /!*            <WatchlistBar/>*!/*/}
                {/*    </div>*/}
                {/*    <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">*/}
                {children}
                {/*        /!*            <WarningBanner/>*!/*/}
                {/*    </div>*/}
                {/*</div>*/}
                {/*        <StatusBar/>*/}
                {/*    </div>*/}
                {/*) : asPath === '/terms-of-use' || asPath === '/privacy-policy' ? (*/}
                {/*    <div className="relative mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">*/}
                {/*        {children}*/}
                {/*    </div>*/}
                {/*) : (*/}
                {/*    <WhitelistTeaser/>*/}
                {/*)}*/}
                {/*<div className={`relative ${isWhitelisted ? 'sm:pb-6' : ''}`}>*/}
                {/*    <Footer/>*/}
                {/*</div>*/}
            </div>
            {/*{popIntroModal ? (*/}
            {/*    // <IntroModal*/}
            {/*    //     isOpen={popIntroModal}*/}
            {/*    //     onClose={() => setShowIntroModal(false)}*/}
            {/*    // />*/}
            {/*) : null}*/}
        </main>
    )
}

export default Layout

// const TermsOfUse = () => {
//   const { connected } = useWallet()
//   const [acceptTerms, setAcceptTerms] = useLocalStorageState(
//     ACCEPT_TERMS_KEY,
//     '',
//   )

//   const showTermsOfUse = useMemo(() => {
//     return (!acceptTerms || acceptTerms < termsLastUpdated) && connected
//   }, [acceptTerms, connected])

//   return (
//     <>
//       {showTermsOfUse ? (
//         <TermsOfUseModal
//           isOpen={showTermsOfUse}
//           onClose={() => setAcceptTerms(Date.now())}
//         />
//       ) : null}
//     </>
//   )
// }
