export class GIFEncoder {
    constructor(width: number, height: number, file: string)
    addFrame(frame: Buffer): void
    setDelay(delay: number): void
}
