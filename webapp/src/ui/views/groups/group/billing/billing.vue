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
          Your plan <b>{{ customer.plan }}</b>
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
        <v-btn text @click="resetCustomer(customer)" :loading="loading">
          Cancel
        </v-btn>

        <v-btn depressed color="primary"
          @click="updateBillingInformation" :loading="loading">
          Save
        </v-btn>
      </v-col>

      <v-col cols="10" xl="8" class="px-5">
        <b-customer v-model="customer" :loading="loading" />
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
          <b-customer v-model="customer" :loading="loading" />
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
/* eslint-disable max-len, @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import BPricingTable from '@/ui/components/kernel/pricing_table.vue';
import BCustomer from '@/ui/components/kernel/customer.vue';
import {
  Customer, BillingPlan, Group,
  UpdateBillingInformation, GetCheckoutSession, BillingInformation,
} from '@/domain/kernel/model';
import { isEu } from '@/app/utils/eu';
import filesize from '@/app/filters/filesize';

export default VueApp.extend({
  name: 'BGroupBilling',
  components: {
    BPricingTable,
    BCustomer,
  },
  data() {
    return {
      loading: false,
      error: '',
      showBillingInformationDialog: false,
      selectedPlan: BillingPlan.FREE,
      group: null as Group | null,
      customer: {
        plan: BillingPlan.FREE,
        name: '',
        email: '',
        country: '',
        country_code: '',
        city: '',
        postal_code: '',
        address_line1: '',
        address_line2: '',
        state: '',
        tax_id: '',
      } as Customer,
      billing: null as BillingInformation | null,
      usedStorage: 0,
      totalStorage: 0,
    };
  },
  computed: {
    groupPath(): string {
      return this.$route.params.groupPath;
    },
    currentPlan(): string {
      return this.customer?.plan ?? BillingPlan.FREE;
    },
    showBillingInformation(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      return this.customer ? true : false;
    },
    showPlans(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      if (!this.group) {
        return false;
      } if (this.customer) {
        return this.customer.plan === BillingPlan.FREE;
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
        this.group = await this.$kernelService.fetchGroup(this.groupPath);
        this.billing = await this.$kernelService.fetchBillingInformation(this.group.namespace_id!);
        this.usedStorage = this.billing.used_storage;
        this.totalStorage = this.billing.total_storage;
        this.resetCustomer(this.billing.customer);
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
      let taxId: string | null = this.customer.tax_id;
      if (taxId === '' || !isEu(this.customer.country_code)) {
        taxId = null;
      }
      const input: UpdateBillingInformation = {
        namespace_id: this.group!.namespace_id!,
        name: this.customer.name,
        email: this.customer.email,
        country_code: this.customer.country_code,
        city: this.customer.city,
        postal_code: this.customer.postal_code,
        address_line1: this.customer.address_line1,
        address_line2: this.customer.address_line2,
        state: this.customer.state,
        tax_id: taxId,
      };

      try {
        this.billing = await this.$kernelService.updateBillingInformation(input);
        this.resetCustomer(this.billing.customer);
        if (this.selectedPlan !== BillingPlan.FREE) {
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

      const input: GetCheckoutSession = {
        namespace_id: this.group!.namespace_id!,
        plan,
      };

      try {
        await this.$kernelService.gotoCheckoutSession(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    resetCustomer(customer: Customer | null) {
      if (customer) {
        this.customer = {
          plan: customer.plan,
          name: customer.name,
          email: customer.email,
          country: customer.country,
          country_code: customer.country_code,
          city: customer.city,
          postal_code: customer.postal_code,
          address_line1: customer.address_line1,
          address_line2: customer.address_line2,
          state: customer.state,
          tax_id: customer.tax_id ?? '',
        };
      } else {
        this.customer = {
          plan: BillingPlan.FREE,
          name: '',
          email: this.$store.state.me?.email ?? '',
          country: '',
          country_code: '',
          city: '',
          postal_code: '',
          address_line1: '',
          address_line2: '',
          state: '',
          tax_id: '',
        };
      }
    },
    planSelected(plan: BillingPlan) {
      // get billing information
      // then, get checkoutSession
      this.selectedPlan = plan;
      if (!this.billing?.customer) {
        this.showBillingInformationDialog = true;
      } else {
        this.gotoCheckoutSession(this.selectedPlan);
      }
    },
    async gotoBillingPortal() {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.gotoCustomerPortal(this.group!.namespace_id!);
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
