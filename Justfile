set positional-arguments

commit_sha := `git rev-parse --verify --short=7 HEAD`
target_release := "x86_64-unknown-linux-musl"
docker_user := `echo "$(id -u):$(id -g)"`

# Lists all available commands
default:
  just --list

# Creates the `.env` file if it doesn't exist
dotenv:
  cp -n .env.example .env || true
  mkdir -p docker/synapse || true

# Dump database to a file
backup_db:
  DOCKER_USER={{docker_user}} docker compose exec -T synapse_database \
    pg_dumpall -c -U synapse_user > ./dump.sql

# Restore database from a file
restore_db:
  cat ./dump.sql | DOCKER_USER={{docker_user}} docker compose exec -T synapse_database \
    psql -U synapse_user -d synapse

# Nuke database
nuke_db:
    DOCKER_USER={{docker_user}} docker compose exec -T synapse_database \
    psql -U synapse_user -d synapse -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

# Generates the synapse configuration file and saves it
gen_synapse_conf: dotenv
  docker run -i --rm \
    -u {{docker_user}} \
    -v ./docker/synapse:/data \
    --env-file .env \
    matrixdotorg/synapse:v1.96.1 generate

# Generates a de-facto admin user
gen_synapse_admin: dotenv
  docker compose exec -i synapse \
    register_new_matrix_user http://localhost:8008 \
    -u {{docker_user}} \
    -c /data/homeserver.yaml \
    -u admin \
    -p admin \
    -a

# Retrieves admin access token uses de-facto admin user and Development Database Credentials
get_access_token:
  sed -i "s/COMMUNE_SYNAPSE_ADMIN_TOKEN='.*'/COMMUNE_SYNAPSE_ADMIN_TOKEN='$( \
    curl -sS -d '{"type":"m.login.password", "user":"admin", "password":"admin"}' \
    http://localhost:8008/_matrix/client/v3/login | jq --raw-output '.access_token' \
  )'/" .env

# Runs backend dependency services
backend: dotenv
  DOCKER_USER={{docker_user}} docker compose up --build

# Stops backend dependency services
stop:
  DOCKER_USER={{docker_user}} docker compose down

# Removes oll Docker related config, volumes and containers for this project
clear: stop
  DOCKER_USER={{docker_user}} docker compose rm --all --force --volumes --stop
  DOCKER_USER={{docker_user}} docker volume rm commune_synapse_database || true

# Runs all the tests from the `test` package. Optionally runs a single one if name pattern is provided
e2e *args='':
  cargo test --package test -- --nocapture --test-threads=1 $1

# Builds the Server binary used in the Docker Image
docker_build_server:
  cargo zigbuild --target {{target_release}} --release -p server

# Builds the Docker image for the backend
docker_build_image: docker_build_server
  mkdir tmp/
  cp ./target/{{target_release}}/release/server ./tmp/server
  chmod +x ./tmp/server
  docker build -t "commune:{{commit_sha}}-{{target_release}}" .

# Publishes the Docker image to the GitHub Container Registry
docker_publish_image:
  docker tag commune:{{commit_sha}}-{{target_release}} ghcr.io/commune-os/commune:{{commit_sha}}-{{target_release}}
  docker push ghcr.io/commune-os/commune:{{commit_sha}}-{{target_release}}
