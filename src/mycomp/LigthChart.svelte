<script lang="ts">
  import { onMount } from "svelte";
  import { getContext } from "svelte";
  import TimeChart from "timechart";
  import { Color } from "modern-color";
  import { scaleLinear, scaleUtc } from "d3-scale";
  import { DataPointsBuffer } from "timechart/core/dataPointsBuffer.js";
  let chartDom: any;
  var d = 0;
  let upd: any = getContext("upd");
  let chart: any = {};
  let series_arr: any = [];

  function formatMilliseconds(ms: number) {
    // 计算总秒数
    const totalSeconds = Math.floor(ms / 1000);

    // 计算小时、分钟、秒和剩余的毫秒数
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    const remainingMilliseconds = ms % 1000;

    // 格式化并返回时间字符串
    return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}.${String(remainingMilliseconds).padStart(3, "0")}`;
  }

  // 示例用法
  const milliseconds = 3661030; // 1小时1分钟1秒30毫秒
  const formattedTime = formatMilliseconds(milliseconds);
  console.log(formattedTime); // 输出: "01:01:01.030"
  onMount(() => {
    if (chart.dispose) {
      chart.dispose();
    }

    chart = new TimeChart(chartDom, {
      realTime: true,
      series: [
        {
          data: DataPointsBuffer._from_array([
            { x: 0, y: 0 },
            { x: 1, y: 1 },
          ]),
        },
      ],
      zoom: {
        x: {
          autoRange: false,
        },
        y: {
          autoRange: false,
          minDomain: 100,
          maxDomain: 1000,
          // minDomainExtent:100,
          // maxDomainExtent:1000
        },
      },
    });

    upd.upd = function (data: any) {
      // console.log(data.length);

      for (let i1 = 0; i1 < data.length; i1++) {
        const element = data[i1];
        d++;

        for (let i2 = 0; i2 < element.length; i2++) {
          series_arr[i2].data.push({
            x: d,
            y: element[i2],
          });
        }
      }

      chart.update();
    };
    upd.scroll = function () {
      console.log("scroll");
      chart.options.realTime = true;
      // chart.update();
    };

    upd.autozoom = function () {
      (chart.options.xRange = "auto"), (chart.options.yRange = "auto");
      chart.update();
    };

    upd.setSeries = function (len: number) {
      console.log(`setSeries ${len}`);
      d = 1;
      if (chart.dispose) {
        console.log(`chart.dispose() ${chart.dispose()}`);
      }
      series_arr = [];
      for (let i = 0; i < len; i++) {
        series_arr.push({
          name: "ch" + i,
          lineWidth: 1,
          data: [],
          lineType: TimeChart.LineType.NativeLine,
          color: Color.parse({
            h: (360 + 38 * i) % 359,
            s: 100,
            v: 80,
            a: 1,
          }).toString("hex"),
        });
      }
      chart = new TimeChart(chartDom, {
        series: series_arr,
        realTime: true,
        tooltip: {
          enabled: true,
          xFormatter: (x) => {
            return formatMilliseconds(x);
          },
        },
        baseTime: 0,
        xScaleType: scaleLinear,
        zoom: {
          x: {
            autoRange: false,
            minDomain: 5 * 1000,
          },
          y: {
            autoRange: false,
            minDomain: 0,
            maxDomain: 250,
          },
        },
      });
      setCursorStyle(chartDom);
    };
    setCursorStyle(chartDom);
  });


  function setCursorStyle(dom:HTMLDivElement){
      // 假设最外层的宿主元素是 'outerHostElement'
      let outerShadowRoot = dom.shadowRoot
      if (outerShadowRoot) {
        // 找到创建了内部影子 DOM 的宿主元素，假设它的选择器是 '.inner-host'
        const innerHostElement = outerShadowRoot.querySelector("chart-legend");
        if (innerHostElement) {
          const innerShadowRoot = innerHostElement.shadowRoot;
          if (innerShadowRoot) {
            // 现在我们位于最内层的影子 DOM 中，可以找到 '.item' 元素并设置样式
            const items = innerShadowRoot.querySelectorAll(".item") as NodeListOf<HTMLElement>;;
            items.forEach((item:HTMLElement) => { 
              item.style.cursor = "pointer";
            });
          } else {
            console.log("内部宿主元素没有影子 DOM");
          }
        } else {
          console.log("在外部影子 DOM 中找不到 chart-legend 元素");
        }
      } else {
        console.log("最外层的元素没有影子 DOM");
      }
  }
</script>

<div bind:this={chartDom} id="timechart"></div>

<style lang="css">
  div {
    height: 100vh;
    width: 100vw;
  }
</style>
