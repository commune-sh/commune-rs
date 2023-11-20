set positional-arguments

# Lists all available commands
default:
  just --list

# Creates the `.env` file if it doesn't exist
dotenv:
  cp -n .env.example .env || true

# Generates the synapse configuration file and saves it
gen_synapse_conf: dotenv
  docker run -it --rm \
    -v ./docker/synapse:/data \
    --env-file .env \
    matrixdotorg/synapse:v1.96.1 generate

# Generates a de-facto admin user
gen_synapse_admin: dotenv
  docker compose exec -it synapse \
    register_new_matrix_user http://localhost:8008 \
    -c /data/homeserver.yaml \
    -u admin \
    -p admin \
    -a

# Runs backend dependency services
backend: dotenv
  docker compose up --build

# Stops backend dependency services
stop:
  docker compose down

# Removes oll Docker related config, volumes and containers for this project
clear: stop
  docker compose rm --all --force --volumes --stop
  docker volume rm commune_synapse_database || true

# Runs all the tests from the `test` package. Optionally runs a single one if name pattern is provided
e2e *args='':
  cargo test --package test -- --test-threads=1 $1
