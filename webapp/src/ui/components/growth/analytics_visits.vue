<template>
  <v-card flat>
    <v-card-text>
      <div id="analytics-visits-chart" />
    </v-card-text>
  </v-card>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import echarts from 'echarts';
import { AnalyticsVisit } from '@/api/graphql/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsVisits',
  props: {
    visits: {
      type: Array as PropType<AnalyticsVisit[]>,
      required: true,
    },
  },
  data() {
    return {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      chart: null as any,
    };
  },
  mounted() {
    this.init();
    window.addEventListener('resize', this.handleResize);
  },
  beforeDestroy() {
    window.removeEventListener('resize', this.handleResize);
  },
  methods: {
    init() {
      this.chart = echarts.init(document.getElementById('analytics-visits-chart') as HTMLDivElement);
      const xAxisData = this.visits.map((visit: AnalyticsVisit) => visit.date);
      const viewsData = this.visits.map((visit: AnalyticsVisit) => visit.views);
      const visitorsData = this.visits.map((visit: AnalyticsVisit) => visit.visitors);

      this.chart.setOption({
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            type: 'cross',
            label: {
              backgroundColor: '#6a7985',
            },
          },
        },
        legend: {
          data: ['Views', 'Visitors'],
        },
        xAxis: {
          type: 'category',
          boundaryGap: false,
          data: xAxisData,
        },
        yAxis: {
          type: 'value',
        },
        series: [
          {
            name: 'Views',
            data: viewsData,
            areaStyle: {},
            type: 'line',
          },
          {
            name: 'Visitors',
            data: visitorsData,
            areaStyle: {},
            type: 'line',
          },
        ],
      });
    },
    handleResize() {
      if (this.chart) {
        this.chart.resize();
      }
    },
  },
});
</script>


<style lang="scss" scoped>
#analytics-visits-chart {
  height: 420px;
}
</style>
