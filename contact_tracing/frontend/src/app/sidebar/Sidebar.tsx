import styles from './Sidebar.module.css'

type Props = {
    camera: {x: number, y: number}
    position: {x: number, y: number}
}

export default function Sidebar(props: Props) {
    return (
        <div className={styles.Sidebar}>
            <h1>cam: [{props.camera.x.toFixed(2)}][{props.camera.y.toFixed(2)}]</h1>
            <h1>pos: [{props.position.x.toFixed(0)}][{props.position.y.toFixed(0)}]</h1>
        </div>
    )
}