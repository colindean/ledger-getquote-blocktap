
deps:
	cargo install cargo-ensure-installed

refresh-schema:
	cargo ensure-installed --package=graphql_client_cli --version= && \
		graphql-client introspect-schema https://api.blocktap.io/graphql --output schema/blocktap.json
