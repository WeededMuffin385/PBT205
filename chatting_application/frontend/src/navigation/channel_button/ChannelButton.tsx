import styles from './ChannelButton.module.css'
import { useNavigate } from "react-router-dom";


type Props = {
    name: string,
    id: string,
}

export default function ChannelButton(props: Props) {
    const navigate = useNavigate();

    return (
        <div onClick={() => {
            navigate(`/channels/${props.id}`)
        }} className={styles.ChannelButton}>
            <h3>{props.name}</h3>

            <button>⚙️</button>
        </div>
    )
}