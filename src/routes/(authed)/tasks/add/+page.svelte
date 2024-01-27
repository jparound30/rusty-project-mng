<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {goto} from "$app/navigation";

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
    <button type="submit">追加</button>
  </form>
  <p>{msg}</p>
</div>
