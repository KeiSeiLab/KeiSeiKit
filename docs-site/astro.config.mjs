// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// KeiSeiKit docs site config.
// Source: docs-site/src/content/docs/**
// Auto-generated input: ../docs/{primitives,skills,hooks}/*.md (produced by `keidocs` Rust primitive)
// Output: ../site/ (relative to docs-site/, i.e. KeiSeiKit-public/site/)

export default defineConfig({
  site: 'https://keisei.app',
  srcDir: './src',
  outDir: '../site',
  publicDir: './public',

  integrations: [
    starlight({
      title: 'KeiSeiKit',
      description:
        'Living wiki — auto-generated from git, signed by commit. Self-extending substrate of Rust primitives, agents, skills, and hooks.',
      // logo: {src: './src/assets/keisei-mark.svg'},  // TODO: add brand mark SVG
      social: {
        github: 'https://github.com/KeiSei84/KeiSeiKit-1.0',
      },
      customCss: ['./src/styles/keisei.css'],
      // Giscus comments injected directly by keidocs renderer (markdown
      // appends <script> per page). No Starlight component override needed.
      sidebar: [
        {
          label: 'Overview',
          items: [
            { label: 'Welcome', link: '/' },
            { label: 'Architecture', link: '/overview/architecture/' },
          ],
        },
        {
          label: 'Primitives',
          autogenerate: { directory: 'primitives' },
          collapsed: true,
        },
        {
          label: 'Skills',
          autogenerate: { directory: 'skills' },
          collapsed: true,
        },
        {
          label: 'Hooks',
          autogenerate: { directory: 'hooks' },
          collapsed: true,
        },
      ],
      components: {
        // Override defaults later as brand evolves.
      },
      lastUpdated: true,
      pagination: true,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 4 },
      editLink: {
        baseUrl:
          'https://github.com/KeiSei84/KeiSeiKit-1.0/edit/main/docs-site/',
      },
      head: [
        {
          tag: 'meta',
          attrs: { name: 'theme-color', content: '#0f1828' },
        },
      ],
    }),
  ],

  markdown: {
    shikiConfig: {
      theme: 'github-dark-default',
      wrap: true,
    },
  },
});
