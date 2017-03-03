module.exports = {
    entry: "./dist/app.js",
    output: {
        library: "lib3rd",
        path: "./dist/assets/",
        publicPath: "assets/",
        filename: "3rd.js"
    },
    module: {
        loaders: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                loader: "babel-loader",
                query: {
                    presets: [
                        "es2015",
                        "stage-0"
                    ]
                }
            }
        ]
    },
    vue: {
        loaders: {
            js: "babel-loader"
        }
    },
    resolve: {
        alias: {
            "vue$": "vue/dist/vue.common.js"
        }
    },
    devServer: {
        contentBase: ".",
        proxy: {
            "/api": {
                target: "http://127.0.0.1:8081/",
                secure: false,
                pathRewrite: {
                    "^/api": ""
                },
                changeOrigin: true
            }
        }
    }
}
