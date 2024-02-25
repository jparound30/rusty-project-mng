<script lang="ts">
    import {invoke} from "@tauri-apps/api/core"
    import {TaskFull} from "components/TaskFull";
    import type {Resource} from "components/Resource";
    import {createEventDispatcher, onMount} from "svelte";

    export let taskId: number | undefined;
    export let showModal: boolean = false;

    let dialog: HTMLDialogElement;
    const dispatch = createEventDispatcher();

    $: {
        if (dialog && showModal) {
            if (taskId !== undefined) {
                getTask(taskId).then(() => dialog.showModal())
            }
        }
    }

    let targetTask: TaskFull = new TaskFull();

    let msg = ""
    // $: if (assigneeResourceId != null && estimatedTime != null) {
    //     const resource = data.resources_list.find(v => v.resource_id == assigneeResourceId)
    //     if (resource !== undefined) {
    //         plannedValue = estimatedTime * resource.cost_per_month / 160
    //     }
    // }

    let task_status_list: { task_status_id: number, title: string, view_order: number }[] = []
    let resources_list: Resource[] = []
    let task_simple_list: { task_id: number, title: string }[] = []

    async function getInfo() {
        task_status_list =
            await invoke<{ task_status_id: number, title: string, view_order: number }[]>("task_status_list", {})
                .then(value => {
                    console.log("ステータス一覧取得成功")
                    return value
                }).catch(reason => {
                    console.error("ステータス一覧取得失敗", reason)
                    return [] as { task_status_id: number, title: string, view_order: number }[];
                })

        resources_list =
            await invoke<Resource[]>("resources_list", {})
                .then(value => {
                    console.log("リソース一覧取得成功")
                    return value
                }).catch(reason => {
                    console.error("リソース一覧取得失敗", reason)
                    return [] as { resource_id: number, name: string, cost_per_month: number }[];
                })

        task_simple_list =
            await invoke<{ task_id: number, title: string }[]>("task_simple_all", {})
                .then(value => {
                    console.log("タスク（idとタイトルのみ）一覧取得成功")
                    return value
                }).catch(reason => {
                    console.error("タスク（idとタイトルのみ）一覧取得失敗", reason)
                    return [] as { task_id: number, title: string }[];
                })
    }

    async function getTask(taskId: number) {
        await invoke<TaskFull>("task_get", {
            taskId
        }).then(value => {
            console.log("更新成功")
            targetTask = value
        }).catch(reason => {
            console.error("更新失敗", reason)
            msg = reason
            // TODO 対象タスクが見つからないエラーダイアログとか出す
        })
    }

    async function updateTask() {
        await invoke<{ msg: String }>("task_update", {
            taskId: taskId,
            title: targetTask.title,
            description: targetTask.description,
            assigneeResourceId: targetTask.assignee_resource_id,
            parentTaskId: targetTask.parent_task_id,
            startDate: targetTask.start_date,
            dueDate: targetTask.due_date,
            estimatedTime: targetTask.estimated_time,
            actualTime: targetTask.actual_time,
            plannedValue: targetTask.planned_value,
            taskStatusId: targetTask.task_status_id,
            progressRate: targetTask.progress_rate,
        }).then(value => {
            console.log("task_update成功")
            dispatch('task-updated', taskId)
            dialog.close()
        }).catch(reason => {
            console.error("task_update失敗", reason)
            msg = reason
        })
    }

    let getInfoPromise: Promise<void>;
    onMount(async () => {
        getInfoPromise = getInfo();
    });

</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
    bind:this={dialog}
    on:close={() => (showModal = false)}
    class="bg-green-300 max-w-5xl"
>
  <div class="p-4 ">
    <h1 class="text-2xl underline underline-offset-4">タスク編集</h1>
    <form class="row" on:submit|preventDefault="{updateTask}">
      <div class="flex flex-row py-2">
        <label for="title" class="form-label basis-48">タスク名</label>
        <input type="text" id="title" name="title" class="basis-full" required bind:value={targetTask.title}>
      </div>
      <div class="flex flex-row py-2">
        <label for="description" class="form-label basis-48">詳細</label>
        <textarea id="description" name="description" rows="4" cols="50" class="basis-full"
                  bind:value={targetTask.description}></textarea>
      </div>
      <div class="flex flex-row py-2">
        <label for="assignee_resource_id" class="form-label basis-48">担当者</label>
        <select id="assignee_resource_id" name="assignee_resource_id" class="basis-full"
                bind:value={targetTask.assignee_resource_id}>
          <option value="{null}"></option>
          {#await getInfoPromise then value}
            {#each resources_list as item (item.resource_id)}
              <option value="{item.resource_id}">{item.name}</option>
            {/each}
          {/await}
        </select>
      </div>
      <div class="flex flex-row py-2">
        <label for="parent_task_id" class="form-label basis-48">親タスク</label>
        <select id="parent_task_id" name="parent_task_id" class="basis-full" bind:value={targetTask.parent_task_id}>
          <option value="{null}"></option>
          {#await getInfoPromise then value}
            {#each task_simple_list as item (item.task_id)}
              <option value="{item.task_id}">{item.title}</option>
            {/each}
          {/await}
        </select>
      </div>
      <div class="flex flex-row py-2">
        <label for="start_date" class="form-label basis-48">開始日</label>
        <input type="date" id="start_date" name="start_date" class="basis-full" bind:value={targetTask.start_date}>
      </div>
      <div class="flex flex-row py-2">
        <label for="due_date" class="form-label basis-48">期限日</label>
        <input type="date" id="due_date" name="due_date" class="basis-full" bind:value={targetTask.due_date}>
      </div>
      <div class="flex flex-row py-2">
        <label for="estimated_time" class="form-label basis-48">予定工数[H]</label>
        <input type="number" id="estimated_time" name="estimated_time" class="basis-full"
               bind:value={targetTask.estimated_time}>
      </div>
      <div class="flex flex-row py-2">
        <label for="actual_time" class="form-label basis-48">実工数[H]</label>
        <input type="number" id="actual_time" name="actual_time" class="basis-full"
               bind:value={targetTask.actual_time}>
      </div>
      <div class="flex flex-row py-2">
        <label for="planed_value" class="form-label basis-48">計画予算[\]</label>
        <input type="number" id="planed_value" name="planed_value" class="basis-full"
               bind:value={targetTask.planned_value}>
      </div>
      <div class="flex flex-row py-2">
        <label for="task_status_id" class="form-label basis-48">状態</label>
        <select id="task_status_id" name="task_status_id" class="basis-full" required
                bind:value={targetTask.task_status_id}>
          {#await getInfoPromise then value}
            {#each task_status_list as item (item.task_status_id)}
              <option value="{item.task_status_id}">{item.title}</option>
            {/each}
          {/await}
        </select>
      </div>
      <div class="flex flex-row py-2">
        <label for="progress_rate" class="form-label basis-48">作業進捗度合[%]</label>
        <select id="progress_rate" name="progress_rate" class="basis-full" required
                bind:value={targetTask.progress_rate}>
          {#await getInfoPromise then value}
            {#each Array.from(Array(11), (v, x) => x * 10) as val}
              <option value="{val}">{val}</option>
            {/each}
          {/await}
        </select>
      </div>
      <button type="submit" value="Submit" class="btn-primary">更新</button>
      <button value="Submit" class="btn-primary" on:click={() => dialog.close()}>キャンセル</button>
    </form>
    <p>{msg}</p>
  </div>
</dialog>

<style>
    dialog {
        max-width: 64em;
        width: 128em;
        border-radius: 0.2em;
        border: none;
        padding: 0;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.3);
    }

    dialog > div {
        padding: 1em;
    }

    dialog[open] {
        animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    @keyframes zoom {
        from {
            transform: scale(0.95);
        }
        to {
            transform: scale(1);
        }
    }

    dialog[open]::backdrop {
        animation: fade 0.2s ease-out;
    }

    @keyframes fade {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }
</style>