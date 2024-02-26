<script lang="ts">
    import type {PageData} from './$types'
    import type {TaskFull} from "$components/TaskFull";
    import type {Resource} from "components/Resource";
    import {onDestroy, onMount} from "svelte";
    import TaskEdit from "$lib/TaskEdit.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import Modal from "$lib/Modal.svelte";
    // import PlannedValueChangesChart from "$lib/PlannedValueChangesChart.svelte";
    // import EvmTable from "$lib/EvmTable.svelte";
    // import EvmChart from "$lib/EvmChart.svelte";
    // import EvmIndex from "$lib/EvmIndex.svelte";

    /** @type {import('./$types').PageData} */
    export let data: PageData

    let menuVisible = false;
    let menuPosition = {x: 0, y: 0};
    let menuElement: HTMLElement;

    let editOpen = false;
    let selectTask: TaskFull | undefined = undefined;
    let selectTaskId: number | undefined = undefined;

    let showDeleteDialog = false;

    function calculateActualCost(task: TaskFull): number | null {
        let actualCost: number | null = null;
        if (task.actual_time != null) {

            const resource: Resource | undefined = data.resources_list.find(s => s.resource_id == task.assignee_resource_id);
            if (resource !== undefined) {
                actualCost = task.actual_time * resource.cost_per_month / 160
            }
        }
        return actualCost;
    }

    function showMenu(event: MouseEvent, taskId: number) {
        // デフォルトの右クリックメニューを表示させない
        event.preventDefault();

        // メニューを表示する位置を設定
        menuPosition = {x: event.pageX, y: event.pageY};

        // メニューを表示
        menuVisible = true;

        selectTask = data.task_list.find(v => v.task_id == taskId);
        selectTaskId = taskId;
    }

    function hideMenu() {
        // メニューを非表示
        menuVisible = false;
    }

    function handleOutsideClick(event: MouseEvent) {
        if (menuElement && !menuElement.contains(event.target as Node)) {
            hideMenu();
        }
    }

    function openEditDialog(event: MouseEvent) {
        editOpen = true
    }

    onMount(() => {
        window.addEventListener("click", handleOutsideClick);
    });

    onDestroy(() => {
        window.removeEventListener("click", handleOutsideClick);
    });


    async function updateTaskList() {
        console.log("updateTaskList")
        let task_list_p =
            invoke<TaskFull[]>("task_all_full", {})
                .then(value => {
                    console.log("タスク（idとタイトルのみ）一覧取得成功")
                    return value
                }).catch(reason => {
                console.error("タスク（idとタイトルのみ）一覧取得失敗", reason)
                return [] as TaskFull[];
            })
        data.task_list = await task_list_p;
    }

    async function deleteTask(taskId: number) {
        await invoke<TaskFull[]>("task_delete", {taskId})
            .then(value => {
                console.log("タスク削除成功")
            }).catch(reason => {
                console.error("タスク削除失敗", reason)
            })
        await updateTaskList();
        closeDeleteDialog();
    }

    function openDeleteDialog() {
        showDeleteDialog = true;
    }
    function closeDeleteDialog() {
        showDeleteDialog = false;
    }
</script>

<div class="p-4">
  <h1 class="text-2xl underline underline-offset-4">タスク一覧</h1>

  <TaskEdit bind:showModal={editOpen} bind:taskId="{selectTaskId}" on:task-updated={updateTaskList}/>
  <!-- TODO いったんコメントアウトしとく。どれかは見れた方が便利そう -->
  <!--  <EvmIndex evmInfo="{data.evm_info}" budgetAtCompletion="{data.planned_value_changes[(data.planned_value_changes.length) - 1].planned_value}"/>-->
  <!--  <EvmChart evm_histories="{data.evm_histories}" planned_value_changes="{data.planned_value_changes}" />-->
  <!--  <PlannedValueChangesChart data={data.planned_value_changes} />-->

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
        <tr class="" on:contextmenu={(event) => {showMenu(event, item.task_id)}}>
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

<!-- アクションメニュー -->
{#if menuVisible}
  <div bind:this={menuElement}
       style="position: fixed; top: {menuPosition.y}px; left: {menuPosition.x}px; background: #fff; border: 1px solid #ccc; padding: 10px;"
       on:click={hideMenu}
       on:keyup={hideMenu}
       role="menu"
       tabindex="-1"
       class="flex flex-col"
  >
    <button class="m-2" on:click={openEditDialog}>編集する</button>
    <button class="m-2" on:click={openDeleteDialog}>削除する</button>
  </div>
{/if}

<Modal bind:showModal={showDeleteDialog}>
  <div slot="header">
    タスク名:[{selectTask?.title}] を削除します
  </div>
  <div slot="footer" class="flex flex-row">
    <button class="bg-red-700 hover:bg-red-600 text-white rounded m-2 px-4 py-1" on:click={() => {if (selectTaskId !== undefined) deleteTask(selectTaskId)}}>削除</button>
    <button class="bg-gray-400 hover:bg-gray-300 text-white rounded m-2 px-4 py-1" on:click={closeDeleteDialog}>キャンセル</button>
  </div>
</Modal>
