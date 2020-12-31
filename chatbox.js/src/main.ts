import Vue from 'vue';
import store from '@/app/store';
import Chatbox from '@/chatbox.vue';
import { ChatboxService, ChatboxServiceProvider } from './domain/chatbox/service';
import APIClient from './api/client';
import { BloomService, BloomServiceProvider } from './domain/bloom';
import { LocalStorageService } from './app/local_storage';

const localStorage = new LocalStorageService();
const bloomService = new BloomService(localStorage);
const apiClient = new APIClient(bloomService);
const chatboxService = new ChatboxService(apiClient, store, bloomService);

if (process.env.NODE_ENV === 'production') {
  Vue.config.productionTip = false;
} else {
  Vue.config.productionTip = true;
}

Vue.use(ChatboxServiceProvider, chatboxService);
Vue.use(BloomServiceProvider, bloomService);

new Vue({
  store,
  render: (h) => h(Chatbox),
}).$mount('#bloom-chatbox');
