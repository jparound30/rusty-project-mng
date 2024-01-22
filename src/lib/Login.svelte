<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"

    let username = ""
    let password = ""
    let msg = ""

    async function authenticate() {
        invoke<{ user_id: string, username: string }>("authenticate", {
            username,
            password
        }).then(value => msg = "Login Successful:[" + value.username + "]").catch(reason => msg = reason)

    }
</script>

<div>
  <p>Login</p>
  <form class="row" on:submit|preventDefault="{authenticate}">
    <input id="username-input" placeholder="Username" bind:value={username}/>
    <input id="password-input" placeholder="Password" type="password" bind:value={password}/>
    <button type="submit">Login</button>
  </form>
  <p>{msg}</p>
</div>