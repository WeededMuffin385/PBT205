import styles from './App.module.css'
import World from "./world/World.tsx";
import Sidebar from "./sidebar/Sidebar.tsx";
import {useEffect, useState} from "react";
import Authorisation from "./authorisation/Authorisation.tsx";

function App() {
    const [camera, setCamera] = useState({x: 0, y: 0})
    const [authorisation, setAuthorisation] = useState(false)

    useEffect(() => {
        const load = async () => {
            const response = await fetch('/api/auth/check', {
                method: 'GET'
            });

            if (response.status == 500) {
                throw new Error('failed to access backend server')
            }

            if (response.ok) {
                setAuthorisation(false)
            } else {
                setAuthorisation(true)
            }
        }

        load()
    }, []);

    return (
        <div className={styles.App}>
            {authorisation && <Authorisation onClose={() => {setAuthorisation(false)}}/>}
            <Sidebar camera={camera}/>
            <World setCamera={setCamera} />
        </div>
    )
}
export default App
