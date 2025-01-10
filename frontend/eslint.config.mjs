import eslintjs from '@eslint/js';
import eslintreact from "eslint-plugin-react";
import eslintts from 'typescript-eslint';

export default eslintts.config(
	eslintjs.configs.recommended,
	eslintreact.configs.flat.all,
	eslintts.configs.recommended,
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
			// Frontend coverage generated files
			"coverage/",
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
			"react/forbid-component-props": [ 'off' ],
			"react/jsx-filename-extension": [ 'error', { "extensions": [".tsx", ".jsx"] } ],
			"react/jsx-indent": [ 'error', 'tab', { checkAttributes: true, indentLogicalExpressions: true } ],
			"react/jsx-max-depth": [ 'off' ],
			"react/jsx-no-literals": [ 'off' ],
		},
	},
);
