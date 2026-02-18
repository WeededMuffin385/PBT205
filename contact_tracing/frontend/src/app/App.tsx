import styles from './App.module.css'
import World from "./world/World.tsx";
import Sidebar from "./sidebar/Sidebar.tsx";
import {useState} from "react";
import Authorisation from "./authorisation/Authorisation.tsx";

function App() {
    const [camera, setCamera] = useState({x: 0, y: 0})
    const [authorisation, setAuthorisation] = useState(true)

    return (
        <div className={styles.App}>
            {authorisation && <Authorisation onClose={() => {setAuthorisation(false)}}/>}
            <Sidebar camera={camera}/>
            <World setCamera={setCamera} />
        </div>
    )
}

export default App
