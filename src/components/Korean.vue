<template>
  <div class="body__profile__title">
    <h1>안드레이 로자노프</h1>
    <h2>소프트웨어 엔지니어</h2>
    <div class="body__profile__bio-pt">
      {{ bioKr }}
    </div>
  </div>
</template>

<script>
  export default {
    name: 'Korean',
    data: () => ({
      bioKr: `많은 프로그래밍 언어와 함께 일하며 매일 새로운 것을 배우는 개발자.`
    })
  }
</script>

<style>
  .body__profile__title {
    top: -20px;
    position: relative;
  }

  .body__profile__title h1 {
    font-size: 1.5em;
    font-weight: 900;
    text-align: center;
    color: rgb(84, 84, 84);
    margin: 0 0 0.5em 0;
  }

  .body__profile__title h2 {
    font-size: 0.75em;
    color: rgb(153, 153, 153);
    line-height: 1.391;
    font-weight: lighter;
    text-align: center;
    text-transform: uppercase;
  }

  .body__profile__bio-pt {
    font-size: 0.75em;
    color: rgb(171, 171, 171);
    font-weight: 900;
    line-height: 1.404;
    text-align: center;
    margin: 1em 0;
    padding: 1.25em 0;
    border-top: 4px solid whitesmoke;
    border-bottom: 4px solid whitesmoke;
    transition: all 0.5s ease-in-out;
  }

  .body__profile__bio-pt:hover {
    transition: all 0.5s ease-in-out;
  }
</style>
