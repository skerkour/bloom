<template>
  <div id="monitor-uptime-chart" />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import echarts from 'echarts';
import { MonitorUptime } from '@/api/graphql/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BMonitorUptimeChart',
  props: {
    uptime: {
      type: Array as PropType<MonitorUptime[]>,
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
      this.chart = echarts.init(document.getElementById('monitor-uptime-chart') as HTMLDivElement);
      const xAxisData = this.uptime.map((uptime: MonitorUptime) => uptime.date);
      const uptimeData = this.uptime.map((uptime: MonitorUptime) => uptime.uptime);

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
          data: ['Uptime (%)'],
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
            name: 'Uptime (%)',
            data: uptimeData,
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
#monitor-uptime-chart {
  height: 420px;
}
</style>
