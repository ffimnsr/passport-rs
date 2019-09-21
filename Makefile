default: serve

.PHONY: serve
serve:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x run

.PHONY: serve.lint
run.lint:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x clippy -x run

.PHONY: check
check:
	@cargo watch -x check

.PHONY: check.lint
check.lint:
	@cargo watch -x clippy -x check

.PHONY: trigger
trigger:
	@cargo watch -x check -w src -i .trigger -s "touch .trigger"

.PHONY: trigger.watch
trigger.watch:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch --no-gitignore -w .trigger -x run

.PHONY: sync
sync:
	@bash -c "ag -l | entr -s 'rsync -avz --exclude .git --exclude target --exclude logs . discoursenet:~/builder/pp'"
