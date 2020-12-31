<template>
  <div id="monitor-response-time-chart" />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import echarts from 'echarts';
import { Ping } from '@/api/graphql/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BMonitorResponseTimeChart',
  props: {
    pings: {
      type: Array as PropType<Ping[]>,
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
      this.chart = echarts.init(document.getElementById('monitor-response-time-chart') as HTMLDivElement);
      const xAxisData = this.pings.map((ping: Ping) => ping.startedAt);
      const responseTimeData = this.pings.map((ping: Ping) => ping.totalDuration);

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
          data: ['Response time (ms)'],
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
            name: 'Response time (ms)',
            data: responseTimeData,
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
#monitor-response-time-chart {
  height: 420px;
}
</style>
