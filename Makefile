# Generate the client
.PHONY: regenerate
regenerate:
	pip3 install -r scripts/requirements.txt
	scripts/regenerate.sh
