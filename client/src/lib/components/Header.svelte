<script>
  import { currentState } from '$lib/shared.svelte.js';
  $inspect(currentState);

  function toggleSidebar() {
    currentState.sidebarOpen = !currentState.sidebarOpen;
  }
</script>

<header>
  <!-- Sidebar button -->
  <button aria-label="toggle-sidebar" onclick={toggleSidebar}>
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 menu-icon">
      <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
    </svg>
  </button>

  <h3>Dashy</h3>
  <div class="user">👤 You</div>
</header>

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 5px;
    border-bottom: 1px solid #020209;

    background-color: #20262e;
  }

  .menu-icon {
    width: 27px;
    height: 27px;
    cursor: pointer;
    color: #ecf9ff;
  }
  
  .user {
    padding-right: 20px;
  }
  
  button {
    background-color: rgba(0, 0, 0, 0);
    border-radius: 10%;
    border: none;
    margin-left: 10px;
    padding: 5px;
    padding-bottom: 4px;
  }

  button:hover {
    background-color: rgba(0, 0, 0, 0.2);
  }
</style>
