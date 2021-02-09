<template>
  <div class="chatbox-header" :style="style">
    <div class="info">
      <img :src="avatarUrl" alt="avatar" />
      <h2>{{ name }}</h2>
      <a target="_blank" ret="noopener" :href="twitterUrl" class="social-link" v-if="twitterUrl">
        <b-twitter-icon />
      </a>
      <a target="_blank" ret="noopener" :href="instagramUrl" class="social-link"
        v-if="instagramUrl">
        <b-instagram-icon />
      </a>
      <a target="_blank" ret="noopener" :href="whatsappUrl" class="social-link"
        v-if="whatsappUrl">
        <b-whatsapp-icon />
      </a>
    </div>
    <div class="close-button" @click="closeChatbox">
      <!-- <img src="@/assets/close.svg" alt="close icon" /> -->
      <b-close-icon />
    </div>
  </div>
</template>

<script lang="ts">
/*  eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import { Mutation } from '@/app/store';
import BCloseIcon from '@/assets/close.vue';
import BInstagramIcon from '@/assets/instagram.vue';
import BTwitterIcon from '@/assets/twitter.vue';
import BWhatsappIcon from '@/assets/whatsapp.vue';
import BAtIcon from '@/assets/at.vue';

export default VueApp.extend({
  name: 'ChatboxHeader',
  components: {
    BCloseIcon,
    BInstagramIcon,
    BTwitterIcon,
    BWhatsappIcon,
    BAtIcon,
  },
  computed: {
    name(): string {
      return this.$store.state.preferences ? this.$store.state.preferences.name : 'Bloom';
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    style(): any {
      return {
        backgroundColor: this.$store.state.preferences?.color,
      };
    },
    avatarUrl(): string {
      if (this.$store.state.preferences!.avatar_url.startsWith('http')) {
        return this.$store.state.preferences!.avatar_url;
      }
      return `${this.$store.state.preferences!.base_url}${this.$store.state.preferences!.avatar_url}`;
    },
    twitterUrl(): string {
      return this.$store.state.preferences!.twitter_url;
    },
    whatsappUrl(): string {
      return this.$store.state.preferences!.whatsapp_url;
    },
    instagramUrl(): string {
      return this.$store.state.preferences!.instagram_url;
    },
  },
  methods: {
    closeChatbox(): void {
      this.$store.commit(Mutation.CLOSE);
    },
  },
});
</script>

<style lang="scss" scoped>
.chatbox-header {
  display: flex;
  justify-content: space-between;
  width: 100%;
  background: #4e8cff;
  height: 64px;
  border-radius: 10px 10px 0px 0px;
  padding: 12px 12px;
  box-sizing: border-box;

  @media (max-width: 450px) {
    border-radius: 0px;
  }
}

.close-button {
  width: 40px;
  height: 40px;
  cursor: pointer;
  border-radius: 5px;
  position: relative;
}

.close-button:hover {
  box-shadow: inset 0 0 100px 100px rgba(255, 255, 255, 0.2);
}

.close-button svg {
  width: 100%;
  height: 100%;
  padding: 13px;
  box-sizing: border-box;
}

.info {
  position: relative;
  color: #fff;
  text-align: center;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;

  * {
    padding: 0;
    margin: 0;
    height: 100%;
  }

  h2 {
    align-self: center;
    padding-top: 2px;
    color: #ffffff;
    font-size: 26px;
    text-align: center;
  }

  img {
    border-radius: 50%;
    align-self: center;
    padding: 10px;
    height: 40px;
    width: 40px;
    box-sizing: initial;
  }

  .social-link {
    align-self: center;
    margin-left: 8px;
    text-decoration: none;
    color: white;
    width: 24px;
    height: 24px;
  }
}

:focus {outline:none;}
::-moz-focus-inner {border:0;}
</style>
