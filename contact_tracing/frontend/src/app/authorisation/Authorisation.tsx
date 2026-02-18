import styles from './Authorisation.module.css'

type Props = {
    onClose: () => void,
}

export default function Authorisation(props: Props) {
    return (
        <div className={styles.Authorisation}>
            <div className={styles.Inner}>
                <h3>Authorise in the system</h3>

                <input placeholder={'username'} />

                <button onClick={() => {
                    props.onClose()
                }}>authorise</button>
            </div>
        </div>
    )
}