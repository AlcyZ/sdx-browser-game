module.exports = {
    mount: {
        "assets": "/",
        "models": "/models"
    },

    optimize: {
        minify: false,
        bundle: false,
    },

    plugins: [
        '@snowpack/plugin-typescript',
        '@snowpack/plugin-sass',
    ],
};
