<script lang="js">
	import { goto } from '$app/navigation';

	let email = "";
	let password = "";
	let confirmPassword = "";
	let error = "";
	let loading = false;

	async function register() {
        console.log("Registering...");
        fetch("/api/test", { method: "POST" });
		error = "";
		if (password !== confirmPassword) {
			error = "Les mots de passe ne correspondent pas";
			return;
		}

		loading = true;

		try {
            console.log("Sending request with :\n - email : " + email + "\n - password : " + password);

			const res = await fetch("/api/auth/register", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ email, password })
			});

            console.log(res);
            
			if (!res.ok) {
				const text = await res.text();
				error = text || "Erreur lors de l'inscription";
				return;
			}

			// rediriger vers login après inscription
			await goto("/login");

		} catch (e) {
			error = "Erreur réseau";
		} finally {
			loading = false;
		}
	}


</script>

<div class="container">

	<h1>Créer un compte</h1>

	<form on:submit|preventDefault={register}>

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

		<input
			type="password"
			placeholder="Confirmer le mot de passe"
			bind:value={confirmPassword}
			required
		/>

		<button type="submit" disabled={loading}>
			{loading ? "Inscription..." : "S'inscrire"}
		</button>

	</form>

	{#if error}
		<p class="error">{error}</p>
	{/if}

	<p>
		Déjà un compte ?
		<a href="/login">Se connecter</a>
	</p>

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