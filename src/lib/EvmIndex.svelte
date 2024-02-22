<script lang="ts">
  import {EarnedValueManagementInfo} from "$components/EarnedValueManagementInfo";

  let scheduleVariance = 0;
  let costVariance = 0;
  let schedulePerIndex = 0;
  let costPerIndex = 0;
  let estimateToCompletion: number | undefined = undefined

  export let evmInfo: EarnedValueManagementInfo;
  export let budgetAtCompletion: number | null = null;

  $: {
    scheduleVariance = evmInfo.earned_value - evmInfo.planned_value;
    costVariance = evmInfo.earned_value - evmInfo.actual_cost;
    schedulePerIndex = evmInfo.earned_value / evmInfo.planned_value;
    costPerIndex = evmInfo.earned_value / evmInfo.actual_cost;
    estimateToCompletion = budgetAtCompletion ? (budgetAtCompletion - evmInfo.earned_value) / costPerIndex : undefined;
  }

</script>

<div class="table-auto my-4">
  <table>
    <caption class="text-left">現時点のEVM計測値</caption>
    <thead class="border-2">
    <tr class="bg-blue-300 border-blue-600">
      <th class="border-2 px-1 py-1 text-left w-56">BAC(完成時総予算) [&yen;]</th>
      <th class="border-2 px-1 py-1 text-left w-56">PV(計画値) [&yen;]</th>
      <th class="border-2 px-1 py-1 text-left w-56">AC(実コスト) [&yen;]</th>
      <th class="border-2 px-1 py-1 text-left w-56">EV(出来高) [&yen;]</th>
    </tr>
    </thead>
    <tbody>
    <tr class="">
      <td class="border-2 py-1 content-center">{budgetAtCompletion?.toLocaleString() ?? ""}</td>
      <td class="border-2 py-1 content-center">{evmInfo.planned_value.toLocaleString()}</td>
      <td class="border-2 py-1 content-center">{evmInfo.actual_cost.toLocaleString()}</td>
      <td class="border-2 py-1 content-center">{evmInfo.earned_value.toLocaleString()}</td>
    </tr>
    </tbody>
  </table>
  <table>
    <caption class="text-left">現時点のEVM指標値</caption>
    <thead class="border-2">
    <tr class="bg-blue-300 border-blue-600">
      <th class="border-2 px-1 py-1 text-left w-48">SV(スケジュール差異)</th>
      <th class="border-2 px-1 py-1 text-left w-48">CV(コスト差異)</th>
      <th class="border-2 px-1 py-1 text-left w-48">SPI(スケジュール効率指数)</th>
      <th class="border-2 px-1 py-1 text-left w-48">CPI(コスト効率指数)</th>
      <th class="border-2 px-1 py-1 text-left w-48">ETC(残作業コスト予測)</th>
    </tr>
    </thead>
    <tbody>
    <tr class="">
      <td class="border-2 py-1 content-center">{scheduleVariance.toLocaleString()}</td>
      <td class="border-2 py-1 content-center">{costVariance.toLocaleString()}</td>
      <td class="border-2 py-1 content-center">{schedulePerIndex.toFixed(3)}</td>
      <td class="border-2 py-1 content-center">{costPerIndex.toFixed(3)}</td>
      <td class="border-2 py-1 content-center">{estimateToCompletion?.toLocaleString() ?? ""}</td>
    </tr>
    </tbody>
  </table>
</div>