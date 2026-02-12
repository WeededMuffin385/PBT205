import styles from './Message.module.css'

export type Props = {
    content: string,
    sender: string,
    mine: boolean,
    time: string,
    date: string,
}

export default function Message(props: Props) {
    return(
        <div className={[styles.Message, props.mine ? styles.Mine : styles.Theirs].join(' ')}>
            <h3>{props.sender}</h3>
            <h3 className={styles.Time}>{props.time}</h3>

            {
                props.content.split('\n').map((line, index) => (
                    <p key={index}>{line}</p>
                ))
            }
        </div>
    )
}