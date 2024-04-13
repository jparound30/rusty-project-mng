<script lang="ts">
  export let url: string
  export let name: string
</script>

<a class="bg-green-700 hover:bg-green-600 cursor-pointer text-white rounded px-4 py-1"
   href={url}>{name}</a>
