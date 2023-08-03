<template>
  <div id="demo">
    <div v-if="korean" class="body__profile__button-area" id="ko-kr">
      <transition name="fade" v-on:enter="enter">
        <Korean />
      </transition>
      <Social />
      <button class="body__profile__button github">
        <a href="https://www.github.com/RAprogramm">GitHub에서 포트폴리오 보기</a>
      </button>
      <button class="body__profile__button cv">
        <a v-bind:href="resume">이력서를 PDF로 다운로드</a>
      </button>
    </div>

    <div v-else class="body__profile__button-area">
      <transition name="fade" v-on:enter="enter">
        <English />
      </transition>
      <Social />
      <button class="body__profile__button github botao">
        <a href="https://www.github.com/RAprogramm">View portfolio on GitHub</a>
      </button>
      <button class="body__profile__button cv botao">
        <a v-bind:href="resume">Download Resume in PDF </a>
      </button>
    </div>
    <div class="body__profile__translate">
      <button @click="translate" v-if="trans" v-on:click="fadeMe">
        <fa icon="flag" /> Translate {{ enFlag }}
      </button>
      <button @click="translate" v-else v-on:click="fadeMe">
        <fa icon="flag" /> 번역하기 (KR)
      </button>
    </div>
  </div>
</template>

<script>
import Korean from './Korean'
import English from './English'
import Social from './Social'

export default {
  name: 'Buttons',
  el: '#demo',
  components: {
    Korean,
    English,
    Social
  },
  data() {
    return {
      trans: Boolean(true),
      korean: Boolean,
      resume:
        'https://github.com/RAprogramm/RAprogramm/files/12246611/AndreiRozanov.pdf',
      krFLag: '(KO-KR)',
      enFlag: '(EN)',
      show: false
    }
  },
  methods: {
    translate() {
      this.korean = !this.korean
      this.trans = !this.trans
    },

    fadeMe: function () {
      this.show = !this.show
    },

    enter: function () {
      var that = this

      setTimeout(function () {
        that.show = false
      }, 3000) // hide the message after 3 seconds
    }
  }
}
</script>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 1s;
}

.fade-enter,
.fade-leave-to

/* .fade-leave-active in <2.1.8 */
  {
  opacity: 0;
}

.body__profile__button-area {
  top: -40px;
  position: relative;
}

.body__profile button a {
  text-decoration: none;
  color: white;
}

.body__profile button {
  border: none;
  border-radius: 10px;
  font-family: Orkney;
  text-transform: uppercase;
  padding: 1.5em 1em;
  width: 100%;
  color: white;
  letter-spacing: 2px;
  font-weight: bold;
  margin: 0.5em 0;
  cursor: pointer;
  transition: all 0.5s;
}

.github {
  background-color: #7ed8a7;
}

.github:hover {
  background-color: #66df6d;
  box-shadow: 3px 3px 5px gray;
}

.cv {
  background-color: #54dd89;
}

.cv:hover {
  background-color: #54e53d;
  box-shadow: 3px 3px 5px gray;
}

.body__profile__translate button {
  font-size: 0.75em;
  border: 0;
  background-color: transparent;
  padding: 0;
  text-transform: uppercase;
  letter-spacing: 3px;
  color: #999;
  top: -10px;
  cursor: pointer;
  position: relative;
  transition: all 0.5s;
}
</style>
