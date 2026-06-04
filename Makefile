.PHONY: help
help: ## Show this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: test
test: ## Run all tests (starters + solutions)
	cargo test --examples

.PHONY: dev
dev: ## Start the Slidev dev server
	cd slides && npm run dev

# ---------------------------------------------------------------------------
# Book targets (see book/PLAN.md)
# ---------------------------------------------------------------------------

.PHONY: book
book: book-serve ## Alias: open the book in a browser with live reload

.PHONY: book-serve
book-serve: ## Serve the book locally with live reload, open in browser
	cd book && mdbook serve --open

.PHONY: book-build
book-build: ## Build the book to book/book/
	cd book && mdbook build

.PHONY: book-test
book-test: ## Compile every inline rust block in the book
	cd book && mdbook test

.PHONY: book-clean
book-clean: ## Remove the generated book output
	cd book && mdbook clean

.PHONY: book-check
book-check: test book-test book-build ## Full CI replica: cargo test + mdbook test + mdbook build

.PHONY: book-open
book-open: book-build ## Build the book once and open the static output
	open book/book/index.html
