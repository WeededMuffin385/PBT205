import styles from './Content.module.css'
import {Route, Routes} from "react-router";
import Dashboard from "./dashboard/Dashboard.tsx";
import Trade from "./trade/Trade.tsx";
import List from "./list/List.tsx";

export default function Content() {
    return (
        <div className={styles.Content}>
            <Routes>
                <Route path="/dashboard" element={<Dashboard />} />


                <Route path="/trade/:id" element={<Trade />} />

                <Route path='/trade' element={<List />} />
            </Routes>
        </div>
    )
}