<script lang="ts">
	import '../app.css';
	import { setContext } from "svelte";
	import LigthChart from '../mycomp/LigthChart.svelte';
	
	let { children } = $props();
	
	setContext("upd", {});
</script>

<div class="app">
	<main>
		<LigthChart />
		{@render children()}
	</main>
</div>
