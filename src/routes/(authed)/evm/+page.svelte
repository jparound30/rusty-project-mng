<script lang="ts">
    import type {PageData} from './$types'
    import EvmTable from "$lib/EvmTable.svelte";
    import EvmChart from "$lib/EvmChart.svelte";
    import EvmIndex from "$lib/EvmIndex.svelte";

    /** @type {import('./$types').PageData} */
    export let data: PageData

</script>

<div class="p-4">
  <h1 class="text-2xl underline underline-offset-4">進捗状況</h1>

  <EvmIndex evmInfo="{data.evm_info}" budgetAtCompletion="{data.planned_value_changes[(data.planned_value_changes.length) - 1].planned_value}"/>
  <EvmChart evm_histories="{data.evm_histories}" planned_value_changes="{data.planned_value_changes}" />
  <EvmTable evm_histories="{data.evm_histories}" planned_value_changes="{data.planned_value_changes}" />
</div>
