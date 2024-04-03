import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: './schema.gql',
  documents: './apps/pwa/**/*.gql',
  generates: {
    'libs/graphql-types/src/lib/graphql-types.ts': {
      plugins: ['typescript'],
    },
    'graphql/': {
      preset: 'near-operation-file',
      presetConfig: {
        extension: '.graphql.ts',
        baseTypesPath: '~@sp/graphql-types',
      },
      plugins: ['typescript-operations', 'typescript-apollo-angular'],
      config: {
        withHooks: true,
        addExplicitOverride: true,
        querySuffix: 'Query',
        mutationSuffix: 'Mutation',
        operationResultSuffix: 'Result',
      },
    },
  },
};
export default config;
