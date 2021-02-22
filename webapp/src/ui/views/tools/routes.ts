import { RouteConfig } from 'vue-router';

const Base64 = () => import(/* webpackChunkName: "chunk-tools" */ './base64.vue');
const Json = () => import(/* webpackChunkName: "chunk-tools" */ './json.vue');
const Rot13 = () => import(/* webpackChunkName: "chunk-tools" */ './rot13.vue');
const Url = () => import(/* webpackChunkName: "chunk-tools" */ './url.vue');
const Timestamp = () => import(/* webpackChunkName: "chunk-tools" */ './timestamp.vue');
// const Hexdump = () => import(/* webpackChunkName: "chunk-tools" */ './hexdump.vue');
const Hex = () => import(/* webpackChunkName: "chunk-tools" */ './hex.vue');
const QRCode = () => import(/* webpackChunkName: "chunk-tools" */ './qrcode.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/tools/base64',
    component: Base64,
  },
  {
    path: '/tools/qrcode',
    component: QRCode,
  },
  {
    path: '/tools/hex',
    component: Hex,
  },
  {
    path: '/tools/json',
    component: Json,
  },
  {
    path: '/tools/rot13',
    component: Rot13,
  },
  {
    path: '/tools/url',
    component: Url,
  },
  {
    path: '/tools/timestamp',
    component: Timestamp,
  },
  // {
  //   path: '/tools/hexdump',
  //   component: Hexdump,
  // },

  { path: '/tools*', redirect: '/tools/base64' },
];

export default routes;
