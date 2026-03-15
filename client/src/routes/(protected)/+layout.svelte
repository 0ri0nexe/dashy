<script>
    import Header from "$lib/components/Header.svelte";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import { currentState } from '$lib/shared.svelte.js';

    import { clickOutside } from '$lib/utils.js';

</script>
<div class="app">
    <aside use:clickOutside={() => currentState.sidebarOpen = false}>
      {#if currentState.sidebarOpen}
        <Sidebar />
      {/if}
    </aside>
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