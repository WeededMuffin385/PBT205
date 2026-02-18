import styles from './App.module.css'
import World from "./world/World.tsx";
import Sidebar from "./sidebar/Sidebar.tsx";

function App() {
    return (
        <div className={styles.App}>
            <Sidebar />
            <World />
        </div>
    )
}

export default App
