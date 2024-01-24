<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import {goto} from "$app/navigation";

    let username = ""
    let password = ""
    let msg = ""

    async function authenticate() {
        let authenticated = false;
        await invoke<{ user_id: string, username: string }>("authenticate", {
            username,
            password
        }).then(value => {
            console.log("認証成功")
            authenticated = true
            msg = "Login Successful:[" + value.username + "]"
            goto('/home');
        }).catch(reason => {
            console.log("認証失敗")
            console.error(reason)
            msg = reason;
        })
    }
</script>

<svelte:head>
  <title>ログイン</title>
</svelte:head>

<div>
  <p>ログイン</p>
  <form class="row" on:submit|preventDefault="{authenticate}">
    <input id="username-input" placeholder="Username" bind:value={username}/>
    <input id="password-input" placeholder="Password" type="password" bind:value={password}/>
    <button type="submit">ログイン</button>
  </form>
  <p>{msg}</p>
</div>