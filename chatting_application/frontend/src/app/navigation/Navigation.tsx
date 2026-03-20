import styles from './Navigation.module.css'
import ChannelButton from "./channel_button/ChannelButton.tsx";
import ChannelCreationPopup from "./channel_creation_popup/ChannelCreationPopup.tsx";
import {useEffect, useRef, useState} from "react";
import ChannelButtonSkeleton from "./channel_button_skeleton/ChannelButtonSkeleton.tsx";

type Channel = {
    name: string,
    channel_id: string,
}

export default function Navigation() {
    const [visibleChannelCreationPopup, setVisibleChannelCreationPopup] = useState(false)
    const {channels, addChannel, removeChannel, loading} = useChannels()

    const containerRef = useRef(null);

    return (
        <div className={styles.Navigation}>
            <button className={styles.CreateChatButton} onClick={() => {
                setVisibleChannelCreationPopup(true)
            }}>create a new channel</button>

            <div className={styles.Channels} ref={containerRef}>
                {
                    !loading && channels.map((channel) => <ChannelButton key={channel.channel_id} name={channel.name} id={channel.channel_id} onDelete={async () => {
                        const response = await fetch(`/api/channels/${channel.channel_id}`, {method: "DELETE"})

                        if (!response.ok) {
                            throw new Error("failed to delete a channel")
                        }

                        removeChannel(channel.channel_id)
                    }} />)
                }

                {
                    loading && [...Array(16).keys()].map((index) => <ChannelButtonSkeleton key={index} />)
                }
            </div>

            <ChannelCreationPopup visible={visibleChannelCreationPopup} onClose={() => {setVisibleChannelCreationPopup(false)}} onCreate={addChannel} />
        </div>
    )
}

function useChannels() {
    const [channels, setChannels] = useState<Channel[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const controller = new AbortController();

        async function load() {
            try {
                const start = performance.now();

                const response = await fetch("/api/channels", {
                    signal: controller.signal
                })

                if (!response.ok) {
                    throw new Error("failed to get channels information")
                }

                const result: Channel[] = await response.json();

                console.log(`channels: ${JSON.stringify(result)}`)

                setChannels(result)

                const end = performance.now();
                const elapsed = end - start;
                const remaining = Math.max(0, 1000 - elapsed);
                setTimeout(() => {
                    setLoading(false)
                }, remaining)
            } catch (error: unknown) {
                if (error instanceof DOMException && error.name === "AbortError") {
                    return;
                }

                console.error(error);
            }
        }

        load();

        return () => controller.abort()
    }, []);


    const addChannel = (name: string, id: string) => {
        setChannels((prev) => [...prev, {name, channel_id: id}])
    }

    const removeChannel = (id: string) => {
        setChannels((prev) => prev.filter(({channel_id: current_id}) => current_id !== id))
    }


    return {channels, addChannel, removeChannel, loading}
}