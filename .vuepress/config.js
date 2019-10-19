module.exports = {
  title: 'Cloudmaker',
  dest: 'dist',
  themeConfig: {
    nav: [
      { text: 'Blog', link: '/' },
      { text: 'About me', link: '/about' },      
    ]
  },
  plugins: [
    [
      '@vuepress/blog',
      {
        directories: [
          {
            // Unique ID of current classification
            id: 'blog',
            // Target directory
            dirname: 'blog',
            // Path of the `entry page` (or `list page`)
            path: '/',
            itemPermalink: ':slug',
            pagination: {
              sorter: (prev, next) => {
                const prevTime = new Date(prev.frontmatter.date).getTime();
                const nextTime = new Date(next.frontmatter.date).getTime();
                return nextTime - prevTime;
              },
            },
          },
        ],
      },
    ],
    // ['vuepress-plugin-clean-urls'],
  ],
}