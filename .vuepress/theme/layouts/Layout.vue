<template lang="pug">
	.theme-container.no-sidebar
		
		Navbar


		main.page
			slot(name='header')
			
			.theme-default-content
				slot
					h1(:id='toKebabCase($frontmatter.title)')
							a.header-anchor(:href='`#${toKebabCase($page.frontmatter.title)}`', area-hidden='true') #
							| {{ $frontmatter.title }}
					span.meta
						template(v-if="$page.frontmatter.date")
							i.fas.fa-clock
							span  {{ formatDate($page.frontmatter.date) }}&ensp;
						template(v-if="$page.frontmatter.categories && $page.frontmatter.categories.length")
							i.fas.fa-layer-group
							span  {{ formatCategories($page.frontmatter.categories) }}
					Content

				footer
					p &copy; Tore Pettersen
		

</template>

<script>
import Navbar from '@theme/components/Navbar.vue';
import spacetime from 'spacetime';

export default {
	components: {
		Navbar,
	},

	methods: {
    formatDate(date) {
      return spacetime(date).format('{month-short} {date-ordinal}, {year}');
    },
    formatCategories(categories) {
      return categories.join(', ');
    },
    toKebabCase(string) {
      return string
        .replace(/[^a-zA-Z\s]/g, '')
        .replace(/\s+/g, '-')
        .toLowerCase();
    }
  }
}
</script>

<style lang="scss">
@import "../styles/fontawesome.scss";

.content__default {
	margin-top: 1.8rem;
}

footer {
	margin-top: 5rem;
	border-top: 1px solid #eaecef;  

  p {
    padding: 0.8rem 0 1rem;
  }
}
</style>