import eslintjs from '@eslint/js';
import eslintreact from "eslint-plugin-react";
import eslintts from 'typescript-eslint';

export default eslintts.config(
	eslintjs.configs.all,
	eslintreact.configs.flat.all,
	eslintts.configs.all,
	{
		languageOptions: {
			parserOptions: {
				projectService: {
					allowDefaultProject: ['*.js', '*.mjs', '*.ts', '*.tsx'],
				},
				tsconfigRootDir: import.meta.dirname,
			},
		},
	},
	{
		// Disable linting on generated files
		ignores: [
			// Backend OpenAPI generated files
			"src/api/",
		],
	},
	{
		files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"]
	},
	{
		files: ["**/*.js"],
		languageOptions: {
			sourceType: "commonjs"
		}
	},
	{
		rules: {
			"react/jsx-filename-extension": ['error', { "extensions": [".tsx", ".jsx"] }],
			"react/jsx-indent": [ 'error', 'tab', { checkAttributes: true, indentLogicalExpressions: true } ],
		},
	},
);
