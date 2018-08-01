The server must be configured using the `config.ini` file in the root directory. A brief description of all the supported options is as follows.

| Variable      | Description   |
| ------------- |:------------- |
|`driver`       | The database backend to use for the API. Possible values are `oracle` and `postgres` |
|`host`         | The address at which the database is available |
|`port`         | The port at the which the database is available  |
|`name`         | The service name or the name of database to use for connection |
|`user`         | The username to use for connection |
|`password`     | The password to use for connection |

Example `config.ini` file.

```
[DEFAULT]
driver = oracle

[ORACLE]
host = oracle_database_address
port = 1234
service-name = service.name
user = user_name
password = password

[POSTGRES]
host = postgres_database_address
port = 1234
database-name = db_name
user = username
password = password
```