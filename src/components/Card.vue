<template>
  <vue-flip height="100%" width="100%" v-model="show">
    <template v-slot:front>
      <div class="body__container">
        <div class="body__profile">
          <Photo :korean="korean" />
          <div class="body__profile__button-area" id="ko-kr">
            <Korean />
            <Social />
            <button class="body__profile__button github" @click="navigateTo(githubUrl)">
              GitHub에서 포트폴리오 보기
            </button>
            <button class="body__profile__button cv" @click="navigateTo(resumeUrl)">
              이력서 다운로드
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
          <div class="body__profile__button-area">
            <English />
            <Social />
            <button class="body__profile__button github" @click="navigateTo(githubUrl)">
              View portfolio on GitHub
            </button>
            <button class="body__profile__button cv" @click="navigateTo(resumeUrl)">
              Download CV
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
      githubUrl: 'https://www.github.com/RAprogramm',
      resumeUrl: 'https://drive.google.com/file/d/1--CHUjt7L6gjNzVYx_3y_IL4iWWUPGum/view?usp=drive_link',
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
    navigateTo(url) {
      window.location.href = url;
    }
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
  padding: 1em 1.3em;
  margin: 2em auto;
  max-width: 330px;
  box-shadow: 0px 0px 50px 30px black;
}

.body__profile__button-area {
  top: -50px;
  position: relative;
}

.body__profile__button a {
  text-decoration: none;
}

.body__profile button {
  border: none;
  border-radius: 10px;
  font-family: Orkney;
  text-transform: uppercase;
  padding: 1em;
  width: 100%;
  letter-spacing: 2px;
  margin: 0.3em 0;
  cursor: pointer;
  transition: all 0.5s;
  border: 1px solid;
}

.cv {
  background-color: #ffffff;
  color: #c678dd;
  border-color: #c678dd;
}

.cv:hover {
  background-color: #c678dd;
  box-shadow: 0px 0px 5px 3px #c678dd;
  text-shadow: 1px 1px darkviolet;
  font-weight: 900;
  border-color: #c678dd;
  color: #ffffff;
}

.github {
  background-color: #ffffff;
  color: #98c379;
  border-color: #98c379;
}

.github:hover {
  background-color: #98c379;
  box-shadow: 0px 0px 5px 3px #98c379;
  text-shadow: 1px 1px darkgreen;
  font-weight: 900;
  border-color: #98c379;
  color: #ffffff;
}

.body__profile__translate button {
  font-size: 0.75em;
  border: 0;
  background-color: transparent;
  padding: 0;
  text-transform: uppercase;
  letter-spacing: 3px;
  color: #999;
  top: -20px;
  cursor: pointer;
  position: relative;
  transition: all 0.5s;
}

.body__profile__translate:hover button {
  color: darkgrey;
  font-weight: 900;
  text-shadow: 1px 1px black;
}

.body__profile__button.github:hover .github-link {
  color: #ffffff;
  font-weight: bold;
}

.body__profile__button.github .github-link {
  color: #98c379;
  font-weight: bold;
}

.body__profile__button.cv .cv-link {
  color: #c678dd;
  font-weight: bold;
}

.body__profile__button.cv:hover .cv-link {
  color: #ffffff;
  font-weight: bold;
}
</style>
