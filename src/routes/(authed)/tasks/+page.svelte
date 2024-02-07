<script lang="ts">
    import type {PageData} from './$types'

    /** @type {import('./$types').PageData} */
    export let data: PageData
</script>

<div class="p-4">
  <h1 class="text-2xl underline underline-offset-4">タスク一覧</h1>
  <button class="btn-primary rounded-3xl font-bold my-2"><a href="/tasks/add">+</a></button>

  <div class="table-auto">
    <table>
      <thead class="border-2">
      <tr class="bg-blue-300 border-blue-600">
        <th class="border-2 px-1 py-1 text-left">タスク名</th>
        <th class="border-2 px-1 py-1 text-left">詳細</th>
        <th class="border-2 px-1 py-1 text-left">担当者</th>
        <th class="border-2 px-1 py-1 text-left">親タスク</th>
        <th class="border-2 px-1 py-1 text-left">状態</th>
        <th class="border-2 px-1 py-1 text-left">進捗率[%]</th>
        <th class="border-2 px-1 py-1 text-left">開始日</th>
        <th class="border-2 px-1 py-1 text-left">期限日</th>
        <th class="border-2 px-1 py-1 text-left">予定工数[H]</th>
        <th class="border-2 px-1 py-1 text-left">実工数[H]</th>
        <th class="border-2 px-1 py-1 text-left">計画予算[\]</th>
        <th class="border-2 px-1 py-1 text-left">実コスト[\]</th>
      </tr>
      </thead>
      <tbody>
      {#each data.task_list as item (item.task_id)}
        <tr class="">
          <td class="border-2 py-1 content-center"><span class="p-2">{item.title}</span></td>
          <td class="border-2 py-1">{item.description ?? ''}</td>
          <td class="border-2 py-1">{item.assignee_resource_name ?? ''}</td>
          <td class="border-2 py-1">{item.parent_task_title ?? ''}</td>
          <td class="border-2 py-1"><div class="bg-amber-400 rounded-xl px-4 py-0.5">{item.task_status_name}</div></td>
          <td class="border-2 py-1 text-right"><span class="p-2">{item.progress_rate}</span></td>
          <td class="border-2 py-1">{item.start_date ?? ''}</td>
          <td class="border-2 py-1">{item.due_date ?? ''}</td>
          <td class="border-2 py-1">{item.estimated_time ?? ''}</td>
          <td class="border-2 py-1">{item.actual_time ?? ''}</td>
          <td class="border-2 py-1">{item.planed_value ?? ''}</td>
          {#if (item.estimated_time != null)}
            <td class="border-2 py-1">{(item.estimated_time * 650000/160).toLocaleString()}</td>
          {:else}
            <td class="border-2 py-1"></td>
          {/if}
        </tr>
      {/each}
      </tbody>
    </table>
  </div>
</div>
