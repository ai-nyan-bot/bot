import React, {useContext} from 'react';
import {ContextAppState} from "@app/context.ts";
import {PortfolioList} from "@pages/home/components/portfolio.tsx";
import {TopBar} from "@pages/home/components/top-bar";

const portfolios = [
    {
        id: '1',
        name: "Crypto Portfolio A",
        balance: 1200.5,
        changes: {"5m": -0.2, "1h": 0.5, "4h": 1.2, "24h": 3.4},
    },
    {
        id: '2',
        name: "Crypto Portfolio B",
        balance: 850.2,
        changes: {"5m": 0.1, "1h": -0.4, "4h": 0.9, "24h": 1.1},
    },
    {
        id: '3',
        name: "Crypto Portfolio C",
        balance: 13400.75,
        changes: {"5m": -1.2, "1h": -0.5, "4h": -0.1, "24h": -2.4},
    },
];


const PortfolioListPage: React.FC = () => {
    let appState = useContext(ContextAppState);
    return (
        <div className="relative">
            <div className="mb-4">
                <TopBar/>
            </div>
            <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">
                <PortfolioList portfolios={portfolios}/>
            </div>
            <div className="mb-4">
                {/*<BottomBar/>*/}
            </div>
        </div>
    )
}
export default PortfolioListPage;


