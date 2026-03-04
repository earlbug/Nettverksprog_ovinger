<script setup>
import axios from 'axios'
import { ref } from 'vue'

  const inputCode = ref("")
  const outputCode = ref("")

  function sendCode() {
    axios({
      method:'post',
      url:'http://localhost:4000',
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*'
      },
      data: {
        'code': inputCode.value
      },
    })
    .then(function (response) {
      outputCode.value = response.data.result
    })
    .catch(function (error) {
      outputCode.value = error.data.result
    });
    

  }

</script>

<template>
  <h1>Coderunner</h1>
  <p>
    A website where you can write [...] code, run it on a serverside server, and get the output in return.
  </p>

  <h2>Insert code to be compiled and rendered serverside here:</h2>
  <textarea v-model="inputCode"></textarea>
  <p></p>
  <button @click="sendCode">Send & Run</button>

  <h2>Output:</h2>
  <textarea disabled="true" v-model="outputCode"></textarea>  


</template>

<style scoped></style>
