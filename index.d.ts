export class GIFEncoder {
    constructor(width: number, height: number, file: string)
    addFrame(frame: Buffer): void
    setFrameRate(framerate: number): void
    finish(): Promise<void>
}
