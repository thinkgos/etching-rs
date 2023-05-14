# include .env
include temp/.env

seaql-entity:
	sea-orm-cli generate entity \
    -u ${database_url} \
    -o entity/src/entities