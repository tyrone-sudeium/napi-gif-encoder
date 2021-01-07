export class GIFEncoder {
    /**
     * Create a new GIF Encoder instance.
     * @param width Image width. Must be < 65535.
     * @param height Image height. Must be < 65535.
     * @param file Absolute path to output file.
     */
    constructor(width: number, height: number, file: string)
    /**
     * Add a frame to the GIF.
     * @param frame Buffer containing RGBA pixel data.
     */
    addFrame(frame: Buffer): void
    /**
     * Sets the framerate for the GIF. Defaults to 25.
     * @param framerate Frame rate in FPS for the target GIF.
     */
    setFrameRate(framerate: number): void
    /**
     * Sets the sampling factor for the Neuquant color quantization algorithm.
     * Generally: 1 is highest quality and slowest, 10 or higher is low
     * quality (lots of color banding) but fast. Must be < 65535.
     * Defaults to 10.
     * @param factor The new sampling factor.
     */
    setSampleFactor(factor: number): void
    /**
     * Render the GIF and write it to disk. As the most time consuming step,
     * it is asynchronous.
     */
    finish(): Promise<void>
}
