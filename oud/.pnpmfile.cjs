"use strict";

module.exports = {
	hooks: {
		readPackage(pkg, context) {
			if (pkg.name === "nuxt") {
				delete pkg.dependencies["@nuxt/telemetry"];
				context.log("nuxt: deleted telemetry with the power of pnpm muahahah");
			}

			return pkg;
		}
	}
};
