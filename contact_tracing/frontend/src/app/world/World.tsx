import styles from './World.module.css'
import {useEffect, useRef} from "react";
import Engine from '../../engine/Engine'


export default function World() {
    const canvasRef = useRef<HTMLCanvasElement | null>(null)

    useEffect(() => {
        const engine = new Engine(canvasRef.current!);
        engine.start()

        return () => engine.stop()
    }, []);

    return (
        <div className={styles.World}>
            <canvas
                ref={canvasRef}
            />
        </div>
    )
}