<template>
  <transition
    mode="out-in"
    name="bounce"
    appear
  >
    <div
      v-if="korean"
      class="body__profile__photo"
    >
      <img
        src="../assets/face.jpg"
        alt="Face"
      />
    </div>
    <div
      v-else
      class="body__profile__photo2"
    >
      <img
        src="../assets/face2.jpg"
        alt="in Korea"
      />
    </div>
  </transition>
</template>

<script>
  export default {
    name: 'Photo',
    props: ['korean']
  }
</script>

<style>
  .bounce-enter-active {
    animation: bounce-in 0.5s;
  }

  .bounce-leave-active {
    animation: bounce-in 0.5s reverse;
  }

  @keyframes bounce-in {
    0% {
      transform: scale(0);
    }

    50% {
      transform: scale(1.25);
    }

    100% {
      transform: scale(1);
    }
  }

  .v-enter-active,
  .v-leave-active {
    transition: opacity 1s ease;
  }

  .v-enter-from,
  .v-leave-to {
    opacity: 0;
  }

  .body__profile__photo img {
    border-radius: 50%;
    height: 250px;
    border: 15px solid #fff;
    top: -100px;
    position: relative;
    box-shadow: 1px 2px 18px 4px rgb(226, 226, 226);
  }

  .body__profile__photo2 img {
    border-radius: 50%;
    height: 250px;
    border: 15px solid #fff;
    top: -100px;
    position: relative;
    box-shadow: 1px 2px 18px 4px rgb(226, 226, 226);
  }
</style>
