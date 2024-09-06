#!/bin/bash

#docker build -t pacebe:test -f ./docker/server.Dockerfile ./server --target runner --progress plain
docker build -t pacebe:test -f ./docker/server.Dockerfile ./server
