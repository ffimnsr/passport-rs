default: serve

serve:
	systemfd --no-pid -s http::5000 -- cargo watch -w src -x run

trigger.serve:
	systemfd --no-pid -s http::5000 -- cargo watch --no-vcs-ignores -w .trigger -x run

run.lint:
	systemfd --no-pid -s http::5000 -- cargo watch -w src -x clippy -x run

check:
	cargo watch -x check

check.lint:
	cargo watch -x clippy -x check

clippy:
	cargo clippy --all-features --all-targets --tests --benches -- -Dclippy::all

trigger:
	cargo watch --no-vcs-ignores -w src -s 'touch .trigger' -x check

sync:
	bash -c "ag -l | entr -s 'rsync -avz --exclude .git --exclude target --exclude logs . discoursenet:~/builder/pp'"

docker.check:
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/app -w /usr/src/app rust:1.37 cargo check

init-git:
	git config --local core.hooksPath .githooks/

.PHONY: \
	serve \
	serve.lint \
	check \
	check.lint \
	clippy \
	trigger \
	trigger.watch \
	sync \
	rcheck
