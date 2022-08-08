# Generate the client
.PHONY: regenerate
regenerate:
	pip install -r scripts/requirements.txt
	scripts/regenerate.sh
