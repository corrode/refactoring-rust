.PHONY: help
help: ## Show this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: test
test: ## Run tests; intentional starter failures are verified separately by integration tests
	cargo test --examples --tests --all-features -- --exact \
		--skip tests::variable_and_declaration_ids_are_distinct \
		--skip tests::debug_does_not_leak_token \
		--skip tests::self_friendship_is_a_noop \
		--skip tests::people_with_same_name_are_distinct

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

.PHONY: book-runner-test
book-runner-test: ## Test the browser runner and rendered-snippet checker (Node.js + Python 3.10+)
	node --test tests/playground-tests.test.cjs tests/themes.test.cjs
	python3 -m unittest discover -s tests -p 'test_book_snippets.py'

.PHONY: book-snippet-check
book-snippet-check: book-build ## Compile and run the exact rendered snippets and their attached tests
	python3 scripts/check_book_snippets.py book/book

.PHONY: book-check
book-check: test book-test book-runner-test book-snippet-check ## Full CI replica, including browser tests and rendered Rust snippets

.PHONY: book-open
book-open: book-build ## Build the book once and open the static output
	open book/book/index.html
