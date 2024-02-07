<script>
    import {onMount} from "svelte";

    onMount(() => {
        console.log("onMount called")
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
