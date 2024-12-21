#!/bin/bash

export MY_SET_CLOUD_BENDER="gcp"
export MY_GCP_PROFILE_NAME="test-project"
export MY_GCP_ACCOUNT_NAME="my-user@gcp.org.melanmeg.com"
export MY_GCP_PROJECT_ID="test-project-373118"
export MY_GCP_REGION="asia-northeast1"
export MY_GCP_ZONE="asia-northeast1-a"

cargo run -q
