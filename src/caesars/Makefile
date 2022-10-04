.DEFAULT_GOAL := help

help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository
	echo "::: Setting up..."
	git cliff --version || cargo install git-cliff
	dprint --version || cargo install dprint

fmt: ## Format the codebase.
	dprint fmt --config configs/dprint.json

fmt_check: ## Check is the codebase properly formatted.
	dprint check --config configs/dprint.json

comply: fmt ## Tasks to make the code-base comply with the rules. Mostly used in git hooks.

check: fmt_check ## Check if the repository comply with the rules and ready to be pushed.

release:  ## Create a release
	bash scripts/release.sh $(version)

