.PHONY: check-model test

check-model:
	weaver registry check -r ./model

test:
	@output=$$(weaver registry live-check -r model --input-source samples/metrics.json --advice-policies policies 2>&1 || true); \
	echo "$$output"; \
	echo "$$output" | grep -q "invalid_value_case"
