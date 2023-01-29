# Needed because we have folders called "docs" and "test" that confuse `make`.
.PHONY: docs test py-venv-check clean

.EXPORT_ALL_VARIABLES:

# Git and docker params
GITCOMMIT=$(shell git rev-parse --short HEAD)
GITBRANCH=$(shell git rev-parse --abbrev-ref --short HEAD)
VIDYUT_PY_VERSION=v0.1
VIDYUT_PY_NAME=vidyut-py
VIDYUT_PY_IMAGE=${VIDYUT_PY_NAME}:${VIDYUT_PY_VERSION}-${GITBRANCH}-${GITCOMMIT}
VIDYUT_PY_IMAGE_LATEST="$(VIDYUT_PY_NAME)-rel:latest"
VIDYUT_VER=0.2.0

# Checks that the caller is in a virtual environment.
py-venv-check: 
ifeq ("$(VIRTUAL_ENV)","")
	@echo "Error: You are not running within a virtualenv. Please run the following command:"
	@echo
	@echo "> source env/bin/activate"
	@echo
	exit 1
endif

# Initializes the repository for local development.
install:
	python3 -m venv env
	. env/bin/activate && pip install -r requirements.txt
	. env/bin/activate && make develop
	. env/bin/activate && pytest
	@echo
	@echo "vidyut-py built successfully."

# Builds the package.
#
# Use this command directly instead of `cargo build`, which will likely fail
# due to missing linker arguments.
build: py-venv-check
	maturin build

# Builds the package and installs it in the local virtualenv.
develop: py-venv-check
	maturin develop

# Runs all Python-specific unit tests.
test: develop
	pytest test/unit/

# Runs integration tests against a data directory.
#
# Usage:
#
# VIDYUT_DATA_DIR="/path/to/your/dir" make integration_tests
integration_tests: develop
	pytest test/integration/

# Lints all Rust and Python code.
lint:
	cargo fmt
	black .

# Generates Sphinx docs.
#
# To view the docs, open `vidyut/docs/build/html/index.html`.
docs: develop
	cd vidyut/docs && make html

# Docker commands
# Build docker image. All tag the latest to the most react image
# docker-build: lint-check
docker-build: 
	@echo "> Vidyut-py build is in progress. Expect it to take 2-5 minutes."
	@printf "%0.s-" {1..21} && echo
	@docker build -t ${VIDYUT_PY_IMAGE} -t ${VIDYUT_PY_IMAGE_LATEST} --build-arg VIDYUT_VER=${VIDYUT_VER} -f build/containers/Dockerfile ${PWD} 
	@echo "Vidyut-py Image    : ✔ (${VIDYUT_PY_IMAGE}, ${VIDYUT_PY_IMAGE_LATEST})"
