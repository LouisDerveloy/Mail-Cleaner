<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {useRouter} from "vue-router";
import { handleError } from "../lib/error";
import ThirdPartyConnexionButton from "../Components/ThirdPartyConnexionButton.vue";
import GmailIco from "../assets/images/gmail.svg";
import ExternalLink from "../Components/ExternalLink.vue";

const router = useRouter();

const connectionType = ref('password'); // 'password' or 'token'

const thirdPartyMessage = ref("Initialization");
const thirdPartyLink = ref("")

const server = ref("");
const port = ref(993);
const token = ref("");
const email = ref("");
const password = ref("");

function TokenConnect() {
  invoke("token_connect", {
    server: server.value,
    port: port.value,
    email: email.value,
    token: token.value
  }).then(value => { 
    console.log("TokenConnect Result : ", value) 
    router.push("/")
  }).catch(handleError)
}

function PasswordConnect() {
  invoke("password_connect", {
    server: server.value,
    port: port.value,
    email: email.value,
    password: password.value
  }).then(value => { 
    console.log("PasswordConnect Result : ", value)
    router.push("/")
  }).catch(handleError)
}

function GoogleConnect() {
  connectionType.value = "thirdparty";

  // invoke("gmail_oauth_request").then(value => {
  //   console.log("Connect with google Resulgt: ", value);
  //   router.push("/")
  // })
}

</script>

<template>
  <section class="connection-view">
    <Transition appear><h1>Choose your connection type</h1></Transition>
    <div class="connection-type-selector">
      <button @click="connectionType = 'password'" :class="{ active: connectionType === 'password' }">Login/Password</button>
      <button @click="connectionType = 'token'" :class="{ active: connectionType === 'token' }">Token Based</button>
      <ThirdPartyConnexionButton :ico="GmailIco" label="Gmail" @click="GoogleConnect" :class="{active: connectionType === 'thirdparty'}"/>
    </div>

    <h1 v-if="connectionType === 'password' || connectionType==='token'">Connection Form</h1>

    <div v-if="connectionType === 'password'" class="connection-form">
        <div class="field">
          <label for="server-pass">Server: </label>
          <input type="text" id="server-pass" v-model="server" placeholder="imap.gmail.com">
        </div>
        <div class="field">
          <label for="port-pass">Port: </label>
          <input type="number" max="65535" id="port-pass" v-model="port">
        </div>
        <div class="field">
          <label for="email-pass">Email: </label>
          <input type="text" id="email-pass" v-model="email" placeholder="your@email.com">
        </div>
        <div class="field">
          <label for="password">Password: </label>
          <input type="password" id="password" v-model="password" placeholder="pass***d">
        </div>
        <button @click="PasswordConnect">Connect</button>
  </div>

    <div v-else-if="connectionType === 'token'" class="connection-form">
    <div class="field">
      <label for="server-token">Server: </label>
      <input type="text" id="server-token" v-model="server" placeholder="imap.gmail.com">
    </div>
    <div class="field">
      <label for="port-token">Port: </label>
      <input type="number" max="65535" id="port-token" v-model="port">
    </div>
    <div class="field">
      <label for="email-token">Email: </label>
      <input type="text" id="email-token" v-model="email" placeholder="your@email.com">
    </div>
    <div class="field">
      <label for="token">Token: </label>
      <input type="password" id="token" v-model="token" placeholder="token***">
    </div>

    <button @click="TokenConnect">Connect</button>
  </div>

    <div v-else-if="connectionType === 'thirdparty'" class="thirdPartyConnexion">
      <h1>STATUS : {{thirdPartyMessage}}</h1>

      <section>
        <h2 v-if="thirdPartyLink === ''">You will shortly be redirected to an authentication page.</h2>
        <h2 v-else>Please open this link to continue : <ExternalLink :href="thirdPartyLink">{{thirdPartyLink}}</ExternalLink></h2>
      </section>

    </div>
  </section>
</template>

<style scoped>
.connection-view {
  max-width: 800px;
  width: 100%;
  margin: 1rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: start;
  gap: 1.3rem;
  height: 100%;
  padding: 2rem;
}

.connection-view h1 {
  width: 100%;
  text-align: start;
  font-size: 2rem;
  font-weight: 100;
  text-transform: uppercase;
}

.connection-type-selector {
  display: flex;
  justify-content: center;
  height: 5rem;
  background: #f7f7f7;
  border-radius: 8px;
  box-shadow: 0px 0px 20px 0 rgba(0, 0, 0, 0.1);
  width: 100%;
}

.connection-type-selector > * {
  padding: 1rem;
  margin: 0;  
  background: #f7f7f7;
  width: 50%;
  height: 100%;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.2s, color 0.2s;
}

.connection-type-selector > *:focus {
  outline: none;
}

.connection-type-selector button:first-child {
  border-top-left-radius: 6px;
  border-bottom-left-radius: 6px;
  border-right: none;
}

.connection-type-selector button:last-child {
  border-top-right-radius: 6px;
  border-bottom-right-radius: 6px;
}

.connection-type-selector button.active {
  background: #007bff;
  color: white;
  border-color: #007bff;
}

.connection-form {
  padding: 2rem;
  background: #f7f7f7;
  border-radius: 8px;
  box-shadow: 0px 0px 20px 0 hsla(0, 0%, 0%, 0.1);
  border: none;
  width: 100%;
}

.connection-form .field label {
  display: block;
  margin-bottom: .3rem;
  color: #333;
}

.connection-form .field input {
  width: 100%;
  padding: .75rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box; /* important */
  margin-bottom: 1rem;
}

.connection-form button {
  width: 100%;
  padding: .75rem;
  border: none;
  border-radius: 4px;
  background: #28a745;
  color: white;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.connection-form button:hover {
  background: #218838;
}

.thirdPartyConnexion {
  width: 100%;
  margin-top: 2rem;
  --font-size: 2rem;
}

.thirdPartyConnexion h1 {
  text-transform: none;
  font-size: var(--font-size);
}

.thirdPartyConnexion h2 {
  font-size: calc(var(--font-size) * 0.75);
  margin-top: 1rem;
}
</style>