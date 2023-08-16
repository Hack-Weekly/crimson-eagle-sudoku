/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./src/**/*.{html,rs}",
		"./dist/**/*.html",
	],
	theme: {
		extend: {
			boxShadow: {
				glow: '0 0 20px 10px rgba(225 29 72 / 0.5)'
			},
			width: {
				'1/9': '10%'
			}
		},
	},
	plugins: [],
}

