export default class Engine {
    private canvas: HTMLCanvasElement;
    private context: CanvasRenderingContext2D;
    private run = false;
    private requestAnimationFrame: number | null = null;
    private previousTime = 0;

    private zoom = 1.0;
    private camera = {x: 0, y: 0};

    constructor(canvas: HTMLCanvasElement) {
        const context = canvas.getContext('2d');
        if (!context) throw new Error('failed to create 2d context')
        this.context = context
        this.canvas = canvas
    }

    start() {
        if (this.run) return;
        this.run = true
        this.update()
    }

    stop() {
        this.run = false
    }

    private update() {
        if (this.requestAnimationFrame !== null) return;
        if (!this.run) return;
        this.draw()
        this.requestAnimationFrame = requestAnimationFrame((time) => {
            const elapsed = time - this.previousTime;

            const fps = 1000 / elapsed;

            console.log(`fps: ${fps}`)

            this.requestAnimationFrame = null;
            this.update();

            this.previousTime = time
        })
    }

    private draw() {
        const w = this.canvas.width
        const h = this.canvas.height

        this.context.setTransform(1, 0, 0, 1, 0, 0)
        this.context.clearRect(0, 0, w, h)

        this.context.fillStyle = "#333333";
        this.context.fillRect(0, 0, w, h)

        const dpr = window.devicePixelRatio || 1;
        const z = this.zoom / dpr;

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
    }
}