<script lang="ts">
    import type {PlannedValueChanges} from "$components/PlannedValueChanges";
    import type {EvmHistory} from "$components/EvmHistory";

    export let planned_value_changes: PlannedValueChanges[];
    export let evm_histories: EvmHistory[];

    let datasets: { dateList: string[], pvList: (number | null)[], acList: (number | null)[], evList: (number | null)[] } = {
        dateList: [],
        pvList: [],
        acList: [],
        evList: [],
    }

    $: datasets = generate_dataset();

    function merged_date_list(): string[] {
        let unique_data_list: string[] = [];
        let last_value: null | string = null;
        planned_value_changes.map(v => v.date)
            .concat(evm_histories.map(v => v.date))
            .sort()
            .forEach(v => {
                if (v != last_value) {
                    unique_data_list.push(v);
                    last_value = v;
                }
            });
        return unique_data_list;
    }

    function generate_dataset(): { dateList: string[], pvList: (number | null)[], acList: (number | null)[], evList: (number | null)[] } {
        const dateList = merged_date_list();
        return dateList.reduce((accumulator, date) => {
            const pv = planned_value_changes.find(v => v.date == date);
            accumulator.pvList.push(pv ? pv.planned_value : null);

            const evmHistory = evm_histories.find(v => v.date == date);
            accumulator.acList.push(evmHistory ? evmHistory.actual_cost : null);
            accumulator.evList.push(evmHistory ? evmHistory.earned_value : null);

            return accumulator;
        }, {
            dateList: dateList,
            pvList: new Array<number | null>(),
            acList: new Array<number | null>(),
            evList: new Array<number | null>(),
        });

    }
</script>

<table>
  <caption>PV</caption>
  <thead class="border-2">
  <tr class="bg-blue-300 border-blue-600">
    <th class="border-2 px-1 py-1 text-left">日付</th>
    <th class="border-2 px-1 py-1 text-left">PV(計画値) [&yen;]</th>
    <th class="border-2 px-1 py-1 text-left">AC(実コスト) [&yen;]</th>
    <th class="border-2 px-1 py-1 text-left">EV(出来高) [&yen;]</th>
  </tr>
  </thead>
  <tbody>
  {#each datasets.dateList as item, index}
    <tr class="">
      <td class="border-2 py-1 content-center">{item}</td>
      <td class="border-2 py-1 content-center">{datasets.pvList[index]?.toLocaleString() ?? ''}</td>
      <td class="border-2 py-1 content-center">{datasets.acList[index]?.toLocaleString() ?? ''}</td>
      <td class="border-2 py-1 content-center">{datasets.evList[index]?.toLocaleString() ?? ''}</td>
    </tr>
  {/each}
  </tbody>
</table>
