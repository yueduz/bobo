<script lang="ts">
  import { Select } from "melt/builders";
	let { value = $bindable(),disabled, ...props } = $props();
  const options = [
    "int8",
    "uint8",
    "int16",
    "uint16",
    "int32",
    "uint32",
    "float32",
    "float64",
    "Type?",
  ] as const;
  type Option = (typeof options)[number];

  const select = new Select<Option>();
    select.value = value
  // 定义组件事件
  // 监听 count 的变化
  $effect(() => {
    console.log(`Count 的值改变了：${select.value }`);
  value = select.value;

    // 在这里执行你需要在 count 变化时进行的操作
  });


</script>
<style>
   .typeBut {
    background: linear-gradient(145deg, #8bcaf5, #1770d6);
  }
  .typeBut:hover {
    background: linear-gradient(145deg, #1770d6, #8bcaf5);
  }

  .typeBut:disabled {
    cursor: not-allowed; /* 更改鼠标指针为“不允许” */
    box-shadow: none;
    background: none;
    background-color: #5b91ce;
    /* background: none; */
  }

  .my-list{
    border-radius: 10px;
    background: linear-gradient(145deg, #8bcaf5, #1770d6);
    box-shadow:
      5px 5px 10px #747474,
      -5px -5px 10px #ffffff;
    margin: 10px;
    background-color: #1770d6;
  }

  .my-list div:hover{
    background: linear-gradient(145deg, #8bcaf5, #1770d6);
    box-shadow:
      5px 5px 10px #747474,
      -5px -5px 10px #ffffff;
      cursor: pointer; 

  }
</style>

<!-- <label for={select.ids.trigger}>类型</label> -->
<button {...select.trigger} disabled={disabled}  class="typeBut" >
  {select.value ?? "Select an anime"}
</button>

<div class="my-list" {...select.content}>
  {#each options as option}
    <div class="m-0 px-3 py-1  hover:bg-blue-50" {...select.getOption(option)}>
      {option}
    </div>
  {/each}
</div>
