<template>
  <v-row justify="center" align="stretch">
    <v-col cols="12" :sm="planCardCols" class="text-center py-0"
      v-for="plan in plans" :key="plan.name">
      <v-hover v-slot:default="{ hover }">
        <v-card class="mx-auto blm-pricing-card d-flex flex-column align-self-stretch"
          outlined :elevation="hover ? 4 : 0" :class="{ 'on-hover': hover }">
          <v-card-title class="display-1 justify-center">{{ plan.name }}</v-card-title>

          <div class="text--secondary py-2">
            {{ plan.bestFor }}
          </div>

          <div class="py-4 px02" v-if="plan.price === 0">
            <div class="d-inline-block">
              <span class="display-2 font-weight-bold">Free</span>
            </div>
          </div>
          <div class="py-4 px-0" v-else-if="plan.price > 300">
            <!-- <span class="d-inline-block">Starting at</span>&nbsp;&nbsp; -->
            <div class="d-inline-block">
              <span class="display-2 font-weight-bold">{{ plan.price }}€</span>
            </div>
            <span class="d-inline-block"> /mo </span>
          </div>
          <div class="py-4 px-0" v-else>
            <div class="d-inline-block">
              <span class="display-2 font-weight-bold">{{ plan.price }}€</span>
            </div>
            <span class=""> /mo </span>
          </div>

          <v-card-text class="blm-pricing-card-text text-left py-2" v-html="plan.description">
          </v-card-text>
          <v-spacer />

          <v-card-actions class="justify-center py-4">
            <v-btn color="success" depressed @click="planSelected(plan.value)"
              v-if="selectable && plan.value !== currentPlan">
              Upgrade to {{ plan.name }}
            </v-btn>
            <v-btn color="success" depressed to="register" v-if="!selectable">
              Get started for free
            </v-btn>
            <v-btn color="primary" outlined :ripple="false"
              v-if="selectable && plan.value === currentPlan">
              Current plan
            </v-btn>

          </v-card-actions>
        </v-card>
      </v-hover>
    </v-col>

    <v-col cols="12" xl="8" class="text-left">
      <p>
        Prices are taxes exclusive
      </p>
      <p>
        Soft limits may apply to assure the continuity of the service for everyone.
        Please contact support if you reach a soft limit.
      </p>
    </v-col>
  </v-row>
</template>


<script lang="ts">
import { BillingPlan } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

// https://stackoverflow.com/questions/56703740/how-to-bottom-align-button-in-card-irrespective-of-the-text-in-vuetify

export default VueApp.extend({
  name: 'BPricingTable',
  props: {
    selectable: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    currentPlan: {
      type: String as PropType<string>,
      required: false,
      default: BillingPlan.FREE,
    },
  },
  data() {
    const plans = [];
    const planCardCols = 4;
    if (this.selectable) {
      plans.push({
        name: 'Free',
        value: BillingPlan.FREE,
        bestFor: 'For testing and MVPs',
        price: 0,
        description: `
          <ul>
            <li>2 workspace members</li>
            <li>7 days data retention</li>
            <li>100 MB storage</li>

            <!--
            <li>5,000 contacts</li>
            <li>1 monitor / project</li>
            <li>Community support</li>
            * 100 bots executions
            -->
          </ul>
          `,
      });
      // planCardCols = 3;
    }
    return {
      planCardCols,
      plans: [
        ...plans,
        {
          name: 'Starter',
          value: BillingPlan.STARTER,
          bestFor: 'For individuals and families',
          price: 10,
          description: `
          <ul>
            <li>Remove branding</li>
            <li>5 workspace members</li>
            <li>100 GB storage</li>
            <li>10 000 newsletter contacts</li>
            <li>30 days analytics data retention</li>
            <li><b>Silver support</b></li>
          </ul>
          `,
        },
        {
          name: 'Pro',
          value: BillingPlan.PRO,
          bestFor: 'For companies of all sizes',
          price: 100,
          description: `
          <ul>
            <li><b>All features from Starter +</b></li>

            <li>Unlimited workspace members</li>
            <li>Unlimited newsletter contacts</li>
            <li>Unlimited analytics data retention</li>
            <li>1 TB storage</li>
            <li><b>Gold support</b></li>
            <!-- <li>400GB storage</li>
            <li>2 parallel Bitflow downloads</li> -->
          </ul>
          `,
        },
        // {
        //   name: 'Ultra',
        //   value: BillingPlan.ULTRA,
        //   bestFor: 'For enterprises and unicorns',
        //   price: 1000,
        //   description: `
        //     <ul>
        //       <li><b>All features from Pro +</b></li>

        //       <li>4 TB storage</li>
        //       <li>5 months events data retention</li>
        //       <li><b>Platinium support</b></li>
        //       <!--  <li>1000GB storage</li>
        //       <li>4 parallel Bitflow downloads</li>
        //       <li>Priority Support</li>
        //       -->
        //     </ul>
        //   `,
        // },
      ],
    };
  },
  methods: {
    planSelected(plan: BillingPlan) {
      this.$emit('selected', plan);
    },
  },
});
</script>


<style lang="scss">
.blm-pricing-card-text ul {
  list-style-type: none;
}

.blm-pricing-card-text ul li {
  margin-top: 9px;
}
</style>

<style scoped lang="scss">
.v-card__text {
  font-size: 20px !important;
}

.blm-pricing-card {
  height: 100%;
}

.v-application p {
  margin-bottom: 0;
}
</style>
