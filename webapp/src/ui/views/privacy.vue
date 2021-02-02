<template>
  <!-- eslint-disable -->
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">

      <v-col cols="12" sm="10" md="8" class="text-center mb-4">
        <h1 class="text-h3 mb-3">
          Privacy Policy
        </h1>
        <h4 class="text-center text--darken-1 mb-3">
          Last updated: Oct 19, 2020
        </h4>
      </v-col>


      <v-col cols="12" sm="10" md="8" class="text-left mb-4 text-body-1">
        <div>
            <p class="mb-3"><i>
              We aim for transparency about how we collect data. Accordingly, we have developed this Policy in order
              for you to understand how we collect and make use of personal
              information. The following outlines our privacy policy.<br />
              <b>We do not, and never will share our user data, whether for advertising, marketing tracking or any other reasons.  <br/>
                Advertising is the disruption of aesthetics, the insults to your intelligence and the interruption of your train of thought. <br/>
              </b>
            </i></p>

            <p>
               Keeping our customers data safe and secure is our top priority. <br />
               You can learn more about our security practices on our dedicated <router-link to="/security">Security page</router-link>.
            </p>

            <h4 class="text-left text--darken-1 mt-3 mb-3 text-h4">
              Owner and Data Controller
            </h4>
            <p>
              PHASER SECURITY SAS <br/>
              107 chemin des vergers, 38190 Bernin, France <br/> <br/>
              Owner contact email: <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>
            </p>

            <h4 class="text-left text--darken-1 mt-3 mb-3 text-h4">
              Providers (Data Processors)
            </h4>


            <p>
              <b><a href="https://aws.amazon.com" target="_blank" rel="noopener">Amazon Web Services (AWS)</a></b>:
              Storage, CDN, Ireland - EU.
            </p>

            <p>
              <b><a href="https://www.heroku.com" target="_blank" rel="noopener">Heroku</a></b>:
                Hosting, Ireland - EU.
            </p>

            <p>
              <b><a href="https://stripe.com" target="_blank" rel="noopener">Stripe</a></b>:
                Payments processing, billing information (invoices, address...), United States.
            </p>

            <p>
              <b><a href="https://sentry.io" target="_blank" rel="noopener">Sentry</a></b>:
                Error reporting, United States.
            </p>



            <h4 class="text-left text--darken-1 mt-3 mb-3 text-h4">
              Respect of Privacy
            </h4>
            <p>
              Here's what we do to protect user privacy:
              <ul>
                <li>Unless required by law, when you delete your account we will permanently delete all the associated data.</li>
                <li>Before or at the time of collecting personal information, we will identify the purposes for which information is being collected.</li>
                <li>We will collect and use of personal information solely with the objective of fulfilling those purposes specified by us and for other compatible purposes, unless we obtain the consent of the individual concerned or as required by law.</li>
                <li>We will only retain personal information as long as necessary for the fulfillment of those purposes.</li>
                <li>We will collect personal information by lawful and fair means and, where appropriate, with the knowledge or consent of the individual concerned.</li>
                <li>Personal data should be relevant to the purposes for which it is to be used, and, to the extent necessary for those purposes, should be accurate, complete, and up-to-date.</li>
                <li>We will protect personal information by reasonable security safeguards against loss or theft, as well as unauthorized access, disclosure, copying, use or modification.</li>
                <li>We will make readily available to customers information about our policies and practices relating to the management of personal information.</li>
              </ul>
            </p>



            <h4 class="text-left text--darken-1 mt-3 mb-3 text-h4">
              Payments processing
            </h4>
            <p>
              We do not process financial data ourselves and rely on an external payment provider instead.<br/>

              Payment processing services enable Bloom to process payments by credit card. To ensure
              greater security, Bloom shares only the information necessary to execute the transaction
              with the financial intermediaries handling the transaction.
            </p>
            <h5 class="text-left text--darken-1 mt-3 mb-3">
              Stripe (Stripe Inc)
            </h5>
            <p>
              Stripe is a payment service provided by Stripe Inc.<br/>
            Personal Data collected: various types of Data as specified in the privacy policy of the service.<br/>
            Place of processing: United States – <a href="https://stripe.com/terms/US" target="_blank" rel="noopener">Privacy Policy</a>.
            Privacy Shield participant. <br/>
            </p>

            <v-divider />

            <p class="mt-5 mb-3"><i>
              We are committed to provide our services in accordance with these principles in
              order to ensure that the confidentiality of personal information is protected and maintained.
            </i></p>

            <p class="mt-5 mb-3">
              <a href="https://github.com/skerkour/bloom" target="_blank" rel="noopener">Bloom is 100% Open Source</a>.
              If this privacy policy does not meet your requirements you can self-host it on your own infrastructure and in your own
              terms.
            </p>



        </div>
      </v-col>

    </v-row>
  </v-container>

</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BPrivacyView',
});
</script>


<style lang="scss" scoped>
</style>
