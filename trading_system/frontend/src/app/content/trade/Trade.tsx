import styles from './Trade.module.css'
import {useParams} from "react-router";

export default function Trade() {
    const {id} = useParams();

    return (
        <div className={styles.Trade}>
            <h1>{id}</h1>
        </div>
    )
}