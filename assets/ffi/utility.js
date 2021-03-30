async function loadHtmlImage(url) {
    return new Promise((resolve, reject) => {
        const image = new Image();
        image.addEventListener('load', () => resolve(image));
        image.addEventListener('error', (event, e) => {
            console.error('Could not load html image from url: ', url, e);
            reject(event);
        });

        image.src = url;
    });
}

export async function load_image(data) {
    const blob = new Blob([data], {"type": "image/png"});
    const objectURL = URL.createObjectURL(blob);

    return await loadHtmlImage(objectURL).catch(() => {
        throw new Error('Could not load image from buffer view');
    });
}
