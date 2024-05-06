<script lang="ts">
  import LinkButton from "$lib/LinkButton.svelte";
  import {onMount} from "svelte";

  onMount(() => {
      console.log("onMount called")
  })

</script>

<nav class="flex items-center text-l line-clamp-1 px-2 py-2">
  <ul>
    <li class="inline-block px-1"><LinkButton name={"ホーム"} url={"/home"} /></li>
    <li class="inline-block px-1"><LinkButton name={"進捗状況"} url={"/evm"} /></li>
    <li class="inline-block px-1"><LinkButton name={"タスク一覧"} url={"/tasks"} /></li>
    <li class="inline-block px-1"><LinkButton name={"リソース一覧"} url={"/configs/resources"} /></li>
    <li class="inline-block px-1"><LinkButton name={"タスク種別"} url={"/configs/task-types"} /></li>
    <li class="inline-block px-1"><LinkButton name={"マイルストーン一覧"} url={"/configs/milestones"} /></li>
  </ul>
</nav>

<slot></slot>