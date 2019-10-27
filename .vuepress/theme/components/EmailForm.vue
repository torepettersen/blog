<template lang="pug">
div
  form.subscribe-form(
    action='https://dev.us20.list-manage.com/subscribe/post?u=b5cba5b1a8baa382a74542a78&amp;id=ddc2acb3df',
    method='post'
  )
    input(name='email', type='email', placeholder='Your email address')
    input(type='hidden' name='b_b5cba5b1a8baa382a74542a78_ddc2acb3df' tabindex='-1' value='')
    button.submit(type='submit', name='subscribe') Subscribe
</template>



<style lang="scss" scoped>
@import "../styles/palette.scss";
$borderRadius: 0.5rem;

.subscribe-form {
  display: flex;

  & * {
    font-size: 0.9rem;
    line-height: 2rem;
    padding: 0 1rem;
    display: inline-block;
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
    border: 1px solid $accentColor;
    outline: none;
    transition: all 0.2s ease;
  }

  .submit {
    cursor: pointer;
    background-color: $accentColor;
    border: 0;
    color: #fff;
    transition: background-color .1s ease;
  }
}


</style>