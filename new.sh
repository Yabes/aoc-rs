#!/usr/bin/env bash

set -e
set -o pipefail

YEAR=$(date "+%Y")
DAY=$(date "+%d")

while test $# -gt 0; do
  case $1 in
  --year)
    shift
    YEAR=$1
    ;;

  --day)
    shift
    DAY=$1
    ;;

  *)
    echo "Unknown arg '${1}'"
    exit 1
    ;;
  esac

  shift
done

PUZZLE_DIRECTORY="${YEAR}/${DAY}"

if [ ! -d "${YEAR}" ]; then
  mkdir -p "${YEAR}"
fi

if [ -d "${PUZZLE_DIRECTORY}" ]; then
  echo "Directory '${PUZZLE_DIRECTORY}' already exists"
  exit 1
fi

cp -r ./template "${PUZZLE_DIRECTORY}"

find "${PUZZLE_DIRECTORY}" -type f -exec sed -i -e "s/{{year}}/${YEAR}/g" -e "s/{{day}}/${DAY}/g" {} \;
