<script lang="ts">
export let url: string
export let name: string
</script>

<button class="bg-green-700 hover:bg-green-600 text-white rounded px-4 py-1">
  <a href={url}>{name}</a>
</button>
