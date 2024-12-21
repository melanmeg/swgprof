#!/bin/bash

SCRIPT_DIR="$HOME/.my/switch_cloud"

if [ "$MY_SET_CLOUD_BENDER" == "gcp" ]; then
    echo "This is GCP."
    source "$SCRIPT_DIR/gcp.sh"
elif [ "$MY_SET_CLOUD_BENDER" == "aws" ]; then
    echo "This is AWS."
elif [ "$MY_SET_CLOUD_BENDER" == "azure" ]; then
    echo "This is Azure."
else
    echo "Unknown environment."
fi
