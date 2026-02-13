import styles from './Channel.module.css'
import {useEffect, useRef, useState} from "react";
import {useMutation} from "@tanstack/react-query";
import Message from '../message/Message'
import { useParams } from "react-router-dom";

export type Message = {
    content: string,
    sender: string,
    time: string,
    date: string,
};

export default function Channel() {
    const { id } = useParams();

    const sendMessageMutation = useMutation({
        mutationFn: async (content: string) => {
            const result = await fetch(`/api/channels/${id}`, {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify({content: content}),
            });

            if (!result.ok) {
                throw new Error("Request Failed");
            }

            return result.json()
        }
    });

    const [messages, setMessages] = useState<Message[]>([]);

    useEffect(() => {
        const eventSource = new EventSource(`/api/channels/${id}/callback`);

        eventSource.onmessage = (event) => {
            const message: Message = JSON.parse(event.data);
            console.log(message);
            setMessages((prev) => [...prev, message])
        };

        return () => eventSource.close()
    }, [])

    const inputRef = useRef<HTMLTextAreaElement | null>(null);

    const submitMessage = () => {
        if (!inputRef.current) return;
        let message = inputRef.current.value
        sendMessageMutation.mutate(message)
        inputRef.current.value = ""

        const el = inputRef.current;
        el.style.height = 'auto';
        el.style.height = `${el.scrollHeight}px`;
    }

    return (
        <div className={styles.Chat}>
            <div className={styles.Messages}>
                {
                    messages.map((message) => (
                        <Message content={message.content} sender={message.sender} mine={true} time={message.time} date={message.date} />
                    ))
                }
            </div>

            <div className={styles.Input}>
                    <textarea ref={inputRef} rows={1} placeholder={"Type a message"} onInput={(event) => {
                        const el = event.currentTarget;
                        el.style.height = 'auto';
                        el.style.height = `${el.scrollHeight}px`;
                    }} onKeyDown={(event) => {
                        if (event.key === "Enter" && !event.shiftKey) {
                            event.preventDefault(); // prevent new line
                            submitMessage();
                        }
                    }}></textarea>

                <div className={styles.ButtonContainer}>
                    <button onClick={submitMessage}>🚀</button>
                </div>
            </div>
        </div>
    )
}