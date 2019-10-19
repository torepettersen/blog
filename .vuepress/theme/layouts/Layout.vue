<template lang="pug">
	.theme-container(:class='pageClasses', @touchstart='onTouchStart', @touchend="onTouchEnd")
		
		Navbar(v-if='shouldShowNavbar', @toggle-sidebar='toggleSidebar')

		Sidebar(:items='sidebarItems', @toggle-sidebar='toggleSidebar')

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
import Sidebar from '@theme/components/Sidebar.vue';
import spacetime from 'spacetime';
import { resolveSidebarItems } from '../util';

export default {
	components: {
		Navbar,
		Sidebar,
	},

	data () {
    return {
      isSidebarOpen: false
    }
  },
  computed: {
    shouldShowNavbar () {
      const { themeConfig } = this.$site
      const { frontmatter } = this.$page
      if (
        frontmatter.navbar === false
        || themeConfig.navbar === false) {
        return false
      }
      return (
        this.$title
        || themeConfig.logo
        || themeConfig.repo
        || themeConfig.nav
        || this.$themeLocaleConfig.nav
      )
    },
    shouldShowSidebar () {
      const { frontmatter } = this.$page
      return (
        !frontmatter.home
        && frontmatter.sidebar !== false
        && this.sidebarItems.length
      )
    },
    sidebarItems () {
      return resolveSidebarItems(
        this.$page,
        this.$page.regularPath,
        this.$site,
        this.$localePath
      )
    },
    pageClasses () {
      const userPageClass = this.$page.frontmatter.pageClass
      return [
        {
          'no-navbar': !this.shouldShowNavbar,
          'sidebar-open': this.isSidebarOpen,
          'no-sidebar': !this.shouldShowSidebar
        },
        userPageClass
      ]
    }
  },

	mounted () {
    this.$router.afterEach(() => {
      this.isSidebarOpen = false
    })
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
		},
		toggleSidebar (to) {
      this.isSidebarOpen = typeof to === 'boolean' ? to : !this.isSidebarOpen
      this.$emit('toggle-sidebar', this.isSidebarOpen)
    },
    // side swipe
    onTouchStart (e) {
      this.touchStart = {
        x: e.changedTouches[0].clientX,
        y: e.changedTouches[0].clientY
      }
    },
    onTouchEnd (e) {
      const dx = e.changedTouches[0].clientX - this.touchStart.x
      const dy = e.changedTouches[0].clientY - this.touchStart.y
      if (Math.abs(dx) > Math.abs(dy) && Math.abs(dx) > 40) {
        if (dx > 0 && this.touchStart.x <= 80) {
          this.toggleSidebar(true)
        } else {
          this.toggleSidebar(false)
        }
      }
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