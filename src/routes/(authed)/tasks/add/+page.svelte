<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import {goto} from "$app/navigation"
    import type {PageData} from './$types'

    /** @type {import('./$types').PageData} */
    export let data: PageData

    let title = ""
    let description = ""
    let assigneeResourceId: null | number = null
    let parentTaskId: null | number = null
    let startDate: null | string = null
    let dueDate: null | string = null
    let estimatedTime: null | number = null
    let actualTime: null | number = null
    let planedValue: null | number = null
    let taskStatusId: 0  //未着手のtask_status_id
    let progressRate = 0
    let msg = ""
    $: if (assigneeResourceId != null && estimatedTime != null) {
        const resource = data.resources_list.find(v => v.resource_id == assigneeResourceId)
        if (resource !== undefined) {
            planedValue = estimatedTime * resource.cost_per_month / 160
        }
    }

    async function addTask() {
        await invoke<{ msg: String }>("task_add", {
            title,
            description,
            assigneeResourceId,
            parentTaskId,
            startDate,
            dueDate,
            estimatedTime,
            actualTime,
            planedValue,
            taskStatusId,
            progressRate,
        }).then(value => {
            console.log("追加成功")
            goto('/tasks')
        }).catch(reason => {
            console.error("認証失敗", reason)
            msg = reason
        })
    }
</script>

<div class="p-4">
  <h1 class="text-2xl underline underline-offset-4">タスク追加</h1>
  <form class="row" on:submit|preventDefault="{addTask}">
    <div class="flex flex-row py-2">
      <label for="title" class="form-label basis-48">タスク名</label>
      <input type="text" id="title" name="title" class="basis-full" required bind:value={title}>
    </div>
    <div class="flex flex-row py-2">
      <label for="description" class="form-label basis-48">詳細</label>
      <textarea id="description" name="description" rows="4" cols="50" class="basis-full" bind:value={description}></textarea>
    </div>
    <div class="flex flex-row py-2">
      <label for="assignee_resource_id" class="form-label basis-48">担当者</label>
      <select id="assignee_resource_id" name="assignee_resource_id" class="basis-full" bind:value={assigneeResourceId} >
        <option value="{null}"></option>
        {#each data.resources_list as item (item.resource_id)}
          <option value="{item.resource_id}">{item.name}</option>
        {/each}
      </select>
   </div>
    <div class="flex flex-row py-2">
      <label for="parent_task_id" class="form-label basis-48">親タスク</label>
      <input type="number" id="parent_task_id" name="parent_task_id" class="basis-full" bind:value={parentTaskId}>
    </div>
    <div class="flex flex-row py-2">
      <!-- TODO 全タスクのリストからselect生成 絞り込み可能なやつ -->
      <label for="start_date" class="form-label basis-48">開始日</label>
      <input type="date" id="start_date" name="start_date" class="basis-full" bind:value={startDate}>
    </div>
    <div class="flex flex-row py-2">
      <label for="due_date" class="form-label basis-48">期限日</label>
      <input type="date" id="due_date" name="due_date" class="basis-full" bind:value={dueDate}>
    </div>
    <div class="flex flex-row py-2">
      <label for="estimated_time" class="form-label basis-48">予定工数[H]</label>
      <input type="number" id="estimated_time" name="estimated_time" class="basis-full" bind:value={estimatedTime}>
    </div>
    <div class="flex flex-row py-2">
      <label for="actual_time" class="form-label basis-48">実工数[H]</label>
      <input type="number" id="actual_time" name="actual_time" class="basis-full" bind:value={actualTime}>
    </div>
    <div class="flex flex-row py-2">
      <label for="planed_value" class="form-label basis-48">計画予算[\]</label>
      <input type="number" id="planed_value" name="planed_value" class="basis-full" bind:value={planedValue}>
    </div>
    <div class="flex flex-row py-2">
      <label for="task_status_id" class="form-label basis-48">状態</label>
      <select id="task_status_id" name="task_status_id" class="basis-full" required bind:value={taskStatusId}>
        {#each data.task_status_list as item (item.task_status_id)}
          <option value="{item.task_status_id}">{item.title}</option>
        {/each}
      </select><br/>
    </div>
    <div class="flex flex-row py-2">
      <label for="progress_rate" class="form-label basis-48">作業進捗度合[%]</label>
      <select id="progress_rate" name="progress_rate" class="basis-full" required bind:value={progressRate}>
        {#each Array.from(Array(11), (v, x) => x * 10) as val}
          <option value="{val}">{val}</option>
        {/each}
      </select>
    </div>
    <button type="submit" value="Submit" class="btn-primary">追加</button>
  </form>
  <p>{msg}</p>
</div>
