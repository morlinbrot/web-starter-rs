default:
    just -l -u

db:
    docker run -d \
        --rm \
        --name web-starter-db \
        -p 5432:5432 \
        --env-file .env \
        postgres:17-alpine

stop:
    docker stop web-starter-db

ps *args:
    docker ps {{args}}

psql:
    docker exec -it -u postgres web-starter-db psql

mig-add name:
    sqlx migrate add {{name}}

mig-up:
    sqlx migrate run

mig-down:
    sqlx migrate revert

mig-info:
    sqlx migrate info

run:
    cargo run

watch cmd="run":
    cargo watch -x {{cmd}}

test:
    cargo test
