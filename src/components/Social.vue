<template>
  <div class="body__profile__icons">
    <a class="linkedin" @click.prevent="navigateTo(linkedinUrl)">
      <fa :icon="['fab', 'linkedin']" />
    </a>
    <a class="telegram" @click.prevent="navigateTo(telegramUrl)">
      <fa :icon="['fab', 'telegram']" />
    </a>
    <a class="google" @click.prevent="navigateToMail(googleMail)">
      <fa :icon="['fab', 'google']" />
    </a>
  </div>
</template>

<script>
export default {
  name: 'Social',
  data() {
    return {
      linkedinUrl: 'https://www.linkedin.com/in/andrei-rozanov-4271a425b/',
      telegramUrl: 'https://t.me/Magistr_RA',
      googleMail: 'mailto:andrey.rozanov.vl@gmail.com'
    }
  },
  methods: {
    navigateTo(url) {
      window.location.href = url;
    },
    navigateToMail(mail) {
      window.location.href = mail;
    }
  }
}
</script>

<style>
a {
  color: #999;
  text-decoration: none;
  position: relative;
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

.body__profile__icons a::after {
  content: '';
  position: absolute;
  width: 1.5em;
  height: 1.5em;
  border-radius: 50%;
  bottom: -0.2em; 
  left: 50%;
  transform: translateX(-50%);
  z-index: -1; 
  opacity: 0.3;
  transition: all 0.5s;
}

.google::after {
  background-color: #ea4335;
}

.linkedin::after {
  background-color: #0077b5;
}

.telegram::after {
  background-color: #229ED9;
}

.google:hover::after,
.linkedin:hover::after,
.telegram:hover::after {
  opacity: 0.6; 
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
