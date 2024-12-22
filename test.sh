#!/bin/bash

export MY_SET_CLOUD_BENDER="gcp"
export MY_GCP_PROFILE_NAME="test-project"
export MY_GCP_ACCOUNT_NAME="my-user@gcp.org.melanmeg.com"
export MY_GCP_PROJECT_ID="test-project-373118"
export MY_GCP_REGION="asia-northeast1"
export MY_GCP_ZONE="asia-northeast1-a"

cargo run -q

# gcloud config configurations list
# gcloud auth revoke my-user@gcp.org.melanmeg.com --quiet
# gcloud config configurations activate default --quiet
# gcloud config configurations delete test-project --quiet
# rm -f ~/.config/gcloud/application_default_credentials.json
# rm -f ~/.config/gcloud/tmp_credentials/test-project.json
# gcloud storage buckets list
