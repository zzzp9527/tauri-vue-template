module.exports = {
  root: true,
  env: {
    node: true,
    browser: true,
    es2021: true,
  },
  parser: "vue-eslint-parser", // 指定 Vue 解析器
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    parser: "@typescript-eslint/parser", // 指定 TS 解析器
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended", // TypeScript 官方推荐规则
    "plugin:vue/vue3-recommended", // Vue3 官方推荐规则

    // 确保这是 extends 数组的最后一项
    // 它会关闭所有与 Prettier 格式化冲突的 ESLint 规则。
    "eslint-config-prettier",
  ],
  // 避免和 prettier 冲突
  rules: {
    "vue/multi-word-component-names": "warn",
    "@typescript-eslint/no-explicit-any": "warn",
  },
};
