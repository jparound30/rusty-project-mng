<script lang="ts">
    import type {PlannedValueChanges} from "$components/PlannedValueChanges";
    import type {EvmHistory} from "$components/EvmHistory";
    import {Line} from 'svelte-chartjs';

    import {
        Chart as ChartJS,
        Title,
        Tooltip,
        Legend,
        LineElement,
        LinearScale,
        PointElement,
        CategoryScale,
    } from 'chart.js';

    ChartJS.register(
        Title,
        Tooltip,
        Legend,
        LineElement,
        LinearScale,
        PointElement,
        CategoryScale
    );

    export let planned_value_changes: PlannedValueChanges[];
    export let evm_histories: EvmHistory[];

    let datasets: {
        dateList: string[],
        pvList: (number | null)[],
        acList: (number | null)[],
        evList: (number | null)[]
    } = {
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

    function generate_dataset(): {
        dateList: string[],
        pvList: (number | null)[],
        acList: (number | null)[],
        evList: (number | null)[]
    } {
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

    function dataset() {
        let ds = generate_dataset();
        return {
            labels: ds.dateList,
            datasets: [
                {
                    label: 'PV（計画値）',
                    data: ds.pvList as number[],  // svelte-chartjsの型定義がイケてなくてnull(skip)を含めないのでキャスト
                    fill: false,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.1,
                    spanGaps: true,
                },
                {
                    label: 'AC（実コスト）',
                    data: ds.acList as number[],  // svelte-chartjsの型定義がイケてなくてnull(skip)を含めないのでキャスト
                    fill: false,
                    borderColor: 'rgb(75,192,81)',
                    tension: 0.1,
                    spanGaps: true,
                },
                {
                    label: 'EV（出来高）',
                    data: ds.evList as number[],  // svelte-chartjsの型定義がイケてなくてnull(skip)を含めないのでキャスト
                    fill: false,
                    borderColor: 'rgb(192,89,75)',
                    tension: 0.1,
                    spanGaps: true,
                }
            ]
        };
    }
</script>

<Line data={dataset()}/>
