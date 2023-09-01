<template>
  <div class="body__profile__icons">
    <a href="https://www.linkedin.com/in/andrei-rozanov-4271a425b/">
      <fa :icon="['fab', 'linkedin']" />
    </a>
    <a href="https://t.me/Magistr_RA">
      <fa :icon="['fab', 'telegram']" />
    </a>
    <a href="mailto:andrey.rozanov.vl@gmail.com">
      <fa :icon="['fab', 'google']" />
    </a>
    <!-- <div> -->
    <!--   <script -->
    <!--     src="https://storage.ko-fi.com/cdn/scripts/overlay-widget.js" -->
    <!--   ></script> -->
    <!--   <script -->
    <!--   > -->
    <!--     kofiWidgetOverlay.draw('rozanov', { -->
    <!--       type: 'floating-chat', -->
    <!--       'floating-chat.donateButton.text': 'Donate', -->
    <!--       'floating-chat.donateButton.background-color': '#5cb85c', -->
    <!--       'floating-chat.donateButton.text-color': '#fff' -->
    <!--     }) -->
    <!--   </script> -->
    <!-- </div> -->
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
    color: #ddd;
  }
</style>
