<script setup>

</script>

<template>
  <h1>Coderunner</h1>
  <p>
    A website where you can write [...] code, run it on a serverside server, and get the output in return.
  </p>

  <h2>Insert code to be compiled and rendered serverside here:</h2>
  <textarea v-model="inputCode"></textarea>

  <h2>Output:</h2>
  <output v-bind>output</output>
  


</template>

<style scoped></style>
