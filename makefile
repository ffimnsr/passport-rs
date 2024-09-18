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

trigger:
	cargo watch --no-vcs-ignores -w src -s 'touch .trigger' -x check

sync:
	bash -c "ag -l | entr -s 'rsync -avz --exclude .git --exclude target --exclude logs . discoursenet:~/builder/pp'"

docker.check:
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
