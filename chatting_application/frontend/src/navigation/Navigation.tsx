import styles from './Navigation.module.css'
import logo from './google_logo.svg'
import ChannelButton from "./channel_button/ChannelButton.tsx";


export default function Navigation() {
    return (
        <div className={styles.Navigation}>
            <button className={styles.GoogleAuthButton} onClick={() => {
                window.location.href = "/api/auth/google"
            }}><img src={logo} /></button>

            <button className={styles.CreateChatButton}>create new chat</button>

            <div className={styles.Chats}>
                <ChannelButton name={"first chat"} id={"1"} />
                <ChannelButton name={"second chat"} id={"2"} />
                <ChannelButton name={"third chat"} id={"3"} />
            </div>
        </div>
    )
}