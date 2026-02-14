<script>
  import { onMount } from 'svelte';

  let btc = null;
  let eth = null;

  async function fetchPrice(symbol) {
    const res = await fetch(`/api/price/${symbol}`);
    return await res.json();
  }

  onMount(async () => {
    btc = await fetchPrice("BTC");
    // eth = await fetchPrice("ETH");
  });
</script>

<div class="cards">
  {#if btc}
    <div class="card">
      <h2>{btc.symbol}</h2>
      <p>${btc.price}</p>
    </div>
  {/if}

  <!-- {#if eth}
    <div class="card">
      <h2>{eth.symbol}</h2>
      <p>${eth.price_usd}</p>
    </div>
  {/if} -->
</div>

<style>
  .cards {
    display: flex;
    gap: 20px;
    padding: 20px;
  }

  .card {
    background: white;
    padding: 20px;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
    min-width: 150px;
  }

  h2 {
    margin: 0;
  }

  p {
    font-size: 1.5rem;
    margin: 8px 0 0 0;
  }
</style>
