import GlTf from '../../types/glTf';

const HEADER_MAGIC_ID = 'glTF';
const HEADER_VERSION_SUPPORT = 2;

const CHUNK_TYPE_JSON = 0x4E4F534A;
const CHUNK_TYPE_BIN = 0x004E4942;

const decoder = new TextDecoder();

interface GlbHeader {
    magic: string;
    version: number;
    byteLength: number;
}

interface GlbJsonChunk {
    byteLength: number;
    glTf: GlTf;
}

interface GlbBuffer {
    byteLength: number;
    glb: ArrayBuffer;
    binStart: number;
}

export interface Glb {
    header: GlbHeader;
    json: GlbJsonChunk;
    buffer: GlbBuffer;
}

/**
 * Loads glb file header information.
 * Based on: [Binary glTF Layout - Header](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#header)
 *
 * @param glb
 */
const loadHeader = (glb: ArrayBuffer): GlbHeader => {
    const headerMagic = new Uint32Array(glb, 0, 1);
    const headerVersion = new Uint32Array(glb, 4, 1);
    const headerLength = new Uint32Array(glb, 8, 1);

    const magic = decoder.decode(headerMagic);
    if (magic !== HEADER_MAGIC_ID) {
        throw new Error(`Invalid glb format! Header magic mismatch ${HEADER_MAGIC_ID}`);
    }
    const version = headerVersion[0];
    if (version !== HEADER_VERSION_SUPPORT) {
        throw new Error('Invalid glb version! Version must be 2');
    }
    const byteLength = headerLength[0];
    if (byteLength !== glb.byteLength) {
        throw new Error(`Invalid glb byte length! Byte length (${byteLength}) must match length of array buffer (${glb.byteLength})`);
    }

    return {
        magic,
        version,
        byteLength
    };
}

/**
 * Loads glb structured json content.
 * Based on [Binary glTF Layout - Structured JSON Content](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#structured-json-content)
 *
 * @param glb
 */
const loadGlTfJson = (glb: ArrayBuffer): GlbJsonChunk => {
    const jsonChunkLength = new Uint32Array(glb, 12, 1);
    const byteLength = jsonChunkLength[0];
    const jsonChunkType = new Uint32Array(glb, 16, 1);

    if (jsonChunkType[0] !== CHUNK_TYPE_JSON) {
        throw new Error('Invalid glb json chunk. Expected first glb chunk to be structured json content');
    }

    const jsonChunkContent = new Uint8Array(glb, 20, byteLength);
    return {
        byteLength,
        glTf: JSON.parse(decoder.decode(jsonChunkContent))
    };
}

/**
 * Loads glb binary buffer.
 * Based on [Binary glTF Layout - Binary buffer](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#binary-buffer)
 *
 * @param glb
 * @param json
 */
const loadBuffer = (glb: ArrayBuffer, json: GlbJsonChunk): GlbBuffer => {
    const binOffset = 20 + json.byteLength;

    const binChunkLength = new Uint32Array(glb, binOffset, 1);
    const byteLength = binChunkLength[0];
    const binChunkType = new Uint32Array(glb, binOffset + 4, 1);

    if (binChunkType[0] !== CHUNK_TYPE_BIN) {
        throw new Error('Invalid glb bin chunk. Expected second glb chunk to be binary buffer');
    }

    return {
        byteLength,
        glb,
        binStart: binOffset + 8,
    }
}

/**
 * Loads glb information from an array buffer.
 * The buffer must contain the binary blob of the glb file.
 *
 * @param glb
 */
const loadGlb = (glb: ArrayBuffer): Glb => {
    try {
        const header = loadHeader(glb);
        const json = loadGlTfJson(glb);
        const buffer = loadBuffer(glb, json);

        return {
            header,
            json,
            buffer,
        }
    } catch (e) {
        throw new Error(`Could not load glb file: ${e}`);
    }
};

/**
 * Loads an glb file from an external url.
 *
 * @param url
 */
const loadGlbFromUrl = async (url: string): Promise<Glb> => {
    const response = await fetch(url);
    const glb = await response.arrayBuffer();

    return loadGlb(glb);
}

export {loadGlb, loadGlbFromUrl};
