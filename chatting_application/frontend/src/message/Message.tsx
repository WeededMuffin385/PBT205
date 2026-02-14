import styles from './Message.module.css'

export type Props = {
    content: string,
    sender: string,
    mine: boolean,
    created_at: string,
}

export default function Message(props: Props) {
    const time = new Date(props.created_at).toLocaleTimeString(undefined, {
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
    });

    return(
        <div className={[styles.Message, props.mine ? styles.Mine : styles.Theirs].join(' ')}>
            <h3>{props.sender}</h3>
            <h3 className={styles.Time}>{time}</h3>

            {
                props.content.split('\n').map((line, index) => (
                    <p key={index}>{line}</p>
                ))
            }
        </div>
    )
}