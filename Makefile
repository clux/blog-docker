NAME=blog
VERSION=$(shell git rev-parse HEAD)
SEMVER_VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=clux

build:
	docker run --rm -v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust \
		cargo build --release
	docker build -t $(REPO)/$(NAME):$(VERSION) .

stop:
	(docker rm -f $(NAME) 2> /dev/null) || true

start: stop
	docker run -p 8000:80 -d --name=$(NAME) -t $(REPO)/$(NAME)
	curl -sSL 0.0.0.0:8000 | grep "Posts"
	@echo "http://0.0.0.0:8000"

tag-latest:
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):latest
	docker push $(REPO)/$(NAME):latest

tag-semver:
	if curl -sSL https://registry.hub.docker.com/v1/repositories/$(REPO)/$(NAME)/tags | jq -r ".[].name" | grep -q $(SEMVER_VERSION); then \
		echo "Tag $(SEMVER_VERSION) already exists" && exit 1 ;\
	fi
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):$(SEMVER_VERSION)
	docker push $(REPO)/$(NAME):$(SEMVER_VERSION)
