<!--
  - Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
  - Licensed under the PolyForm Noncommercial License 1.0.0
  - SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
  - See the LICENSE file in the project root for details.
  -->

<script lang="ts" setup>
import {onMounted, onBeforeUnmount, ref} from "vue";
import {Channel, invoke} from "@tauri-apps/api/core";
import {useRouter} from "vue-router";
import {handleError} from "../lib/error";
import ExternalLink from "../Components/ExternalLink.vue";
import TabItem from "../Components/Tab/TabItem.vue";
import TabContainer from "../Components/Tab/TabContainer.vue";
import Button from "../Components/Button.vue";
import Input from "../Components/Input.vue";
import gsap from "gsap";

import GmailIco from "../assets/images/gmail.svg";
import LeftArrow from "../assets/images/arrow-left.svg";

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

// Interface to describe communication between backend and frontend
interface UserFeedback {
  update_type: string;
  content: string;
}

function GoogleConnect() {
  connectionType.value = "thirdparty";

  const channel = new Channel();

  channel.onmessage = (response: unknown) => {
    let update_type = (response as UserFeedback).update_type;

    if (update_type === "Status") {
      thirdPartyMessage.value = (response as UserFeedback).content;
    } else if (update_type === "Link") {
      thirdPartyLink.value = (response as UserFeedback).content;
    }

    console.log(response);
  }

  invoke("gmail_oauth_request", {feedbackChannel: channel}).then(value => {
    console.log("Connect with google Resulgt: ", value);
    router.push("/")
  })
}

let ctx: gsap.Context | null = null;
onMounted(() => ctx = gsap.context(() => {
  gsap.fromTo(".fade-in", {
    y: -50,
    opacity: 0,
  }, {
    y: 0,
    opacity: 1,
    duration: 0.3,
    stagger: 0.05,
    ease: "power1.out",
  })
}));
onBeforeUnmount(() => ctx?.revert())

</script>

<template>
  <section class="connection-view">
    <Button size="small" :icon="LeftArrow" icon-alt="Left Arrow icon" @click="router.push('/')">Go back</Button>
    <section>
      <h1 class="fade-in">Choose your connection type</h1>
      <TabContainer class="fade-in">
        <TabItem label="Login/Password" @click="connectionType = 'password'"></TabItem>
        <TabItem label="OAuth2.0 Based" @click="connectionType = 'token'"></TabItem>
        <TabItem :ico="GmailIco" label="Gmail" @click="GoogleConnect"/>
      </TabContainer>
    </section>
    <section>
      <h1 class="fade-in" v-if="connectionType === 'password' || connectionType==='token'">Connection Form</h1>
      <div v-if="connectionType === 'password' || connectionType === 'token'" class="connection-form">
        <Input class="fade-in" v-model="server" label="Server" placeholder="imap.google.com" tip="Your email provider IMAP server's address."
               type="text"/>
        <Input class="fade-in" v-model="port" :max=65535 :min=0 label="Port" placeholder="993" tip="Your email provider IMAP server's port."
               type="number"/>
        <Input class="fade-in" v-model="email" label="Email" placeholder="example@email.com" tip="The email you want to clean using this app."
               type="text"/>

        <Input class="fade-in" v-if="connectionType === 'password'" v-model="password" label="Password" placeholder="pass***d"
               tip="The password associated to the email." type="password"/>
        <Input class="fade-in" v-else v-model="token" label="OAuth token" placeholder="toke***" tip="The OAuth2.0 token to authenticate to the IMAP server."
               type="text"/>

        <div>
          <Button class="fade-in" expand @click="connectionType === 'password' ? PasswordConnect() : TokenConnect()">Sign in</Button>
          <h3 class="fade-in" :style="{marginTop: 'var(--xs-spacing)'}">Please consider that IMAP must be enabled/authorised by your email provider for this application to work.<ExternalLink href="https://en.wikipedia.org/wiki/Internet_Message_Access_Protocol#Email_protocols">What's IMAP</ExternalLink></h3>
        </div>
      </div>

      <div v-else-if="connectionType === 'thirdparty'" class="thirdPartyConnexion">
        <h1>STATUS : {{ thirdPartyMessage }}</h1>

        <section>
          <h2 v-if="thirdPartyLink === ''">You will shortly be redirected to an authentication page.</h2>
          <div v-else>
            <h2>Please open this link to continue :
              <ExternalLink copybutton :href="thirdPartyLink">{{ thirdPartyLink }}</ExternalLink>
            </h2>
            <h3>Consider using another browser than firefox to open this link please.</h3>
          </div>
        </section>

      </div>
    </section>
  </section>
</template>

<style scoped>
.connection-view {
  max-width: 800px;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: start;
  justify-content: start;
  gap: var(--s-spacing);
  padding: var(--s-spacing)  var(--m-spacing);
}

.connection-view > section {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--s-spacing);
}

.connection-view > section > *:not(h1) {
  margin-left: var(--s-spacing);
}

.connection-view h1 {
  width: 100%;
  text-align: start;
}

.connection-form > * {
  margin-bottom: var(--s-spacing);
}

.connection-form > *:last-child {
  margin-bottom: 0;
  margin-top: var(--m-spacing);
}

.thirdPartyConnexion {
  width: 100%;
  margin-top: var(--m-spacing);
  --font-size: var(--l-font-size);
}

.thirdPartyConnexion h1 {
  text-transform: none;
  font-size: var(--font-size);
}

.thirdPartyConnexion h2 {
  font-size: calc(var(--font-size) * 0.75);
  margin-top: var(--s-spacing);
}
</style>