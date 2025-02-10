import React, {FC} from "react";
import "./index.css";
import {TopBar} from "@pages/portfolio/components/top-bar";

// Manual Ordering Portfolio

interface amount {
    quote: number;
    usd: number;
}

interface Changes {
    "5m": number;
    "1h": number;
    "4h": number;
    "24h": number;
}

interface Token {
    name: string;
    image: string;
    amount: amount;
    changes: Changes;
}

interface PortfolioProps {
}

// Helper function to determine change class
const getChangeClass = (value: number): string => (value >= 0 ? "positive" : "negative");

// TokenCard component
const TokenCard: FC<Token> = ({name, image, amount, changes}) => {
    return (
        <div className="token-card">
            <div className="token-header">
                <img src={image} alt={`${name} logo`} className="token-image"/>
                <h3 className="token-name">{name}</h3>
            </div>
            <div className="token-info">
                <p className="token-amount">amount: {amount.quote}</p>
                <p className="token-amount-usd">USD Value: ${amount.usd.toFixed(2)}</p>
                <div className="token-changes">
                    <div className="change">
                        <span>5m:</span>
                        <span className={getChangeClass(changes["5m"])}>{changes["5m"].toFixed(2)}%</span>
                    </div>
                    <div className="change">
                        <span>1h:</span>
                        <span className={getChangeClass(changes["1h"])}>{changes["1h"].toFixed(2)}%</span>
                    </div>
                    <div className="change">
                        <span>4h:</span>
                        <span className={getChangeClass(changes["4h"])}>{changes["4h"].toFixed(2)}%</span>
                    </div>
                    <div className="change">
                        <span>24h:</span>
                        <span className={getChangeClass(changes["24h"])}>{changes["24h"].toFixed(2)}%</span>
                    </div>
                </div>
            </div>
        </div>
    );
};

interface Price {
    quote: number;
    usd: number;
}

interface Order {
    id: string;
    token: string;
    amount: number;
    price: Price;
    type: "Buy" | "Sell";
}

interface OrdersProps {
    orders: Array<Order>;
    onCancelOrder: (id: string) => void;
}

const Orders: FC<OrdersProps> = ({orders, onCancelOrder}) => {
    return (
        <div className="pending-orders">
            <h2 className="section-title">Orders</h2>
            <div className="orders-list">
                {orders.map((order) => (
                    <div className="order-card" key={order.id}>
                        <div className="order-info">
                            <p className="order-type">{order.type}</p>
                            <p className="order-token">Token: {order.token}</p>
                            <p className="order-amount">Amount: {order.amount}</p>
                            <p className="order-price">
                                Price: {order.price.quote}
                            </p>
                        </div>
                        <button
                            className="cancel-button"
                            onClick={() => onCancelOrder(order.id)}
                        >
                            Cancel
                        </button>
                    </div>
                ))}
            </div>
        </div>
    );
};

const handleCancelOrder = (id: string) => {
    console.log(`Order ${id} canceled.`);
};


const PortfolioPage: FC<PortfolioProps> = ({}) => {
    const tokens = [
        {
            name: "Bitcoin",
            image: "https://cryptologos.cc/logos/bitcoin-btc-logo.png",
            amount: {quote: 28300.5, usd: 28003},
            changes: {"5m": 0.2, "1h": -0.5, "4h": 1.3, "24h": 3.1},
        },
        {
            name: "Ethereum",
            image: "https://cryptologos.cc/logos/ethereum-eth-logo.png",
            amount: {quote: 1900.75, usd: 1900.75},
            changes: {"5m": -0.1, "1h": 0.3, "4h": -1.2, "24h": 2.8},
        },
        {
            name: "Cardano",
            image: "https://cryptologos.cc/logos/cardano-ada-logo.png",
            amount: {quote: 0.35, usd: 0.35},
            changes: {"5m": 0.5, "1h": 0.9, "4h": 1.2, "24h": 4.3},
        },
    ];

    const orders: Array<Order> = [
        {
            id: "1",
            token: "BTC",
            amount: 0.01,
            price: {quote: 28300, usd: 28300},
            type: "Buy",
        },
        {
            id: "2",
            token: "ETH",
            amount: 1.5,
            price: {quote: 1900, usd: 1900},
            type: "Sell",
        },
    ];

    return (
        <div className="relative">
            <div className="mb-4">
                <TopBar/>
            </div>
            <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">
                <div className="portfolio-details">
                    <h1 className="page-title">Portfolio A</h1>
                    <h2 className="section-title">Tokens</h2>
                    <div className="tokens-list">
                        {tokens.map((token, index) => (
                            <TokenCard
                                key={index}
                                name={token.name}
                                image={token.image}
                                amount={token.amount}
                                changes={token.changes}
                            />
                        ))}
                    </div>
                    <Orders orders={orders} onCancelOrder={handleCancelOrder}/>
                </div>
            </div>
        </div>

    );
};

export default PortfolioPage;