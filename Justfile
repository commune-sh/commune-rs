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
