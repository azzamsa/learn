const postcssRustHelpers = require("../scripts/postcss_rust_helpers")

module.exports = (ctx) => {
  // we want to filter out unused css classes in production mode
  // NOTE: ctx.env is set in postcss-cli `--env` configs

  var usedCssClasses = null
  if (ctx.env == "production") {
    usedCssClasses = postcssRustHelpers.getUsedCssClasses()
  } else {
    usedCSSClasses = null
  }

  console.log("postcss ctx.env: " + ctx.env)
  return {
    // allow inline comment in SASS
    syntax: "postcss-scss",
    plugins: [
      require("postcss-import"),
      require("tailwindcss")("configs/tailwind.config.js"),
      require("autoprefixer"),
      require("postcss-typed-css-classes")({
        output_filepath: "src/generated/css_classes.rs",
        generator: "rust",
        filter: (tailwind_class) => {
          if (ctx.env === "production") {
            // convert TailwindCSS class to typed class.
            var typed_class = postcssRustHelpers.transformToTypedClass(tailwind_class)
            // check if the class is in the file
            const isUsedRustClass = usedCssClasses.has(typed_class)
            return isUsedRustClass
          } else {
            return true
          }
        },
      }),
    ],
  }
}
