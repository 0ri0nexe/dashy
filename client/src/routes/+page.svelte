<script>
  import { onMount } from "svelte";

  let message = "Loading...";

  onMount(async () => {
    try {
      const res = await fetch("/api/hello");
      const data = await res.json();
      message = data.message;
    } catch (err) {
      message = "Error connecting to backend";
      console.error(err);
    }
  });
</script>

<h1>{message}</h1>

<style>
  h1 {
    font-family: system-ui;
    text-align: center;
    margin-top: 40vh;
  }
</style>
