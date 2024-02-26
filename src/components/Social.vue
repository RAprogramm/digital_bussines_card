<template>
  <div class="body__profile__icons">
    <a class="linkedin" href="https://www.linkedin.com/in/andrei-rozanov-4271a425b/">
      <fa :icon="['fab', 'linkedin']" />
    </a>
    <a class="telegram" href="https://t.me/Magistr_RA">
      <fa :icon="['fab', 'telegram']" />
    </a>
    <a class="google" href="mailto:andrey.rozanov.vl@gmail.com">
      <fa :icon="['fab', 'google']" />
    </a>
  </div>
</template>

<script>
  export default {
    name: 'Social'
  }
</script>

<style>
  a {
    color: #999;
  }

  .body__profile__icons {
    font-size: 3.5em;
    letter-spacing: 0.4em;
    top: -20px;
    color: #737373;
    position: relative;
  }

  .body__profile__icons a {
    font-size: inherit;
    padding: 0 0.15em;
    color: #737373;
    transition: all 0.5s;
  }

  .body__profile__icons a:hover {
    font-size: inherit;
  }

  .google:hover {
    color: #ea4335;
  }
  .linkedin:hover {
    color: #0077b5;
  }
  .telegram:hover {
    color: #229ED9;
  }
</style>
