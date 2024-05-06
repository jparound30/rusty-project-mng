<script lang="ts">
  import {invoke} from "@tauri-apps/api/core"
  import {Resource} from "components/Resource";
  import {createEventDispatcher, onMount} from "svelte";

  export let resourceId: number | undefined;
  export let showModal: boolean = false;

  export let isCreate: boolean = false;

  let dialog: HTMLDialogElement;
  const dispatch = createEventDispatcher();

  let name = ""
  let costPerMonth = 0.0

  $: {
    if (dialog && showModal) {
      if (resourceId !== undefined) {
        getResource(resourceId).then(() => dialog.showModal())
      } else if (isCreate) {
        dialog.showModal()
      }
    }
  }


  let msg = ""

  async function getResource(resourceId: number) {
    await invoke<Resource>("resource_get", {
      resourceId: resourceId
    }).then(value => {
      console.log("resource_get成功")
      costPerMonth = value.cost_per_month
      name = value.name
    }).catch(reason => {
      console.error("resource_get失敗", reason)
      msg = reason
      // TODO 対象タスクが見つからないエラーダイアログとか出す
    })
  }

  async function updateResource() {
    if (isCreate) {
      await invoke<{ msg: String }>("resource_add", {
        name: name,
        costPerMonth: costPerMonth,
      }).then(value => {
        console.log("resource_add成功")
        dispatch('resource-updated', resourceId)
        dialog.close()
      }).catch(reason => {
        console.error("resource_add失敗", reason)
        msg = reason
      })
    } else {
      await invoke<{ msg: String }>("resource_update", {
        resourceId: resourceId,
        name: name,
        costPerMonth: costPerMonth,
      }).then(value => {
        console.log("resource_update成功")
        dispatch('resource-updated', resourceId)
        dialog.close()
      }).catch(reason => {
        console.error("resource_update失敗", reason)
        msg = reason
      })
    }
    reset()
  }

  async function cancel() {
    reset()
    dialog.close()
  }

  function reset() {
    name = ""
    costPerMonth = 0
  }

  onMount(async () => {
  });

</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
    bind:this={dialog}
    on:close={() => (showModal = false)}
    class="bg-green-300 max-w-5xl"
>
  <div class="p-4 ">
    <h1 class="text-2xl underline underline-offset-4">リソース
      {#if isCreate}新規追加{:else}編集{/if}
    </h1>
    <form class="row" on:submit|preventDefault="{updateResource}">
      <div class="flex flex-row py-2">
        <label for="name" class="form-label basis-48">名前</label>
        <input type="text" id="name" name="name" class="basis-full" required bind:value={name}>
      </div>
      <div class="flex flex-row py-2">
        <label for="actual_time" class="form-label basis-48">コスト/月[&yen;]</label>
        <input type="number" id="cost_per_month" name="cost_per_month" class="basis-full"
               bind:value={costPerMonth}>
      </div>
      <button type="submit" value="Submit" class="btn-primary">
        {#if isCreate}追加{:else}更新{/if}
      </button>
      <button type="button" value="cancel" class="btn-primary" on:click={cancel}>キャンセル</button>
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