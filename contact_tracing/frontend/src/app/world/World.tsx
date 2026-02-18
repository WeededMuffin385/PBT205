import styles from './World.module.css'
import {useEffect, useRef} from "react";
import Engine from '../../engine/Engine'

type Props = {
    setCamera: React.Dispatch<React.SetStateAction<{
        x: number
        y: number
    }>>
}

export default function World(props: Props) {
    const canvasRef = useRef<HTMLCanvasElement | null>(null)

    useEffect(() => {
        const engine = new Engine(canvasRef.current!);
        engine.start()

        const interval = setInterval(() => {
            props.setCamera(engine.getCamera())
        }, 100)

        return () => {
            engine.stop()
            clearInterval(interval)
        }
    }, []);

    return (
        <div className={styles.World}>
            <canvas
                ref={canvasRef}
            />
        </div>
    )
}