##
##  Copyright 2021 neko.rs contributors <https://neko.rs>
##
##  Licensed with GNU Affero General Public License v3.0 or later
##

# Define global arguments
ARG PROJECT="nekobin"
ARG PORT="8080"
ARG NODE_ENV="production"

####################

FROM node:alpine as postcss

# Copy package.json and use it to install dependencies
COPY styles/package.json styles/package.json
WORKDIR styles/
RUN npm install

# Copy the rest of the needed files
COPY styles/postcss.config.js postcss.config.js
COPY styles/tailwind.config.js tailwind.config.js
COPY styles/src src
COPY templates ../templates

# Build the css file
ARG NODE_ENV
RUN NODE_ENV=$NODE_ENV npx postcss src/main.scss -o /styles.css

####################

FROM rust:slim-bullseye as build

# Update and install system build dependencies
RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y upgrade

RUN DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends libpq-dev && \
    DEBIAN_FRONTEND=noninteractive apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create an empty rust project
ARG PROJECT
RUN USER=root cargo new --bin ${PROJECT}
WORKDIR /${PROJECT}

# Precompile project dependencies
COPY Cargo.toml Cargo.toml
RUN cargo build --release

# Remove precompile garbage
RUN rm src/* target/release/deps/${PROJECT}*

# Build the actual project
COPY src src
COPY templates templates
RUN cargo build --release

####################

FROM debian:bullseye-slim

# Update and install system dependencies
RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y upgrade
    
RUN DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends libpq5 && \
    DEBIAN_FRONTEND=noninteractive apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set the main executable path
CMD ["./run"]

# Create the project directory and switch to it
ARG PROJECT
RUN mkdir ${PROJECT}
WORKDIR /${PROJECT}

# Expose the application port
ARG PORT
EXPOSE ${PORT}

# Copy the static assets
COPY assets assets/

# Copy the built css from the first stage
RUN mkdir -p assets/css
COPY --from=postcss /styles.css ./assets/css/styles.css 

# Copy the built binary from the second stage
COPY --from=build /${PROJECT}/target/release/${PROJECT} ./run
