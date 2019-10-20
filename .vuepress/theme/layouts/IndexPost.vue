<template lang="pug">
	Layout
		template(v-slot:header)
			.header-image(style="background-image: url(/img/list-cover.jpg)")
		template
			.list
				article.list-item(v-for="page in $pagination.pages")
					router-link(:to="page.path")
						h2 {{ page.title }}
					p {{ page.frontmatter.description || page.description }}
					template(v-if="page.frontmatter.date")
						i.fas.fa-clock
						span  {{ formatDate(page.frontmatter.date) }}&ensp;
					template(v-if="page.frontmatter.categories && page.frontmatter.categories.length")
						i.fas.fa-layer-group
						span  {{ formatCategories(page.frontmatter.categories) }}
</template>

<script>
import Layout from '@theme/layouts/Layout';
import spacetime from 'spacetime';

export default {
	components: {
		Layout,
	},

	methods: {
    formatDate(date) {
      return spacetime(date).format('{month-short} {date-ordinal}, {year}');
    },
    formatCategories(categories) {
      return categories.join(', ');
    }
  }
}
</script>

<style lang="scss" scoped>
@import "../styles/fontawesome.scss";

.theme-default-content:not(.custom) {
  & > .list {
    margin-top: 0;
  }
}

.header-image {
  margin-top: 3.6rem;
  height: 30vh;
  background-size: cover;
  background-repeat: no-repeat;
  background-position: 50% 20%;
  color: white;
  text-align: center;
  position: relative;

  h1 {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
}

.list-item {
  margin-bottom: 1.5rem;
  padding-bottom: 1.5rem;

  &:not(:last-child) {
    border-bottom: 1px solid #f7eeee;
  }

  h2 {
    font-size: 1.8rem;
    border-bottom: none;
    margin-bottom: 0.2rem;
    color: #2c3e50;
  }

  p {
    margin: 0 0 0.6rem;
  }
}
</style>