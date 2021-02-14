<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="text-h3">
          Frequently Asked Questions
        </h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-left">
        <div class="mb-5 mt-5" v-for="(question, index) in questions" :key="index">
          <h2 class="text-h4 font-weight-medium mb-2">{{ question.question }}</h2>
          <p class="text-body-1" v-html="question.answer" />
          <v-divider v-if="index !== questions.length - 1"/>
        </div>
      </v-col>

    </v-row>

    <v-row class="justify-center">
      <v-btn color="success" to="/register" depressed>
        Get started for free
      </v-btn>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BFaqView',
  data() {
    return {
      questions: [
        {
          question: 'I had a very bad experience with $BigCompany customer support, is yours better?',
          answer: `
            We are 100% self-funded, thus your problems are our problems. <br />
            Being an human scale company, we do our best to provide a fast and friendly support. <br />
            The people we value are our customers paying us, not the advertisers or investors.
          `,
        },
        {
          question: 'Why is Bloom more ethical than other service providers?',
          answer: `
          Bloom is <a href="https://github.com/skerkour/bloom" target="_blank" rel="noopener"><b>100% Open Source</b></a>.
          Anyone can inspect how Bloom works and even contribute to it!
          There are no ads or no creepy marketing.
          Just open technology for a fast, simple, and secure experience.`,
        },
        {
          question: 'How secure is my data?',
          answer: `
          As some of your most important data is stored on Bloom, we designed it with care to be as secure as possible using state
          of the art cryptography. <br />
          One-time passwords are hashed using <a href="https://en.wikipedia.org/wiki/Argon2" target="_blank" rel="noopener">Argon2id</a>. <br />
          External services credentials are encrypted at rest using the <a href="https://tools.ietf.org/html/draft-irtf-cfrg-xchacha-03" target="_blank" rel="noopener">XChaCha20-Poly1305 AEAD</a>. <br />
          Files are encrypted at rest using <a href="https://aws.amazon.com/blogs/security/how-to-protect-data-at-rest-with-amazon-ec2-instance-store-encryption/" target="_blank" rel="noopener">AES-256</a>.<br />
          All the disks of our servers and databases are encrypted using <a href="https://aws.amazon.com/blogs/security/how-to-protect-data-at-rest-with-amazon-ec2-instance-store-encryption/" target="_blank" rel="noopener">AES-256</a>. <br />
          Read our <a href="/security">Security</a> and <a href="/privacy">Privacy</a> pages to learn in detail how we secure data.
          `,
        },
        // {
        //   question: 'Where is the data stored?',
        //   answer: `
        //   The data are stored in a <a href="https://aws.amazon.com" target="_blank" rel="noopener">Amazon AWS</a>
        //   datacenter in a privacy friendly country European: Ireland. <br />
        //   Learn more in our <a href="/privacy">privacy policy</a>.
        //   `,
        // },
        {
          question: 'Do you issue refunds?',
          answer: `We do not issue refunds, but we offer a free tier
          to allow you to test our service before subscribing.`,
        },
      ],
    };
  },
});
</script>


<style lang="scss" scoped>
</style>
