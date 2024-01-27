<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {goto} from "$app/navigation";
    import type { PageData} from './$types';

    /** @type {import('./$types').PageData} */
    export let data: PageData;

    let taskName = ""
    let assignee = ""
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
    <input id="task-name-input" placeholder="タスク名" bind:value={taskName}/>
    <input id="assignee-input" placeholder="担当者" bind:value={assignee}/>
    <select>
      {#each data.task_status_list as list}
        <option value="{list.task_status_id}">{list.title}</option>
      {/each}
    </select>
    <button type="submit">追加</button>
  </form>
  <p>{msg}</p>
</div>
