<script>
  import { slide } from 'svelte/transition';

  import { clickOutside } from '$lib/utils';
  import { currentState } from '$lib/shared.svelte.js';

  import icon from '$lib/assets/favicon.ico';
  import { goto } from '$app/navigation';

  // Enum
  const Spaces = {
    HOME: "Home",
    PORTFOLIO: "Portfolio",
    SETTINGS: "Settings",
  }

  let currentSpace = $state(Spaces.HOME);
</script>

<nav class="sidebar" transition:slide={{axis: 'x', duration: 300}} use:clickOutside={() => currentState.sidebarOpen = false}>
  <div class="box title">
    <img src={icon} alt="Dashy" class="logo" />
    <h2>Dashy</h2>
  </div>
  <button onclick={() => {goto("/"); currentSpace = Spaces.HOME}} class:selected={currentSpace === Spaces.HOME}>
    <div class="box element">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 logo">
        <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
      </svg>
      <p>Home</p>
    </div>
  </button>
  <button onclick={() => {goto("/portfolio"); currentSpace = Spaces.PORTFOLIO}} class:selected={currentSpace === Spaces.PORTFOLIO}>
    <div class="box element">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 logo">
        <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z" />
        <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z" />
      </svg>

      <p>Portfolio</p>
    </div>
  </button>
  <button onclick={() => {goto("/settings"); currentSpace = Spaces.SETTINGS}} class:selected={currentSpace === Spaces.SETTINGS}>
    <div class="box element">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 logo">
        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12a7.5 7.5 0 0 0 15 0m-15 0a7.5 7.5 0 1 1 15 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077 1.41-.513m14.095-5.13 1.41-.513M5.106 17.785l1.15-.964m11.49-9.642 1.149-.964M7.501 19.795l.75-1.3m7.5-12.99.75-1.3m-6.063 16.658.26-1.477m2.605-14.772.26-1.477m0 17.726-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205 12 12m6.894 5.785-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495" />
      </svg>

      <p>Settings</p>
    </div>
  </button>
</nav>

<style>
  .sidebar {
    width: 220px;
    background: #1d232a;
    color: white;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    position: absolute;
    inset: 0 auto 0 0;
  }

  button {
    color: #aaa;
    text-decoration: none;
    background: none;
    border: 1px solid #333;
    border-radius: 10px;
    padding-top: 5px;
    padding-bottom: 5px;
  }

  button:hover {
    background-color: rgba(0, 0, 0, 0.2);
  }

  .box {
    display: flex;
    align-items: center;
    gap: 10px;
    
    /* border-bottom: 1px solid #000000; */
  }

  .title {
    margin-bottom: 20px;
  }

  .logo {
    width: 30px;
    height: 30px;
  }
  
  .element {
    padding-left: 10px;
  }

  .selected {
    background-color: rgba(9, 33, 103, 0.2);
    color: white;
  }

</style>
