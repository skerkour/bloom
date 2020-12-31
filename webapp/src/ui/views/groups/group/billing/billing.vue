<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row justify="center" v-if="group">
      <v-col cols="12" class="pa-5">
        <h3 class="text-h4">Storage</h3>
      </v-col>

      <v-col cols="12" class="text-center">
        <v-progress-linear :value="storage"></v-progress-linear>
        {{  filesize(usedStorage) }}
            used of {{ filesize(totalStorage) }}
      </v-col>
    </v-row>

    <v-row v-if="showPlans">
      <v-col cols="12" class="pa-5">
        <h3 class="text-h4">Start automating your projects today</h3>
      </v-col>
      <v-col cols="12" xl="10" class="px-4 py-0">
        <b-pricing-table selectable :currentPlan="currentPlan" @selected="planSelected" />
      </v-col>
    </v-row>

    <v-row justify="center" v-else>
      <v-col cols="12" class="pa-5">
        <h3 class="text-h4">Billing Portal</h3>
        <p class="text-subtitle-1">
          Go to the Billing Portal to change or cancel your plan, update your payment methods
          and access your invoices.
        </p>
      </v-col>

      <v-col cols="12" class="text-center">
        <v-btn color="success" @click="gotoBillingPortal" :loading="loading" depressed>
          Go to Billing Portal
        </v-btn>
      </v-col>
      <v-col cols="12" class="text-center">
        <p class="text-body-1">
          Your plan <b>{{ billingInformation.plan }}</b>
        </p>
      </v-col>
    </v-row>

    <v-row justify="center" v-if="showBillingInformation">
      <v-col cols="12">
        <h3 class="text-h4">Billing Information</h3>
      </v-col>

      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>

      <v-col cols="10" xl="8">
        <v-btn text @click="resetBillingInformation(group.billing)" :loading="loading">
          Cancel
        </v-btn>

        <v-btn depressed color="primary"
          @click="updateBillingInformation" :loading="loading">
          Save
        </v-btn>
      </v-col>

      <v-col cols="10" xl="8" class="px-5">
        <b-billing-information v-model="billingInformation" :loading="loading" />
      </v-col>
    </v-row>

    <v-dialog v-model="showBillingInformationDialog" persistent>
      <v-card>
        <v-card-title>
          <h3 class="text-h4">Billing Information</h3>
        </v-card-title>

        <v-card-text>
          <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
            {{ error }}
          </v-alert>
          <b-billing-information v-model="billingInformation" :loading="loading" />
        </v-card-text>

        <v-card-actions>
          <v-btn text @click="cancelBillingInformationDialog" :loading="loading">
            Cancel
          </v-btn>

          <v-spacer />

          <v-btn depressed color="success"
            @click="updateBillingInformation" :loading="loading">
            Save and proceed to payment
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-container>
</template>


<script lang="ts">
import {
  BillingPlan, Group, UpdateBillingInformationInput,
  BillingInformation as ApiBillingInformation, CheckoutSessionInput,
} from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BPricingTable from '@/ui/components/kernel/pricing_table.vue';
import BBillingInformation from '@/ui/components/kernel/billing_information.vue';
import { BillingInformation } from '@/domain/groups/service';
import { isEu } from '@/app/utils/eu';
import filesize from '@/app/filters/filesize';

export default VueApp.extend({
  name: 'BGroupBilling',
  components: {
    BPricingTable,
    BBillingInformation,
  },
  data() {
    return {
      loading: false,
      error: '',
      showBillingInformationDialog: false,
      selectedPlan: BillingPlan.Free,
      group: null as Group | null,
      actorEmail: '',
      billingInformation: {
        plan: '',
        name: '',
        email: '',
        country: '',
        countryCode: '',
        city: '',
        postalCode: '',
        addressLine1: '',
        addressLine2: '',
        state: '',
        taxId: '',
        usedStorage: 0,
        totalStorage: 0,
      } as BillingInformation,
    };
  },
  computed: {
    groupPath(): string {
      return this.$route.params.groupPath;
    },
    currentPlan(): string {
      return this.group?.billing ? this.group?.billing.plan : BillingPlan.Free;
    },
    showBillingInformation(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      return this.group?.billing ? true : false;
    },
    showPlans(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      if (!this.group) {
        return false;
      } if (this.group.billing) {
        return this.group.billing.plan === BillingPlan.Free;
      }

      // if group has not customer attached
      return true;
    },
    storage(): number {
      if (!this.group) {
        return 0;
      }
      return Math.round((this.usedStorage / this.totalStorage) * 100);
    },
    usedStorage(): number {
      return this.group?.billing?.usedStorage ?? 0;
    },
    totalStorage(): number {
      return this.group?.billing?.totalStorage ?? 100000000;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    filesize,
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        const res = await this.$groupsService.fetchGroupBilling(this.groupPath);
        this.group = res.group;
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.actorEmail = res.me.email!;
        this.resetBillingInformation(this.group.billing);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateBillingInformation(): Promise<void> {
      this.loading = true;
      this.error = '';
      // eslint-disable-next-line prefer-destructuring
      let taxId: string | null = this.billingInformation.taxId;
      if (taxId === '' || !isEu(this.billingInformation.countryCode)) {
        taxId = null;
      }
      const input: UpdateBillingInformationInput = {
        namespace: this.groupPath,
        name: this.billingInformation.name,
        email: this.billingInformation.email,
        countryCode: this.billingInformation.countryCode,
        city: this.billingInformation.city,
        postalCode: this.billingInformation.postalCode,
        addressLine1: this.billingInformation.addressLine1,
        addressLine2: this.billingInformation.addressLine2,
        state: this.billingInformation.state,
        taxId,
      };

      try {
        const billingInformation = await this.$groupsService.updateBillingInformation(input);
        this.resetBillingInformation(billingInformation);
        if (this.selectedPlan !== BillingPlan.Free) {
          await this.gotoCheckoutSession(this.selectedPlan);
        }
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async gotoCheckoutSession(plan: BillingPlan) {
      this.loading = true;
      this.error = '';

      const input: CheckoutSessionInput = {
        namespace: this.groupPath,
        plan,
      };

      try {
        await this.$groupsService.gotoCheckoutSession(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    resetBillingInformation(billingInformation: ApiBillingInformation | null | undefined) {
      if (billingInformation) {
        this.billingInformation = {
          plan: billingInformation.plan,
          name: billingInformation.name,
          email: billingInformation.email,
          country: billingInformation.country,
          countryCode: billingInformation.countryCode,
          city: billingInformation.city,
          postalCode: billingInformation.postalCode,
          addressLine1: billingInformation.addressLine1,
          addressLine2: billingInformation.addressLine2,
          state: billingInformation.state,
          taxId: billingInformation.taxId ?? '',
          usedStorage: billingInformation.usedStorage ?? 0,
          totalStorage: billingInformation.totalStorage ?? 0,
        };
      } else {
        this.billingInformation = {
          plan: '',
          name: '',
          email: this.actorEmail ?? '',
          country: '',
          countryCode: '',
          city: '',
          postalCode: '',
          addressLine1: '',
          addressLine2: '',
          state: '',
          taxId: '',
          usedStorage: 0,
          totalStorage: 0,
        };
      }
    },
    planSelected(plan: BillingPlan) {
      // get billing information
      // then, get checkoutSession
      this.selectedPlan = plan;
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      if (!this.group!.billing) {
        this.showBillingInformationDialog = true;
      } else {
        this.gotoCheckoutSession(this.selectedPlan);
      }
    },
    async gotoBillingPortal() {
      this.loading = true;
      this.error = '';

      try {
        await this.$groupsService.gotoBillingPortal(this.groupPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    cancelBillingInformationDialog() {
      this.error = '';
      this.loading = false;
      this.showBillingInformationDialog = false;
    },
  },
});
</script>


<style lang="scss" scoped>
.v-application p {
  margin-bottom: 0;
}
</style>
