import React, {FC} from "react";
import './portfolio.css';
import {useNavigate} from "react-router-dom";

interface Portfolio {
    id: string,
    name: string,
    balance: number,
    changes: {
        "5m": number,
        "1h": number,
        "4h": number,
        "24h": number,
    }
}

interface PortfolioListProps {
    portfolios: Array<Portfolio>
}

export const PortfolioList: FC<PortfolioListProps> = ({portfolios}) => {
    return (
        <div className="list-container">
            {portfolios.map((portfolio, index) => (
                <PortfolioCard
                    key={index}
                    id={portfolio.id}
                    name={portfolio.name}
                    balance={portfolio.balance}
                    changes={portfolio.changes}
                />
            ))}
        </div>
    );
};


interface PortfolioProps {
    id: string,
    name: string,
    balance: number,
    changes: {
        "5m": number,
        "1h": number,
        "4h": number,
        "24h": number,
    }
}

// A single portfolio card component
const PortfolioCard: FC<PortfolioProps> = ({id, name, balance, changes}) => {
    const navigate = useNavigate();
    return (
        <div className="card" onClick={() => {
            navigate(`/portfolio/${id}`)
        }}>
            <h3 className="name">{name}</h3>
            <p className="balance">Balance: ${balance.toFixed(2)}</p>
            <div className="changes-container">
                <div className="change">
                    <span>5m:</span>
                    <span className={getChangeStyle(changes["5m"])}>{changes["5m"].toFixed(2)}%</span>
                </div>
                <div className="change">
                    <span>1h:</span>
                    <span className={getChangeStyle(changes["1h"])}>{changes["1h"].toFixed(2)}%</span>
                </div>
                <div className="change">
                    <span>4h:</span>
                    <span className={getChangeStyle(changes["4h"])}>{changes["4h"].toFixed(2)}%</span>
                </div>
                <div className="change">
                    <span>24h:</span>
                    <span className={getChangeStyle(changes["24h"])}>{changes["24h"].toFixed(2)}%</span>
                </div>
            </div>
        </div>
    );
};

// Helper to style changes based on positive/negative values
const getChangeStyle = (value: number) => {
    return value >= 0 ? "positive" : "negative";
};