#!/bin/bash
#Run docker containers for money-map application

#remove any existing containers
docker rm money-map-db
docker rm money-map-api

#run money map DB container
docker run -d -p 28017:27017 -v $(pwd)/mongo/data/db:/data/db --name money-map-db money-map-db

#run money map api container
docker run -d -p 8080:6767 -v $(pwd):/source --name money-map-api --link money-map-db:money-map-db1 money-map-api
