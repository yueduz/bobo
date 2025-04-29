<script lang="ts">
  import TypeSel from "./TypeSel.svelte";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import _ from "lodash-es";
  import { getContext, onMount } from "svelte";
  let groups = $state(1);
  let dataType = $state(["int32"]);
  let isStart = $state(false);
  let isPause = $state(false);
  let littleEndian = $state(true);
  let upd: any = getContext("upd");
  let dataLength = 0;

  function compareUint8Arrays(arr1:Uint8Array, arr2:Uint8Array) {
    if (arr1.length !== arr2.length) {
      return false;
    }
    for (let i = 0; i < arr1.length; i++) {
      if (arr1[i] !== arr2[i]) {
        return false;
      }
    }
    return true;
  }

  function getTypeNum(
    dataview: DataView,
    s: String,
    offset: number,
    littleEndian: boolean
  ) {
    switch (s) {
      case "int8":
        return [dataview.getInt8(offset), 1];
      case "int16":
        return [dataview.getInt16(offset, littleEndian), 2];
      case "int32":
        return [dataview.getInt32(offset, littleEndian), 4];
      case "uint8":
        return [dataview.getUint8(offset), 1];
      case "uint16":
        return [dataview.getUint16(offset, littleEndian), 2];
      case "uint32":
        return [dataview.getUint32(offset, littleEndian), 4];
      case "float32":
        return [dataview.getFloat32(offset, littleEndian), 4];
      case "float64":
        return [dataview.getFloat64(offset, littleEndian), 8];
      default:
        console.log("类型不匹配");
        return null;
    }
  }

  listen<number>("pipe_close_event", (event) => {
    console.log("pipe_close_event");

    isStart = false;
  });

  async function start() {
    if (isStart == false) {
      upd.setSeries(dataType.length - 1);
      isStart = true;
      console.log("接收开始 ");
      let matchTa = new Uint32Array([0xfffffff0, 0x0fffffff]);
      const matchUint8Ta = new Uint8Array(matchTa.buffer);

      let revArr = new Array(dataLength).fill(0);
      let revArrI = 0;
      //设置一个开始标记,匹配到开始标记时重新分组

      const onEvent = new Channel<number[]>();
      onEvent.onmessage = (message: number[]) => {

        // 遍历message数组
        for (let i = 0; i < message.length; i++) {
          revArr.shift();
          revArr.push(message[i]);

          if (
            compareUint8Arrays(new Uint8Array(revArr.slice(-16)), matchUint8Ta)
          ) {
            revArrI = 0;
            // 检测到开始标记
            console.log("检测到开始标记\n");
            console.log(new Uint8Array(revArr.slice(-16)));
            console.log(matchUint8Ta);
          } else {
            revArrI++;
            // console.log("没有检测到开始标记\n");
          }

          if (revArrI == dataLength) {
            // 使用 Uint8Array 从普通数组创建一个 TypedArr"build": "tsc && rollup -c",ay
            const typedArray = new Uint8Array(revArr);
            const arrayBuffer = typedArray.buffer;
            const dataview = new DataView(arrayBuffer);
            let varr: any = [];
            let offset = 0;
            for (let i1 = 0; i1 < groups; i1++) {
              varr.push([]);
              for (let i2 = 0; i2 < dataType.length - 1; i2++) {
                const element = dataType[i2];
                var tmp = getTypeNum(dataview, element, offset, littleEndian);
                if (tmp != null) {
                  varr[i1].push(tmp[0]);

                  offset += tmp[1];
                  // console.log("offset" + offset);
                } else {
                  console.log("读取类型数组时错误");
                }
              }
            }
            revArrI = 0;
            upd.upd(varr);
          }
        }
      };

      await invoke("start_fifo", {
        onEvent,
        length: dataLength * groups,
      });
    } else {
      console.log("接收结束 ");

      isStart = false;
      await invoke("stop_tcp");
    }
  }

  async function read_fifo_pause() {
    if (isPause == false) {
      try {
        isPause = true;
        await invoke("read_fifo_pause");
      } catch (error) {
        console.error("Failed to pause FIFO read:", error);
      }
    } else {
      try {
        isPause = false;
        await invoke("read_fifo_continue");
      } catch (error) {
        console.error("Failed to pause FIFO read:", error);
      }
    }
  }

  // // 计算各类型长度和
  $effect(() => {
    if (dataType[dataType.length - 1] != "Type?") {
      dataType.push("Type?");
    }
    for (let index = 0; index < dataType.length - 1; index++) {
      const element = dataType[index];
      if (element == "Type?") {
        dataType.splice(index, 1);
      }
    }

    dataLength = 0;
    for (let index = 0; index < dataType.length - 1; index++) {
      const element = dataType[index];
      const buffer = new ArrayBuffer(8); //无用,填空
      let dataview = new DataView(buffer); //无用,填空
      let offset = 0; //无用,填空
      var tmp = getTypeNum(dataview, element, offset, littleEndian);
      if (tmp) {
        dataLength = dataLength + tmp[1];
      }
    }
  });

  import { Tooltip } from "melt/builders";

  const tooltip = new Tooltip({
    forceVisible: true,
    openDelay: 0,
  });
</script>

<div class="m-3 h-[4em]">
  <button onclick={start} class={isStart ? "butEn start" : "start"}>
    开始
  </button>
  <!-- <button
    onclick={read_fifo_pause}
    disabled={!isStart}
    class={isPause ? "butEn pause" : "pause"}
  >
    暂停
  </button> -->
  <button onclick={upd.scroll} disabled={!isStart} class="but">滚动</button>
  <button onclick={upd.autozoom} disabled={!isStart} class="but">适应</button>
  <button
    onclick={() => (littleEndian = !littleEndian)}
    class={littleEndian ? "butEn" : ""}
  >
    小端
  </button>
  <div
    class=" text-emerald-400 h-[4em] inline-flex items-center align-middle mx-[1em]"
  >
    <p>组队:</p>
    <input
      {...tooltip.trigger}
      disabled={isStart}
      type="number"
      placeholder="数据长度"
      class="max-w-xs w-[70px] align-middle border-1"
      bind:value={groups}
    />
    <div
      {...tooltip.content}
      class="rounded-xl bg-emerald-100 p-0 shadow-xl dark:bg-gray-800"
    >
      <div {...tooltip.arrow} class="size-2"></div>
      <p class="px-4 py-1 bg-emerald-100 text-emerald-400 rounded-xl">
        组队传输提高性能<br />单次传输总长=组队数 * 类型总长
      </p>
    </div>
  </div>

  <div
    class=" border-blue-500 inline-flex items-center align-middle h-[4em] mx-[1em]"
  >
    <div class="text-blue-500 inline-block">数据<br />类型</div>
    {#each dataType as item, i}
      {#if i == dataType.length - 1}
        <TypeSel
          bind:value={dataType[i]}
          disabled={isStart}
          autofocus
          class="typeBut"
        />
      {:else}
        <TypeSel bind:value={dataType[i]} disabled={isStart} />
      {/if}
    {/each}
  </div>
</div>

<style>
  .start {
    background: linear-gradient(145deg, #ffafaf, #d41515);
  }
  .start:hover {
    background: linear-gradient(145deg, #d41515, #ffafaf);
  }

  .pause {
    background: linear-gradient(145deg, #fff2aa, #d6b317);
  }
  .pause:hover {
    background: linear-gradient(145deg, #d6b317, #fff2aa);
  }

  /* 禁用按钮样式 */
  .pause:disabled {
    cursor: not-allowed; /* 更改鼠标指针为“不允许” */
    box-shadow: none;
    background: none;
    background-color: #d6b317;
    /* background: none; */
  }

  [data-melt-tooltip-content] {
    border: 0;

    position: absolute;
    pointer-events: none;
    opacity: 0;

    transform: scale(0.9);

    transition: 0.3s;
    transition-property: opacity, transform;
  }

  [data-melt-tooltip-content][data-open] {
    pointer-events: auto;
    opacity: 1;

    transform: scale(1);
  }
</style>
