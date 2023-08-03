<template>
  <div class="body__profile__title">
    <h1>안드레이 로자노프</h1>
    <h2>프로그래머</h2>
    <h2>풀스택 개발자</h2>
    <div class="body__profile__bio-pt">
      {{ bioKr }}
    </div>
  </div>
</template>

<script>
export default {
  name: 'Korean',
  data: () => ({
    bioKr: `HTML, CSS, Javascript, Vue, Lua, Python, GoLang 작업하고 매일 새로운 것을 배우는 개발자.`
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
