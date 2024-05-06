<script lang="ts">
  import type {PageData} from './$types'
  import type {Resource} from "components/Resource";
  import {onDestroy, onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/core";
  import ResourceEdit from "$lib/ResourceEdit.svelte";

  /** @type {import('../../../../../.svelte-kit/types/src/routes').PageData} */
  export let data: PageData

  let menuVisible = false;
  let menuPosition = {x: 0, y: 0};
  let menuElement: HTMLElement;

  let isCreate = true;
  let editOpen = false;
  let selectResource: Resource | undefined = undefined;
  let selectResourceId: number | undefined = undefined;

  let showDeleteDialog = false;

  function showMenu(event: MouseEvent, resourceId: number) {
    // デフォルトの右クリックメニューを表示させない
    event.preventDefault();

    // メニューを表示する位置を設定
    menuPosition = {x: event.pageX, y: event.pageY};

    // メニューを表示
    menuVisible = true;

    selectResource = data.resource_list.find(v => v.resource_id == resourceId);
    selectResourceId = resourceId;
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
    isCreate = false
    editOpen = true
  }

  function openCreateDialog(event: MouseEvent) {
    isCreate = true
    selectResourceId = undefined
    selectResource = undefined
    editOpen = true
    console.log("openCreateDialog called")
  }

  onMount(() => {
    window.addEventListener("click", handleOutsideClick);
  });

  onDestroy(() => {
    window.removeEventListener("click", handleOutsideClick);
  });


  async function updateResourceList() {
    console.log("updateResourceList")
    let resource_list_p =
      invoke<Resource[]>("resource_list", {})
        .then(value => {
          console.log("リソース一覧取得成功")
          return value
        }).catch(reason => {
        console.error("リソース一覧取得失敗", reason)
        return [] as Resource[];
      })
    data.resource_list = await resource_list_p;
  }

  function openDeleteDialog() {
    showDeleteDialog = true;
  }
  function closeDeleteDialog() {
    showDeleteDialog = false;
  }
</script>

<div class="p-4">
  <h1 class="text-2xl underline underline-offset-4">リソース一覧</h1>

  <ResourceEdit bind:showModal={editOpen} bind:resourceId="{selectResourceId}" bind:isCreate={isCreate} on:resource-updated={updateResourceList}/>

  <button class="btn-primary rounded-3xl font-bold my-2" on:click={openCreateDialog}>+</button>

  <div class="table-auto">
    <table>
      <thead class="border-2">
      <tr class="bg-blue-300 border-blue-600">
        <th class="border-2 px-1 py-1 text-start w-40">名前</th>
        <th class="border-2 px-1 py-1 text-start w-36">コスト/月 [&yen;]</th>
      </tr>
      </thead>
      <tbody>
      {#each data.resource_list as item (item.resource_id)}
        <tr class="" on:contextmenu={(event) => {showMenu(event, item.resource_id)}}>
          <td class="border-2 py-1 text-start"><span class="p-2">{item.name}</span></td>
          <td class="border-2 py-1 text-end">{item.cost_per_month?.toLocaleString() ?? ''}</td>
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
<!--    <button class="m-2" on:click={openDeleteDialog}>削除する</button>-->
  </div>
{/if}

<!--<Modal bind:showModal={showDeleteDialog}>-->
<!--  <div slot="header">-->
<!--    タスク名:[{selectTask?.title}] を削除します-->
<!--  </div>-->
<!--  <div slot="footer" class="flex flex-row">-->
<!--    <button class="bg-red-700 hover:bg-red-600 text-white rounded m-2 px-4 py-1" on:click={() => {if (selectTaskId !== undefined) deleteTask(selectTaskId)}}>削除</button>-->
<!--    <button class="bg-gray-400 hover:bg-gray-300 text-white rounded m-2 px-4 py-1" on:click={closeDeleteDialog}>キャンセル</button>-->
<!--  </div>-->
<!--</Modal>-->
