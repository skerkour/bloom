// vue.config.js
module.exports = {
  css: { extract: false },
  configureWebpack: {
    // No need for splitting
    optimization: {
      splitChunks: false,
    },
  },
  productionSourceMap: false,
};
