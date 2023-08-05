<template>
  <vue-flip height="100%" width="100%" v-model="show">
    <template v-slot:front>
      <div class="body__container">
        <div class="body__profile">
          <Photo :korean="korean" />
          <div class="body__profile__button-area" id="ko-kr">
            <Korean />
            <Social />
            <button class="body__profile__button github">
              <a href="https://www.github.com/RAprogramm">GitHub에서 포트폴리오 보기</a>
            </button>
            <button class="body__profile__button cv">
              <a v-bind:href="resume">이력서를 PDF로 다운로드</a>
            </button>
          </div>

          <div class="body__profile__translate">
            <button @click="translate" v-on:click="fadeMe">
              <fa icon="flag" /> Translate {{ enFlag }}
            </button>
          </div>

        </div>
      </div>
    </template>

    <template v-slot:back>
      <div class="body__container">
        <div class="body__profile">
          <Photo :korean="korean" />
          <div  class="body__profile__button-area">
            <English />
            <Social />
            <button class="body__profile__button github botao">
              <a href="https://www.github.com/RAprogramm">View portfolio on GitHub</a>
            </button>
            <button class="body__profile__button cv botao">
              <a v-bind:href="resume">Download Resume in PDF </a>
            </button>
          </div>
          <div class="body__profile__translate">
            <button @click="translate" v-on:click="fadeMe">
              <fa icon="flag" /> 번역하기 {{ krFlag }}
            </button>
          </div>
        </div>
      </div>
    </template>
  </vue-flip>
</template>

<script>
import Photo from './Photo'
import { VueFlip } from 'vue-flip'
import Korean from './Korean'
import Social from './Social'
import English from './English'

export default {
  name: 'Card',
  data() {
    return {
      trans: Boolean(true),
      korean: Boolean,
      resume:
        'https://github.com/RAprogramm/RAprogramm/files/12246611/AndreiRozanov.pdf',
      krFlag: '(KR)',
      enFlag: '(EN)',
      show: false
    }
  },
  components: {
    Photo,
    'vue-flip': VueFlip,
    Korean,
    English,
    Social
  },
  methods: {
     translate() {
       this.korean = !this.korean
       this.trans = !this.trans
     },
     fadeMe: function () {
       this.show = !this.show
     },
  }
}
</script>

<style>
.body__container {
  margin: 0 auto;
  display: flex;
  justify-content: center;
  align-items: center;
}

.body__profile {
  background: white;
  border-radius: 50px;
  padding: 1.5em 1.75em;
  margin: 2em auto;
  max-width: 330px;
  box-shadow: 0px 3px 136px 0px rgba(88, 87, 119, 0.21);
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
