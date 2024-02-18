<script lang="ts">
    import type {PlannedValueChanges} from "components/PlannedValueChanges";
    import { Line } from 'svelte-chartjs';

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
    export let data: PlannedValueChanges[]

    function labels() {
        return data.map(v => v.date)
    }

    function planned_values() {
        return data.map(v => v.planned_value)
    }

    function dataset() {
        return {
            labels: labels(),
            datasets: [{
                label: 'PV（計画値）',
                data: planned_values(),
                fill: false,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1
            }]
        }
    }
</script>

<Line data={dataset()}/>
