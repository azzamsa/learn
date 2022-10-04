help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	pnpm i
	pnpx husky install

dev: ## Develop the app.
	pnpm dev

lint: ## Lint the codebase.
	pnpm run lint
	pnpm run type-check

fmt: lint ## Format the codebase.
	dprint fmt --config configs/dprint.json

fmt_check: lint ## Check is the codebase properly formatted.
	dprint check --config configs/dprint.json

test: ## Test the codebase.
	pnpm run test

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check: fmt_check lint test ## Check if the repository comply with the rules. Mostly used in CI.

release:  ## Create a release
	bash scripts/release.sh $(version)

##
## Misc

check_dependencies: ## Check dependencies health.
	ncu
