<template>
  <v-card flat>
    <v-card-text>
      <div id="analytics-visits-chart" />
    </v-card-text>
  </v-card>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import * as echarts from 'echarts/core';
import { LineChart, LinesChart } from 'echarts/charts';
import {
  TitleComponent, TooltipComponent,
  GridComponent, LegendComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import { Visit } from '@/domain/analytics/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsVisits',
  props: {
    visits: {
      type: Array as PropType<Visit[]>,
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
      echarts.use([
        TitleComponent, TooltipComponent, GridComponent,
        LinesChart, LineChart, CanvasRenderer, LegendComponent,
      ]);

      this.chart = echarts.init(document.getElementById('analytics-visits-chart') as HTMLDivElement);
      const xAxisData = this.visits.map((visit: Visit) => visit.date);
      const viewsData = this.visits.map((visit: Visit) => visit.views);
      const visitorsData = this.visits.map((visit: Visit) => visit.visitors);

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
            color: '#99ff66',
          },
          {
            name: 'Visitors',
            data: visitorsData,
            areaStyle: {},
            type: 'line',
            color: '#cc66ff',
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
