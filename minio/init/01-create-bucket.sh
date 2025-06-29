#!/bin/sh
set -e

mc alias set ums http://minio:9000 "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD"

mc mb --ignore-existing ums/profile-photo
