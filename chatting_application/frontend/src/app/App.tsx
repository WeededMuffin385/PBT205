import styles from './App.module.css'


import Navigation from "./navigation/Navigation.tsx";
import Channel from './channel/Channel.tsx';
import {Navigate, Route, Routes} from "react-router-dom";
import Placeholder from "./placeholder/Placeholder.tsx";



export default function App() {
    return (
        <div className={styles.App}>
            <Navigation />

            <Routes>
                <Route path="/" element={<Placeholder />} />
                <Route path="/channels/:id" element={<Channel />} />

                <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
        </div>
    )
}