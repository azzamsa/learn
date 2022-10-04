help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	dprint --version || cargo install dprint

	pnpm i
	pnpx husky install

dev: ## Develop the app.
	pnpm run dev

lint: ## Lint the codebase.
	pnpm run lint
	pnpm run check

fmt:  ## Format the codebase.
	pnpm run format
	dprint fmt --config configs/dprint.json

fmt_check: ## Check is the codebase properly formatted.
	pnpm run format:check
	dprint check --config configs/dprint.json

comply: fmt lint ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check: fmt_check lint ## Check if the repository comply with the rules. Mostly used in CI.

release:  ## Create a release
	bash scripts/release.sh $(version)

##
## Misc

check_dependencies: ## Check dependencies health.
	ncu
