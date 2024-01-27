<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {goto} from "$app/navigation";
    import type { PageData } from './$types';

    /** @type {import('./$types').PageData} */
    export let data: PageData;

    let taskName = ""
    let assignee = ""
    let taskStatusId: number | null = null;
    let progressRate = 0;
    let msg = "";
    async function addTask() {
        await invoke<{msg: String}>("task_add", {
            taskName,
            assignee
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
    <label for="title">タスク名:</label><br>
    <input type="text" id="title" name="title" required bind:value={taskName}><br>
    <label for="description">詳細:</label><br>
    <textarea id="description" name="description" rows="4" cols="50"></textarea>
    <br>
    <!-- TODO ユーザのリストからselect生成 絞り込み可能なやつ -->
    <label for="assignee_user_id">担当者:</label><br>
    <input type="number" id="assignee_user_id" name="assignee_user_id" bind:value={assignee}><br>
    <!-- TODO 全タスクのリストからselect生成 絞り込み可能なやつ -->
    <label for="parent_task_id">親タスク:</label><br>
    <input type="number" id="parent_task_id" name="parent_task_id"><br>
    <label for="start_date">開始日:</label><br>
    <input type="date" id="start_date" name="start_date"><br>
    <label for="due_date">期限日:</label><br>
    <input type="date" id="due_date" name="due_date"><br>
    <label for="task_status_id">状態:</label><br>
    <select id="task_status_id" name="task_status_id" required bind:value={taskStatusId}>
      {#each data.task_status_list as item (item.task_status_id)}
        <option value="{item.task_status_id}">{item.title}</option>
      {/each}
    </select><br/>
    <label for="progress_rate">作業進捗度合:</label><br>
    <select id="progress_rate" name="progress_rate" required bind:value={progressRate}>
      {#each Array.from(Array(11), (v, x) => x * 10) as val}
        <option value="{val}">{val}</option>
      {/each}
    </select>
    <br><br>
    <button type="submit" value="Submit">追加</button>
  </form>
  <p>{msg}</p>
</div>
