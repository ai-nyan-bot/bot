SERVER_HOST = one
REMOTE_DIR = ~/nyanbot

.PHONY: all
all: check test test-frontend build push migrate deploy

.PHONY: check
check:
	@if ! git diff-index --quiet HEAD --; then \
		echo "Error: You have uncommitted changes. Please commit or stash them before pushing."; \
		exit 1; \
	fi

.PHONY: test
test:
	cargo test --lib --bins --tests

.PHONY: test-frontend
test-frontend:
	cd bin/frontend &&	pnpm build && pnpm test


.PHONY: build
build:
	docker build . -t nyanbot/nyanbot

.PHONY: push
push: check
	git push
	docker push nyanbot/nyanbot

.PHONY: migrate
migrate: check
	@echo "Migrating database of $(SERVER_HOST)..."
	ssh $(SERVER_HOST) 'cd $(REMOTE_DIR)/repo && \
		git pull --rebase && \
		~/.cargo/bin/sqlx migrate run'
	@echo "Migration completed"

.PHONY: deploy
deploy: migrate
	@echo "Deploying Docker container to $(SERVER_HOST)..."
	ssh $(SERVER_HOST) 'cd $(REMOTE_DIR) && \
		docker compose pull && \
		docker compose down && \
		docker compose rm -f && \
		docker compose up -d'
	@echo "Deployment complete."

