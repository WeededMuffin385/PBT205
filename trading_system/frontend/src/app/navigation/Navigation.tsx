import styles from './Navigation.module.css'
import {NavLink} from "react-router";

export default function Navigation() {
    return (
        <div className={styles.Navigation}>
            <NavLink className={({isActive}) =>
                isActive ? styles.active : ""
            } to="/dashboard">dashboard</NavLink>
            <NavLink  className={({isActive}) =>
                isActive ? styles.active : ""
            } to="/portfolio">portfolio</NavLink>
            <NavLink  className={({isActive}) =>
                isActive ? styles.active : ""
            } to="/orders">orders</NavLink>
            <NavLink  className={({isActive}) =>
                isActive ? styles.active : ""
            } to="/trade">trade</NavLink>
        </div>
    )
}