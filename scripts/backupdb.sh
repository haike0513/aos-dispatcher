#!/bin/bash
# BACKUP_DIR="$HOME/postgresql/data/backup"
BACKUP_DIR="/var/postgresql/data/backup"

# DIST_FILE="$BACKUP_DIR/db_data_"`date "+%Y-%m-%d_%H_%M_%S"`".bak"
DIST_FILE="$BACKUP_DIR/db_data_"`date "+%Y-%m-%d_%H_%M_%S"`".sql"

# echo $BACKUP_DIR;

# mkdir -p $BACKUP_DIR
# docker exec --user postgres dispatcher_postgres sh -c "pg_dump postgres > $DIST_FILE"
docker exec dispatcher_postgres sh -c "pg_dump -U postgres postgres > $DIST_FILE"

