// eslint-disable-next-line node/no-unpublished-require
const { ProvideSharedPlugin } = require("@rspack/core").sharing;

/** @type {import("@rspack/core").Configuration} */
module.exports = {
	plugins: [
		new ProvideSharedPlugin({
			provides: ["shared"]
		})
	]
};
