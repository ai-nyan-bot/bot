import {useMemo} from "react"
import {breakpoints} from "@utils"

export const useViewport = () => {
    // const width = store((s) => s.window.width)
    // const height = store((s) => s.window.height)
    const width = window.innerWidth;
    const height = window.innerHeight;

    const [isMobile, isTablet, isDesktop] = useMemo(() => {
        if (!width) return [false, false, false]
        const mobile = width < breakpoints.sm
        const tablet = width >= breakpoints.sm && width < breakpoints.md
        const desktop = width >= breakpoints.md
        return [mobile, tablet, desktop]
    }, [width])

    return {width, height, isMobile, isTablet, isDesktop}
}
