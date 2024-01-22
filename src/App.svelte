<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import Login from './lib/Login.svelte'
</script>

<main class="container">
  <div class="row">
    <Greet />
  </div>
  <div class="row">
    <Login/>
  </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>