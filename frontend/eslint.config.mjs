// @ts-check

import eslintjs from '@eslint/js';
import eslintreact from "eslint-plugin-react";
import eslintts from 'typescript-eslint';

export default eslintts.config(
	eslintjs.configs.all,
	eslintts.configs.strictTypeChecked,
	eslintts.configs.stylisticTypeChecked,
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
	eslintreact.configs.flat.recommended,
);
