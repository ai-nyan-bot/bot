import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './app.tsx'
import './index.css'
import EnsureVersion from "@app/verison";

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <EnsureVersion>
            <App/>
        </EnsureVersion>
    </React.StrictMode>,
)
