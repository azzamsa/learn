declare module '*.yaml' {
  const json: Record<string, string>
  export default json
}

// declare module '*.vue' {
//   import { type DefineComponent } from 'vue'
//   const component: DefineComponent<{}, {}, any>
//   export default component
// }
