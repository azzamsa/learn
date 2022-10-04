help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	pnpm i

dev: ## Develop the app.
	pnpm run dev

lint: ## Lint the codebase.
	pnpm lint

fmt:  ## Format the codebase.
	pnpx prettier --config configs/prettier.config.cjs --ignore-path configs/prettierignore --write . --plugin-search-dir=. ./**/*.svelte

fmt_check: ## Check is the codebase properly formatted.
	pnpx prettier --config configs/prettier.config.cjs --ignore-path configs/prettierignore --check . --plugin-search-dir=. ./**/*.svelte

comply: fmt lint ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.
	pnpm run check

check: fmt_check lint ## Check if the repository comply with the rules. Mostly used in CI.
	pnpm run check

##
## Misc

check_dependencies: ## Check dependencies health.
	ncu
