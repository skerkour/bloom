module.exports = {
  assetsDir: 'assets',
  transpileDependencies: [
    'vuetify',
  ],
  pwa: {
    name: 'Bloom',
    appleMobileWebAppCapable: 'yes',
    themeColor: '#24292e',
    msTileColor: '#24292e',
    iconPaths: {
      favicon32: './assets/imgs/icons/favicon.png',
      favicon16: './assets/imgs/icons/favicon.png',
      appleTouchIcon: './assets/imgs/icons/bloom_256.png',
      maskIcon: './assets/imgs/icons/bloom_256.png',
      msTileImage: './assets/imgs/icons/bloom_256.png',
    },
  },
};
