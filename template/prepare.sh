#!/bin/bash
CMD_NAME=`basename $0`
TEMPLATE_DIR=$(cd $(dirname $0); pwd)
TARGET_DIR="."
CONTEST_NAME=""

echo "XXX"
echo ${TEMPLATE_DIR}
echo "YYY"


if [ $# -lt 1 ]; then
	echo "Usage: ${CMD_NAME} <project> [<dir>]" 1>&2
	exit 1
fi
CONTEST_NAME=$1

if [ $# -gt 1 ]; then
	TARGET_DIR=$2
fi

pushd ${TARGET_DIR}

if [ -e ${CONTEST_NAME} ]; then
	echo "Error! ${CONTEST_NAME} already exists."
	exit 1
fi
mkdir ${CONTEST_NAME}

pushd ${CONTEST_NAME}

for q in a b c d e f
do
	cargo new ${q}
	cp ${TEMPLATE_DIR}/main.rs ${CONTEST_NAME}_${q}/src/
done

popd
popd

