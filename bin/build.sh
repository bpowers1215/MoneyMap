#!/bin/bash

#Build Docker images

#build money-map-api image
docker build -t money-map-api .

#build money map DB image
docker build -t money-map-db ./mongo
