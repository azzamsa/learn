help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	pnpm i

dev: ## Develop the app.
	pnpm dev

lint: ## Lint the codebase.
	pnpm lint

fmt: lint ## Format the codebase.

fmt_check: lint ## Check is the codebase properly formatted.

comply: fmt lint ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.
	pnpm run type-check

check: fmt_check lint ## Check if the repository comply with the rules. Mostly used in CI.
	pnpm run type-check

##
## Misc

check_dependencies: ## Check dependencies health.
	ncu
