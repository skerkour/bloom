<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12" sm="6">
        <v-text-field
          label="Full name"
          :value="value.name"
          @input="updateValue('name', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field
          label="Email"
          :value="value.email"
          @input="updateValue('email', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-select
        :items="countrySelectItems"
        label="Country"
        :value="value.country_code"
        @input="updateCountry"
        :disabled="loading"
        :readonly="readonly"
      />

      <v-col cols="12" sm="4">
        <v-text-field
          label="City"
          :value="value.city"
          @input="updateValue('city', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-col cols="12" sm="4">
        <v-text-field
          label="Postal or Zip code"
          :value="value.postal_code"
          @input="updateValue('postal_code', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-col cols="12">
        <v-text-field
          label="Address line 1"
          :value="value.address_line1"
          @input="updateValue('address_line1', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-col cols="12" sm="8">
        <v-text-field
          label="Address line 2 (optional)"
          :value="value.address_line2"
          @input="updateValue('address_line2', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-col cols="12" sm="4">
        <v-text-field
          label="State, county, province or region"
          :value="value.state"
          @input="updateValue('state', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

      <v-col cols="12" sm="6" v-if="isEu">
        <v-text-field
          label="VAT number (Optional)"
          :value="value.tax_id"
          @input="updateValue('tax_id', $event)"
          :disabled="loading"
          :readonly="readonly"
        />
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Customer } from '@/domain/kernel/model';
import { PropType } from 'vue';
import countries from '@/app/utils/countries';
import { isEu } from '@/app/utils/eu';

export default VueApp.extend({
  name: 'BCustomer',
  props: {
    value: {
      type: Object as PropType<Customer>,
      required: true,
    },
    loading: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    readonly: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  computed: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    countrySelectItems(): any[] {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return countries.map((country: any) => {
        country.text = country.name;
        country.value = country.code;
        return country;
      });
    },
    isEu(): boolean {
      return isEu(this.value.country_code);
    },
  },
  methods: {
    updateValue(key: string, value: string) {
      this.$emit('input', { ...this.value, [key]: value });
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    updateCountry(selectedCountry: any) {
      const countryCode = selectedCountry;
      this.$emit('input', { ...this.value, country_code: countryCode });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
