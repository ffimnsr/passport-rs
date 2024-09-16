default: serve

serve:
	systemfd --no-pid -s http::5000 -- cargo watch -x run

run.lint:
	systemfd -s http::5000 -- cargo watch -x clippy -x run

trigger.watch:
	systemfd -s http::5000 -- cargo watch --no-gitignore -w .trigger -x run

check:
	cargo watch -x check

check.lint:
	cargo watch -x clippy -x check

trigger:
	cargo watch -x check -w src -i .trigger -s "touch .trigger"

sync:
	@bash -c "ag -l | entr -s 'rsync -avz --exclude .git --exclude target --exclude logs . discoursenet:~/builder/pp'"

rcheck:
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/app -w /usr/src/app rust:1.37 cargo check

.PHONY: \
	serve \
	serve.lint \
	check \
	check.lint \
	trigger \
	trigger.watch \
	sync \
	rcheck
