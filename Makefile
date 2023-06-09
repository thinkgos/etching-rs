# include .env
include temp/.env

seaql-entity:
	@sea-orm-cli generate entity \
    -u ${database_url} \
	--with-serde both \
    -o entity/src/entities

local:
	@env APP_DEPLOY_MODE=local cargo run --example config 