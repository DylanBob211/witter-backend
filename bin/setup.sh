#!/bin/bash
set -e

dropdb witter || true
dropdb witter-test || true

createdb witter || true
createdb witter-test || true

psql -d witter < bin/setup.sql
psql -d witter-test < bin/setup.sql

