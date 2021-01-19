module.exports = {
  root: true,
  env: {
    node: true,
  },
  ignorePatterns: ['_old/**'],
  extends: [
    'plugin:vue/essential',
    '@vue/airbnb',
    '@vue/typescript/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2020,
  },
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-param-reassign': 'off',
    'lines-between-class-members': 'off',
    'import/prefer-default-export': 'off',
    'no-multiple-empty-lines': ['error', { max: 2 }],
    '@typescript-eslint/explicit-module-boundary-types': 'off',
    '@typescript-eslint/no-non-null-asserted-optional-chain': 'off',
  },
};
