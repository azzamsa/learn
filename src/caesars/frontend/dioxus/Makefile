help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	rustup target add wasm32-unknown-unknown

	trunk --version || cargo install trunk
	microserver --version || cargo install microserver

clean: ## Clean build artifacts.
	trunk clean

dev: ## Develop the app.
	pnpx tailwindcss --config configs/tailwind.config.js --input styles/main.scss  --output styles/compiled.scss
	trunk serve

build: ## Build the app.
	NODE_ENV=production pnpx tailwindcss --config configs/tailwind.config.js --input styles/main.scss --output styles/compiled.scss
	trunk build --release

preview-build: ## Preview the build output.
	microserver dist/

fmt: ## Format the codebase.
	cargo +nightly fmt --all
	prettier --config configs/prettier.config.js --ignore-path configs/prettierignore --write .

fmt_check: ## Check is the codebase properly formatted.
	cargo +nightly fmt --check
	prettier --config configs/prettier.config.js --ignore-path configs/prettierignore --check .

lint: ## Lint the codebase.
	cargo clippy --all-targets --all-features

doc_check: ## Check the documentation.
	cargo doc --all-features --no-deps

test: ## Lint the codebase.
	cargo test --all --all-features --all-targets

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check: fmt_check lint test doc_check ## Check if the repository comply with the rules. Mostly used in CI.

##
## Misc

update_graphql_schema: ## Update the schema
	## run the app before running this command `cargo r`
	get-graphql-schema http://127.0.0.1:8000/graphql > src/graphql/schema.graphql

check_dependencies: ## Check dependencies health.
	cargo +nightly udeps
	cargo outdated --root-deps-only
