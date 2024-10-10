# Dockit

Dockit is cli app for manage docker-compose files

## Usage 
### ```dockit init```

```yaml
version: '3'
services: {}
volumes: {}
networks: {}
```

### ```dockit add service --name db -i surrealdb/surrealdb:latest -p 80:8000```

```yaml
version: '3'
services:
  db:
    image: surrealdb/surrealdb:latest
    container_name: db
    ports:
    - 80:8000
volumes: {}
networks: {}
```

### ```dockit add network -n frontend -e my_net```

```yaml
version: '3'
services:
  db:
    image: surrealdb/surrealdb:latest
    container_name: db
    ports:
    - 80:8000
volumes: {}
networks:
  frontend:
    external:
      name: my_net
```

### ```dockit add volume -n pg_volume -e data```

```yaml
version: '3'
services:
  db:
    image: surrealdb/surrealdb:latest
    container_name: db
    ports:
    - 80:8000
volumes:
  pg_volume:
    external:
      name: data
networks:
  frontend:
    external:
      name: my_net
```

## Installation

* ```cargo install dockit```

## License
* [LICENSE-APACHE](LICENSE-APACHE)