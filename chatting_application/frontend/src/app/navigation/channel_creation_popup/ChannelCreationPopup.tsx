import styles from './ChannelCreationPopup.module.css'
import {useRef} from "react";

type Props = {
    visible: boolean,
    onClose: () => void,
    onCreate: (name: string, id: string) => void,
}

export default function ChannelCreationPopup(props: Props) {
    const nameInputRef = useRef<HTMLInputElement | null>(null)

    if (!props.visible) return null

    return (
        <div className={styles.ChannelCreationPopup}>
            <div className={styles.Inner}>
                <h3>Create a new channel  <button onClick={props.onClose}>❌</button></h3>
                <input ref={nameInputRef} placeholder={"channel name"} />
                <button onClick={async () => {
                    if (!nameInputRef.current) {
                        throw new Error("failed to get name input")
                    }

                    const name = nameInputRef.current.value;

                    const response = await fetch("/api/channels", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json",
                        },
                        body: JSON.stringify({
                            name
                        })
                    });

                    if (!response.ok) {
                        throw new Error("failed to create new channel")
                    }

                    const {id}: {id: string} = await response.json();
                    props.onCreate(name, id)

                    props.onClose()
                }}>create</button>
            </div>
        </div>
    )
}