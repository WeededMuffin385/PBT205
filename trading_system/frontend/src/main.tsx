import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import './index.css'
import App from './app/App.tsx'
import {CookiesProvider} from "react-cookie";
import {BrowserRouter} from "react-router";

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <CookiesProvider>
            <BrowserRouter>
                <App/>
            </BrowserRouter>
        </CookiesProvider>
    </StrictMode>,
)
