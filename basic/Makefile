.PHONY: build test check-model generate check-generated generate-docs check-generated-docs

build:
	cargo build

test:
	cargo test

check-model:
	weaver registry check -r ./model

generate:
	weaver registry generate -r ./model --templates ./templates rust ./src/generated

check-generated: generate
	@if ! git diff --exit-code ./src/generated; then \
		echo "FAIL: Generated code is out of sync!"; \
		echo "Run 'make generate' and commit the changes."; \
		exit 1; \
	fi
	@echo "OK: Generated code is up to date"

generate-docs:
	weaver registry generate \
		-r ./model \
		--templates "https://github.com/open-telemetry/semantic-conventions/archive/refs/tags/v1.37.0.zip[templates]" \
		markdown ./docs

check-generated-docs: generate-docs
	@if ! git diff --exit-code ./docs; then \
		echo "FAIL: Generated documentation is out of sync!"; \
		echo "Run 'make generate-docs' and commit the changes."; \
		exit 1; \
	fi
	@echo "OK: Generated documentation is up to date"
