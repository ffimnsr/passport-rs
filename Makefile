default: serve

.PHONY: serve
serve:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x clippy -x run

.PHONY: run.only
run.only:
	@catflap -h 127.0.0.1 -p 5000 -- cargo watch -x run

.PHONY: check
check:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x clippy -x check

.PHONY: check.only
check.only:
	@catflap -h 127.0.0.1 -p 5000 -- cargo watch -x check
