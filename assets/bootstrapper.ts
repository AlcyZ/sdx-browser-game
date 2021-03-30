import init, {example} from './wasm/sdx_browser_game';
import glbSample from './glbSample';

export default async function (): Promise<void> {
    await init();
    // await glbSample();

    example();
}
