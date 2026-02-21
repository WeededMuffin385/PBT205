import styles from './Channel.module.css'
import {useEffect, useRef, useState} from "react";
import {useMutation} from "@tanstack/react-query";
import Message from '../message/Message'
import { useParams } from "react-router-dom";
import {useCookies} from "react-cookie";

export type Message = {
    account_name: string,
    account_id: number,

    content: string,
    created_at: string,
};

export default function Channel() {
    const {id} = useParams();
    const [cookies] = useCookies(['account_id'])

    const account_id = Number(cookies.account_id)

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

    const bottomRef = useRef<HTMLDivElement>(null);
    const [messages, setMessages] = useState<Message[]>([]);

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: "smooth" });
    }, [messages.length]);

    useEffect(() => {
        setMessages([])

        const eventSource = new EventSource(`/api/channels/${id}/callback`);

        eventSource.onmessage = (event) => {
            const message: Message = JSON.parse(event.data);
            console.log(message);
            setMessages((prev) => [...prev, message])
            bottomRef.current?.scrollIntoView({ behavior: "smooth" });
        };

        return () => eventSource.close()
    }, [id]);

    const inputRef = useRef<HTMLTextAreaElement | null>(null);

    const submitMessage = () => {
        if (!inputRef.current) return;
        const message = inputRef.current.value
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
                        <Message key={`${message.account_id}-${message.created_at}`} content={message.content} sender={message.account_name} mine={message.account_id === account_id} created_at={message.created_at} />
                    ))
                }

                <div ref={bottomRef}></div>
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