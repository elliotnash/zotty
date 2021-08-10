/* eslint-disable no-undef */
/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: {url: '/', static: true},
    src: {url: '/dist'},
  },
  plugins: [ 
    '@snowpack/plugin-react-refresh', 
    '@snowpack/plugin-dotenv', 
    '@snowpack/plugin-webpack', 
    '@snowpack/plugin-typescript', // TS support
    'snowpack-plugin-svgr' // import SVG as React component
  ],
  /* for local SPA fallback routing support, more below */
  routes: [
    {"match": "routes", "src": ".*", "dest": "/index.html"},
  ],
  devOptions: {
    port: 3000,
  },
  testOptions: {
    files: ['src/**/*.test.*']
  },
  /* optional, if you want to use alias when importing */
  alias: {
    "@app": "./src/",
  }
};