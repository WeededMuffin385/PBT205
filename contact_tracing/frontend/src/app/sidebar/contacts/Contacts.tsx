import styles from './Contacts.module.css'
import {useRef, useState} from "react";

export default function Contacts() {
    const [contacts, setContacts] = useState<number[]>([])
    const inputRef = useRef<HTMLInputElement | null>(null);
    const [error, setError] = useState<string | null>(null)

    return (
        <div className={styles.Contacts}>
            <div className={styles.Search}>
                <input ref={inputRef} placeholder={"🔎 account id or name"}/>
                <button onClick={async () => {
                    if (inputRef.current === null) return

                    const response = await fetch(`/api/accounts/${inputRef.current.value}/contacts`);
                    if (!response.ok) setError('failed to get contacts of a specific user from the backend')

                    const contacts: number[] = await response.json()

                    if (contacts.length === 0) setError('contact list is empty'); else setError(null);
                    setContacts(contacts)
                }}>show</button>

            </div>
            <div className={styles.Container}>
                {error !== null && <h3 className={styles.Error}>{error}</h3>}

                {contacts.map((id) => <h3>{id}</h3>)}
            </div>
        </div>
    )
}