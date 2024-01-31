<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {goto} from "$app/navigation";
    import type {PageData} from './$types';

    /** @type {import('./$types').PageData} */
    export let data: PageData;

    let title = ""
    let assigneeResourceId: null | number  = null
    let taskStatusId: 0;  //未着手のtask_status_id
    let progressRate = 0;
    let msg = "";

    async function addTask() {
        await invoke<{ msg: String }>("task_add", {
            title,
            assigneeResourceId,
            taskStatusId,
            progressRate,
        }).then(value => {
            console.log("追加成功")
            goto('/tasks');
        }).catch(reason => {
            console.error("認証失敗", reason)
            msg = reason;
        })
    }
</script>

<div>
  <h1>タスク追加</h1>
  <form class="row" on:submit|preventDefault="{addTask}">
    <div class="flex flex-row px-4 py-2">
      <label for="title" class="form-label basis-48">タスク名</label><br>
      <input type="text" id="title" name="title" class="basis-full" required bind:value={title}><br>
    </div>
    <div class="flex flex-row px-4 py-2">
      <label for="description" class="form-label basis-48">詳細</label><br>
      <textarea id="description" name="description" rows="4" cols="50" class="basis-full"></textarea>
    </div>
    <div class="flex flex-row px-4 py-2">
      <!-- TODO ユーザのリストからselect生成 絞り込み可能なやつ -->
      <label for="assignee_resource_id" class="form-label basis-48">担当者</label><br>
      <input type="number" id="assignee_resource_id" name="assignee_resource_id" class="basis-full" bind:value={assigneeResourceId}><br>
    </div>
    <div class="flex flex-row px-4 py-2">
      <label for="parent_task_id" class="form-label basis-48">親タスク</label><br>
      <input type="number" id="parent_task_id" name="parent_task_id" class="basis-full"><br>
    </div>
    <div class="flex flex-row px-4 py-2">
      <!-- TODO 全タスクのリストからselect生成 絞り込み可能なやつ -->
      <label for="start_date" class="form-label basis-48">開始日</label><br>
      <input type="date" id="start_date" name="start_date" class="basis-full"><br>
    </div>
    <div class="flex flex-row px-4 py-2">
      <label for="due_date" class="form-label basis-48">期限日</label><br>
      <input type="date" id="due_date" name="due_date" class="basis-full"><br>
    </div>
    <div class="flex flex-row px-4 py-2">
      <label for="task_status_id" class="form-label basis-48">状態</label><br>
      <select id="task_status_id" name="task_status_id" class="basis-full" required bind:value={taskStatusId}>
        {#each data.task_status_list as item (item.task_status_id)}
          <option value="{item.task_status_id}">{item.title}</option>
        {/each}
      </select><br/>
    </div>
    <div class="flex flex-row px-4 py-2">
      <label for="progress_rate" class="form-label basis-48">作業進捗度合</label><br>
      <select id="progress_rate" name="progress_rate" class="basis-full" required bind:value={progressRate}>
        {#each Array.from(Array(11), (v, x) => x * 10) as val}
          <option value="{val}">{val}</option>
        {/each}
      </select>
    </div>
    <div class="flex flex-row px-4 py-2">
    </div>
    <div class="flex flex-row px-4 py-2">
    </div>
    <button type="submit" value="Submit" class="btn-primary">追加</button>
  </form>
  <p>{msg}</p>
</div>
