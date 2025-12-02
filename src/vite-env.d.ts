/// <reference types="vite/client" />

declare module 'vue3-markdown-it' {
  import { Component } from 'vue'
  const Vue3MarkdownIt: Component
  export default Vue3MarkdownIt
}

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
