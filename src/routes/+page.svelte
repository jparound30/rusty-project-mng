<script>
    import { appWindow, LogicalSize } from '@tauri-apps/api/window';
    import {onMount} from "svelte";

    onMount(() => {
        console.log("onMount called")
        appWindow.setSize(new LogicalSize(1200, 500));
    })
</script>

<div>
  ルート画面
  表示されることはない予定
</div>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.green.100);
  }
</style>
