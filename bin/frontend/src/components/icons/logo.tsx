import {useViewport} from '@hooks';

export const LogoDark = ({className}: { className: string }) => {
    const {isMobile} = useViewport()
    return (
        <svg
            className={`${className}`}
            xmlns="http://www.w3.org/2000/svg"
            viewBox={isMobile ? '0 0 48 48' : '0 0 146 48'}
            fill="currentColor"
        >

        </svg>
    )
}

export const LogoLight = ({className}: { className: string }) => {
    const {isMobile} = useViewport()
    return (
        <svg
            className={`${className}`}
            xmlns="http://www.w3.org/2000/svg"
            viewBox={isMobile ? '0 0 48 48' : '0 0 146 48'}
            fill="currentColor"
        >

        </svg>
    )
}
