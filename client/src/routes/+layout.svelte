<script>
  import Header from '$lib/components/Header.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
</script>

<div class="app">
  <Sidebar />
  <div class="main">
    <Header />
    <slot />
  </div>
</div>

<style>
  .app {
    display: flex;
    height: 100vh;
    font-family: system-ui;
  }

  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
</style>
