import styles from './World.module.css'
import {useEffect, useRef} from "react";
import Engine from '../../engine/Engine'
import {useCookies} from "react-cookie";
import * as React from "react";

type Props = {
    setCamera: React.Dispatch<React.SetStateAction<{
        x: number,
        y: number,
    }>>

    setPosition: React.Dispatch<React.SetStateAction<{
        x: number,
        y: number,
    }>>
}

export type Account = {
    account_name: string,
    account_id: number,
    x: number,
    y: number,
}

export type Dimensions = {
    w: number,
    h: number,
}

export default function World(props: Props) {
    const [cookies] = useCookies(['account_id'])

    const canvasRef = useRef<HTMLCanvasElement | null>(null)
    const engineRef = useRef<Engine | null>(null)
    const intervalRef = useRef<number | null>(null)

    useEffect(() => {
        let cancelled = false;

        const init = async () => {
            const response = await fetch('/api/world/dimensions');
            if (!response.ok) throw new Error('failed to acquire world dimensions from the backend server');

            const response_auth = await fetch('/api/auth/check');
            if (!response_auth.ok) throw new Error('failed to authenticate');

            const response_accounts = await fetch('/api/accounts');
            if (!response_accounts.ok) throw new Error('failed to get accounts');

            const dimensions: Dimensions = await response.json();
            const accounts: Account[] = await response_accounts.json();
            const account: Account = await response_auth.json();

            if (cancelled) return;

            if (engineRef.current) {
                engineRef.current.stop();
            }

            const engine = new Engine(canvasRef.current!, dimensions, account, accounts)
            engine.start()

            engineRef.current = engine;

            intervalRef.current = window.setInterval(() => {
                props.setCamera(engine.getCamera())
                props.setPosition(engine.getPosition())
            }, 100)
        }

        init()

        return () => {
            cancelled = true

            if (intervalRef.current !== null) {
                clearInterval(intervalRef.current);
                intervalRef.current = null;
            }

            if (engineRef.current !== null) {
                engineRef.current.stop()
                engineRef.current = null
            }
        }
    }, [cookies.account_id]);

    return (
        <div className={styles.World}>
            <canvas
                ref={canvasRef}
            />
        </div>
    )
}