const path = require('path');
module.exports = function(env) {
    let config = {
        entry: path.join(__dirname, 'analytics.ts'),
        output: {
            path: path.join(__dirname),
            filename: 'analytics.js',
        },
        resolve: {
            extensions: ['.ts', '.tsx', '.js', '.jsx', '.wasm']
        },
        module: {
            rules: [{
                test: /\.tsx?$/,
                use: 'awesome-typescript-loader'
            }]
        },
        devtool: 'source-map',
    };
    if (env != 'prod') {
        config.mode = 'development';
    } else {
        config.mode = 'production';
    }
    return config;
}