<script lang="js">
	import { goto } from '$app/navigation';

	let email = "";
	let password = "";
	let error = "";
	let loading = false;

	async function login() {
		error = "";
		loading = true;

		try {
			const res = await fetch("/api/auth/login", {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify({
					email,
					password
				})
			});

			if (!res.ok) {
				error = "Email ou mot de passe incorrect";
				return;
			}

			// le cookie session_id est automatiquement stocké
			await goto("/");

		} catch (e) {
			error = "Erreur réseau";
		} finally {
			loading = false;
		}
	}
</script>

<div class="container">

	<h1>Connexion</h1>

	<form on:submit|preventDefault={login}>

		<input
			type="email"
			placeholder="Email"
			bind:value={email}
			required
		/>

		<input
			type="password"
			placeholder="Mot de passe"
			bind:value={password}
			required
		/>

		<button disabled={loading}>
			{loading ? "Connexion..." : "Se connecter"}
		</button>
	</form>

    <button disabled={loading} on:click={() => goto("/register")} >
        {loading ? "Connexion..." : "Créer un compte"}
    </button>

	{#if error}
		<p class="error">{error}</p>
	{/if}

</div>

<style>

.container {
	max-width: 400px;
	margin: 100px auto;
	display: flex;
	flex-direction: column;
	gap: 20px;
}

form {
	display: flex;
	flex-direction: column;
	gap: 10px;
}

input {
	padding: 10px;
	font-size: 16px;
}

.error {
	color: red;
}

</style>