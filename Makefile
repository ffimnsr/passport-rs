default: serve

.PHONY: serve
serve:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x clippy -x run

.PHONY: check
check:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x clippy -x check
