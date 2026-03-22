import styles from './App.module.css'
import Navigation from "./navigation/Navigation.tsx";
import Content from "./content/Content.tsx";

export default function App() {
    return (
        <div className={styles.App}>
            <Navigation />
            <Content />
        </div>
    )
}
