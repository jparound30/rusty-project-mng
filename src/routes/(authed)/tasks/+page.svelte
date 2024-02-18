<script lang="ts">
    import type {PageData} from './$types'
    import type {TaskFull} from "$components/TaskFull";

    /** @type {import('./$types').PageData} */
    export let data: PageData

    function calculateActualCost(task: TaskFull): number | null {
        let actualCost: number | null = null;
        if (task.actual_time != null) {

            const resource = data.resources_list.find(s => s.resource_id == task.assignee_resource_id);
            if (resource !== undefined) {
                actualCost = task.actual_time * resource.cost_per_month / 160
            }
        }
        return actualCost;
    }
</script>

<div class="p-4">
  <h1 class="text-2xl underline underline-offset-4">タスク一覧</h1>

  <div class="table-auto my-4">
    <table>
      <caption>現時点のEVM指標値</caption>
      <thead class="border-2">
      <tr class="bg-blue-300 border-blue-600">
        <th class="border-2 px-1 py-1 text-left">PV(計画値) [&yen;]</th>
        <th class="border-2 px-1 py-1 text-left">AC(実コスト) [&yen;]</th>
        <th class="border-2 px-1 py-1 text-left">EV(出来高) [&yen;]</th>
      </tr>
      </thead>
      <tbody>
        <tr class="">
          <td class="border-2 py-1 content-center">{data.evm_info.planned_value.toLocaleString()}</td>
          <td class="border-2 py-1 content-center">{data.evm_info.actual_cost.toLocaleString()}</td>
          <td class="border-2 py-1 content-center">{data.evm_info.earned_value.toLocaleString()}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <!-- TODO こことかに、EVMの指標値の履歴をDBからとってグラフ化するなどすれば、最低限ほしいものはできる -->
  <!-- TODO PVの時系列データの表。データを受け取ってSVGでグラフ化するコンポーネントを作りたいね -->
  <table>
    <caption>PV</caption>
    <thead class="border-2">
    <tr class="bg-blue-300 border-blue-600">
      <th class="border-2 px-1 py-1 text-left">日付</th>
      <th class="border-2 px-1 py-1 text-left">PV(計画値) [&yen;]</th>
    </tr>
    </thead>
    <tbody>
    {#each data.planned_value_changes as item (item.date)}
    <tr class="">
      <td class="border-2 py-1 content-center">{item.date}</td>
      <td class="border-2 py-1 content-center">{item.planned_value.toLocaleString()}</td>
    </tr>
    {/each}
    </tbody>
  </table>

  <a href="/tasks/add">
    <button class="btn-primary rounded-3xl font-bold my-2">+</button>
  </a>

  <div class="table-auto">
    <table>
      <thead class="border-2">
      <tr class="bg-blue-300 border-blue-600">
        <th class="border-2 px-1 py-1 text-left">タスク名</th>
        <th class="border-2 px-1 py-1 text-left">詳細</th>
        <th class="border-2 px-1 py-1 text-left">担当者</th>
        <th class="border-2 px-1 py-1 text-left">親タスク</th>
        <th class="border-2 px-1 py-1 text-left">状態</th>
        <th class="border-2 px-1 py-1 text-left">進捗率 [%]</th>
        <th class="border-2 px-1 py-1 text-left">開始日</th>
        <th class="border-2 px-1 py-1 text-left">期限日</th>
        <th class="border-2 px-1 py-1 text-left">予定工数 [H]</th>
        <th class="border-2 px-1 py-1 text-left">実工数 [H]</th>
        <th class="border-2 px-1 py-1 text-left">計画予算 [&yen;]</th>
        <th class="border-2 px-1 py-1 text-left">実コスト [&yen;]</th>
      </tr>
      </thead>
      <tbody>
      {#each data.task_list as item (item.task_id)}
        <tr class="">
          <td class="border-2 py-1 content-center"><span class="p-2">{item.title}</span></td>
          <td class="border-2 py-1">{item.description ?? ''}</td>
          <td class="border-2 py-1">{item.assignee_resource_name ?? ''}</td>
          <td class="border-2 py-1">{item.parent_task_title ?? ''}</td>
          <td class="border-2 py-1">
            <div class="bg-amber-400 rounded-xl px-4 py-0.5">{item.task_status_name}</div>
          </td>
          <td class="border-2 py-1 text-right"><span class="p-2">{item.progress_rate}</span></td>
          <td class="border-2 py-1">{item.start_date ?? ''}</td>
          <td class="border-2 py-1">{item.due_date ?? ''}</td>
          <td class="border-2 py-1">{item.estimated_time ?? ''}</td>
          <td class="border-2 py-1">{item.actual_time ?? ''}</td>
          <td class="border-2 py-1">{item.planned_value?.toLocaleString() ?? ''}</td>
          {#if (item.actual_time != null)}
            <td class="border-2 py-1">{calculateActualCost(item)?.toLocaleString() ?? "情報不足のため算出不可"}</td>
          {:else}
            <td class="border-2 py-1"></td>
          {/if}
        </tr>
      {/each}
      </tbody>
    </table>
  </div>
</div>
