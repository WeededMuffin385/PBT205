import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import styles from './ChannelButton.module.css'
import { useNavigate } from "react-router-dom";
import {faTrashCan} from "@fortawesome/free-solid-svg-icons";


type Props = {
    name: string,
    id: string,

    onDelete: () => void,
}

export default function ChannelButton(props: Props) {
    const navigate = useNavigate();

    return (
        <div onClick={() => {
            navigate(`/channels/${props.id}`)
        }} className={styles.ChannelButton}>
            <h3>{props.name}</h3>

            <button onClick={(e) => {
                e.stopPropagation()
                props.onDelete()
            }}><FontAwesomeIcon icon={faTrashCan} className={styles.Icon} /></button>
        </div>
    )
}