#!/bin/bash

function clone() {
  # go to directory proto
  cd oes-protobuf
  # change to specific tag when the branch is merged   
  git clone -b v0.0.18 git@gitlab.com:oesinc/oes-protobuf.git --single-branch
  cd ..
}

# check if directory exists
if [ -d "oes-protobuf" ]; then
  echo "Directory oes-protobuf exists."
  # confirm to delete directory
  read -p "Do you want to delete directory oes-protobuf? (y/n) " -n 1 -r
  # if yes, delete directory
  if [[ $REPLY =~ ^[Yy]$ ]]
  then
    echo "Removing directory oes-protobuf."
    rm -rf oes-protobuf
    echo "Creating directory oes-protobuf."
    mkdir oes-protobuf
    clone
  fi  
else
  echo "Creating directory oes-protobuf."
  mkdir oes-protobuf
  clone
fi

mkdir -p protos
mkdir -p protos/event
cp oes-protobuf/oes-protobuf/opendso/historian/historian.proto ./protos/historian.proto
cp oes-protobuf/oes-protobuf/opendso/event/cloudevents.proto ./protos/event/cloudevents.proto

# delete oes-protobuf directory
rm -rf oes-protobuf

if [ ! -d "openfmb-ops-protobuf" ]
then
    OPENFMB_VERSION="v2.1.0"
    git clone -b $OPENFMB_VERSION https://gitlab.com/openfmb/psm/ops/protobuf/openfmb-ops-protobuf
fi

cp -r openfmb-ops-protobuf/proto/openfmb/* ./protos/

