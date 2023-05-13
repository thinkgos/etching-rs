# database_url="mysql://root:123456@127.0.0.1:3306/dev"

seaql-entity:
	sea-orm-cli generate entity \
    -u ${database_url} \
    -o entity/src/entities