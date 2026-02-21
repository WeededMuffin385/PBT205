import pawn from './pawn.svg'
import grass from './grass.svg'

type Message = {
    x: number,
    y: number,
    account_id: number,
    account_name: string,
}

type Account = {
    x: number,
    y: number,
    account_name: string,
}

export default class Engine {
    private canvas: HTMLCanvasElement;
    private context: CanvasRenderingContext2D;
    private run = false;
    private requestAnimationFrame: number | null = null;

    private zoom = 50.0;
    private camera = {x: 0, y: 0};
    private position = {x: 0, y: 0};
    private isDragging = false;
    private lastCursorPos = {x: 0, y: 0};
    private renderDistance = 128;
    private size = 512;

    private pawn: HTMLImageElement = new Image();
    private grass: HTMLImageElement = new Image();

    private accounts: Record<number, Account> = {};
    private positionEventSource: EventSource;

    constructor(canvas: HTMLCanvasElement) {
        const context = canvas.getContext('2d');
        if (!context) throw new Error('failed to create 2d context')
        this.context = context
        this.canvas = canvas

        this.pawn.src = pawn;
        this.grass.src = grass;

        window.addEventListener("resize", this.resize)

        for (let i = 0; i < 32; ++i) {
            const x = Math.floor(Math.random() * (this.size + 1));
            const y = Math.floor(Math.random() * (this.size + 1));

            this.accounts[i] = {
                x,
                y,
                account_name: "noname"
            }
        }

        this.positionEventSource = new EventSource(`/api/position/callback`)
        this.positionEventSource.onmessage = (event) => {
            const message: Message = JSON.parse(event.data);
            this.accounts[message.account_id] = {
                x: message.x,
                y: message.y,
                account_name: message.account_name,
            }
            console.log(message);
        };

        this.canvas.addEventListener("wheel", (e) => {
            if (!e.ctrlKey) return;

            e.preventDefault();

            const rect = this.canvas.getBoundingClientRect();
            const cursor = {x: e.clientX - rect.left, y: e.clientY - rect.top}

            const dpr = window.devicePixelRatio || 1;

            const w = this.canvas.width;
            const h = this.canvas.height;

            const zBefore = this.zoom * dpr;
            const center = {x: w / 2, y: h / 2}

            // world position of cursor
            const world = {x: (cursor.x * dpr - center.x) / zBefore + this.camera.x, y: (cursor.y * dpr - center.y) / zBefore + this.camera.y}

            // change zoom
            const zoomFactor = 1.1;
            if (e.deltaY < 0) {
                this.zoom *= zoomFactor;
            } else {
                this.zoom /= zoomFactor;
            }

            this.zoom = Math.max(1, Math.min(this.zoom, 500));
            const zAfter = this.zoom * dpr;
            this.camera = {x: world.x - (cursor.x * dpr - center.x) / zAfter, y: world.y - (cursor.y * dpr - center.y) / zAfter}
        }, { passive: false });

        this.canvas.addEventListener("pointerdown", (e) => {
            if (e.button !== 0) return;

            this.isDragging = true;
            this.lastCursorPos = {x: e.clientX, y: e.clientY};

            this.canvas.setPointerCapture(e.pointerId)
        })

        this.canvas.addEventListener("pointermove", (e) => {
            if (!this.isDragging) return;

            const offset = {x: e.clientX - this.lastCursorPos.x, y: e.clientY - this.lastCursorPos.y};
            this.lastCursorPos = {x: e.clientX, y: e.clientY};

            this.camera.x -= offset.x / (this.zoom);
            this.camera.y -= offset.y / (this.zoom);
        })

        this.canvas.addEventListener("pointerup", (e) => {
            this.isDragging = false;

            try {
                this.canvas.releasePointerCapture(e.pointerId)
            } catch {}
        })

        this.canvas.addEventListener("pointerleave", (e) => {
            this.isDragging = false;

            try {
                this.canvas.releasePointerCapture(e.pointerId)
            } catch {}
        })

        window.addEventListener("keydown", (e) => {
            const step = 1; // размер шага

            switch (e.code) {
                case "ArrowUp":
                    this.position.y -= step;
                    break;

                case "ArrowDown":
                    this.position.y += step;
                    break;

                case "ArrowLeft":
                    this.position.x -= step;
                    break;

                case "ArrowRight":
                    this.position.x += step;
                    break;
            }

            fetch('/api/position', {
                method: 'POST',
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify({x: this.position.x, y: this.position.y}),
            })
        });
    }

    start() {
        if (this.run) return;
        this.run = true

        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                this.resize()
                this.update()
            })
        })
    }

    stop() {
        this.run = false
        this.positionEventSource.close()
    }

    getCamera() {
        return {...this.camera}
    }

    private update() {
        if (this.requestAnimationFrame !== null) return;
        if (!this.run) return;
        this.draw()
        this.requestAnimationFrame = requestAnimationFrame((/*time*/) => {
            this.requestAnimationFrame = null;
            this.update();
        })
    }

    private draw() {
        const w = this.canvas.width
        const h = this.canvas.height

        this.context.setTransform(1, 0, 0, 1, 0, 0)
        this.context.clearRect(0, 0, w, h)

        this.context.fillStyle = "gray";
        this.context.fillRect(0, 0, w, h)

        const dpr = window.devicePixelRatio || 1;
        const z = this.zoom * dpr;

        const center = {
            x: w / 2,
            y: h / 2,
        };

        this.context.setTransform(
            z,
            0,
            0,
            z,
            center.x - this.camera.x * z,
            center.y - this.camera.y * z,
        )


        this.draw_board();
        this.draw_accounts();

        /*        this.context.fillStyle = "red";
                this.context.fillRect(0, 0, 1, 1);*/
    }

    private draw_board() {
        for (let i = -this.renderDistance; i < this.renderDistance; ++i) {
            for (let j = -this.renderDistance; j < this.renderDistance; ++j) {
                const x = Math.floor(this.camera.x) + i;
                const y = Math.floor(this.camera.y) + j;

                if (x < 0) continue;
                if (y < 0) continue;

                if (x >= this.size) continue;
                if (y >= this.size) continue;

                if ((x + y) % 2 === 0) {
                    this.context.fillStyle = 'green';
                } else {
                    continue
                }

                this.context.fillRect(x, y, 1, 1)
            }
        }
    }

    private draw_accounts() {
        for (const [_, value] of Object.entries(this.accounts)) {
            this.context.drawImage(this.pawn, value.x, value.y, 1, 1);
        }
    }

    private resize = () => {
        console.log("resized")

        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.getBoundingClientRect();

        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;
    }
}