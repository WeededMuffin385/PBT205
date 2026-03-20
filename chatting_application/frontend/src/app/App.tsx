import styles from './App.module.css'


import Navigation from "./navigation/Navigation.tsx";
import Channel from './channel/Channel.tsx';
import {Navigate, Route, Routes} from "react-router-dom";
import Placeholder from "./placeholder/Placeholder.tsx";
import {useEffect, useState} from "react";
import AuthorisationPopup from "./authorisation_popup/AuthorisationPopup.tsx";



export default function App() {
    const [authorisationInProgress, setAuthorisationInProgress] = useState(false)

    useEffect(() => {
        const load = async () => {
            const response = await fetch('/api/auth/check', {
                method: 'GET'
            });

            if (response.status == 500) {
                throw new Error('failed to access backend server')
            }

            if (response.ok) {
                setAuthorisationInProgress(false)
            } else {
                setAuthorisationInProgress(true)
            }
        }

        load()
    }, [])

    return (
        <div className={styles.App}>
            {authorisationInProgress && <AuthorisationPopup onClose={() => {setAuthorisationInProgress(false)}}/>}

            <Navigation />
            <Routes>
                <Route path="/" element={<Placeholder />} />
                <Route path="/channels/:id" element={<Channel />} />
                <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
        </div>
    )
}