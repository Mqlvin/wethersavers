#!bin/bash

set -e

echo '[wethersavers] building static frontend files'
cd ./frontend
npm i --legacy-peer-deps
npm run build
echo '[wethersavers] frontend built'

cd ../backend

rm -rf frontend-build
cp -r ../frontend/build frontend-build
echo '[wethersavers] frontend files copied to stage'

echo '[wethersavers] building main image'
rm -f webserver.tar && podman build -t backend:glibc-bullseye -f Dockerfile && podman save localhost/backend:glibc-bullseye -o webserver.tar
echo '[wethersavers] main image built'
mv ./webserver.tar ../prod.tar
echo ''
echo '[wethersavers] to kill/remove , "docker kill wethersavers; docker container rm wethersavers"'
echo '[wethersavers] on the server, "docker load -i prod.tar && docker run -p 3000:3000 -v wethersavers-data:/data --name wethersavers -d --restart unless-stopped localhost/backend:glibc-bullseye"'
