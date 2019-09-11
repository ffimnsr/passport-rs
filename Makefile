default: serve

.PHONY: serve
serve:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x run

.PHONY: run.only
run.only:
	@catflap -h 0.0.0.0 -p 5000 -- cargo watch -x run

.PHONY: check
check:
	@cargo watch -x clippy -x check

.PHONY: check.only
check.only:
	@cargo watch -x check

.PHONY: sync
sync:
	@bash -c "ag -l | entr -s 'rsync -avz --exclude .git --exclude target --exclude logs . discoursenet:~/builder/pp'"
