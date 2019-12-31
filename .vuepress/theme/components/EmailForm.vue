<template lang="pug">
.form-container
  h4 Never miss an article about about building the cloud with Rust
  form.subscribe-form(@submit.prevent='submit', novalidate)
    input(type='email', v-model='params.EMAIL', placeholder='Your email address')
    button.submit(type='submit') Subscribe
  .message(v-html='message')
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      message: '',
      params: {
        EMAIL: '',
        u: 'b5cba5b1a8baa382a74542a78',
        id: 'ddc2acb3df',
      },
    }
  },

  methods: {
    async submit() {
      try {
        const { data } = await axios.get('/api/subscribe', { params: this.params });
        if (data.result === 'error') {
          if (/0 -/.test(data.msg)) {
            this.message = 'Looks like that email address is invalid. Mind trying again?';
          }
          else {
            this.message = data.msg;
          }
        }
        else if (data.result === 'success') {
          this.message = 'Almost done. Please confirm your email by clicking the link in the email that was just sent.';
        }
      }
      catch (err) {
        this.message = 'Seems like something went wrong! Please check that you are still online or try again later.';
      }

    }
  }
}
</script>

<style lang="scss" scoped>
@import "../styles/palette.scss";
$borderRadius: 0.5rem;

.form-container {
  text-align: center;
}

.subscribe-form {
  & * {
    font-size: 0.9rem;
    line-height: 2rem;
    padding: 0 1rem;
    display: inline-block;
    border: 1px solid $accentColor;
  }

  :first-child {
    border-radius: $borderRadius 0 0 $borderRadius;
  }

  :last-child {
    border-radius: 0 $borderRadius $borderRadius 0;
  }

  :not(:last-child) {
    border-right: 0;
  }

  input {
    cursor: text;
    color: #4e6e8e;
    outline: none;
    transition: all 0.2s ease;
  }

  .submit {
    cursor: pointer;
    background-color: $accentColor;
    color: #fff;
    transition: background-color .1s ease;
  }
}

.message {
  margin-top: 0.2rem;
  font-size: 0.9rem;
}

</style>