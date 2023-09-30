#!/bin/bash
set -e

psql school_finder < /sql_scirpts/schema.sql
psql -U postgres -d school_finder -a -f /sql_scirpts/schema.sql
