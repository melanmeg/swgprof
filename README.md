# switch gcloud profile

## 前提

- miseがインストールされている

## インストール

```bash
PACKAGE_NAME=swgprof
sudo curl -Lo "$HOME/.local/bin/$PACKAGE_NAME" https://github.com/melanmeg/swgprof/releases/download/v1.0/swgprof-1.0-x86_64-unknown-linux-musl
sudo chown "$USER:$USER" "$HOME/.local/bin/$PACKAGE_NAME"
sudo chmod +x "$HOME/.local/bin/$PACKAGE_NAME"
```

- armはこちら `https://github.com/melanmeg/swgprof/releases/download/v1.0/swgprof-1.0-aarch64-unknown-linux-musl`

## 使い方

- .mise.local.toml

```bash
[env]
MY_SET_CLOUD_BENDER = "gcp"
MY_GCP_PROFILE_NAME = "my-project"
MY_GCP_ACCOUNT_NAME = "my-user@gcp.org.melanmeg.com"
MY_GCP_PROJECT_ID   = "my-project-melanmeg"
MY_GCP_REGION       = "asia-northeast1"
MY_GCP_ZONE         = "asia-northeast1-a"

[hooks.enter]
shell  = "bash"
script = "swgprof"
```

- swgprof/へ移動する

```bash
$ cd swgprof/
Enter Project: test-project-373118
Check configure gcloud.
 Account: my-user@gcp.org.melanmeg.com is not active.

Please login to gcloud.
Go to the following link in your browser, and complete the sign-in prompts:

    https://accounts.google.com/o/oauth2/auth?response_type=code&client_id=32555940559.apps.googleusercontent.com&redirect_uri=https%3A%2F%2Fsdk.cloud.google.com%2Fauthcode.html&scope=openid+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcloud-platform+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fappengine.admin+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fsqlservice.login+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcompute+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Faccounts.reauth&state=Y3WRarpnvMCPQ46beWGWyhgicRBhjm&prompt=consent&token_usage=remote&access_type=offline&code_challenge=FkZmXvjTN24OZXP-5ocOhb7rD1xWmBrGpD_L6lgBoXY&code_challenge_method=S256

Once finished, enter the verification code provided in your browser: 4/0AanRRrsH6KHaKUpivtSZ2EsIG5VvzAQ7THVHafggtWqCCdHuAQmCQx3Ywj3id7nH2renoQ

You are now logged in as [my-user@gcp.org.melanmeg.com].
Your current project is [my-project-melanmeg].  You can change this setting by running:
  $ gcloud config set project PROJECT_ID

 File /home/melanmeg/.config/gcloud/tmp_credentials/test-project.json does not exist.

Please application login to gcloud.
Go to the following link in your browser, and complete the sign-in prompts:

    https://accounts.google.com/o/oauth2/auth?response_type=code&client_id=764086051850-6qr4p6gpi6hn506pt8ejuq83di341hur.apps.googleusercontent.com&redirect_uri=https%3A%2F%2Fsdk.cloud.google.com%2Fapplicationdefaultauthcode.html&scope=openid+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcloud-platform+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fsqlservice.login&state=NXlPBEfZx7zrn5JuvUgWvfvxGjFkOd&prompt=consent&token_usage=remote&access_type=offline&code_challenge=PfuAfCIXqTFhoIxVJM1HdvHNpQ37PjQyfIjfFmSyk6E&code_challenge_method=S256

Once finished, enter the verification code provided in your browser: 4/0AanRRrsoAhslGkrvvn2O5YWKGP3ZAXCnOK7vGjoVBm2uUeMrOb18smJMSfZGMH5-YN0X6g

Credentials saved to file: [/home/melanmeg/.config/gcloud/application_default_credentials.json]

These credentials will be used by any library that requests Application Default Credentials (ADC).

Quota project "my-project-melanmeg" was added to ADC which can be used by Google client libraries for billing and quota. Note that some services may still bill the project owning the resource.

 Profile test-project not found.
 Setting profile test-project.
 Profile test-project is active.
 Application login is ok.
 Login session is ok.

Done.
```

- 再度swgprof/へ移動する

```bash
$ cd swgprof/
Enter Project: test-project-373118
Check configure gcloud.
 Account my-user@gcp.org.melanmeg.com is active.
 File /home/melanmeg/.config/gcloud/tmp_credentials/test-project.json exists.
 Profile test-project found.
 Profile test-project is active.
 Application login is ok.
 Login session is ok.

Done.
```

## 仕組み

- `~/.config/gcloud/application_default_credentials.json` ファイルをバックアップしてプロファイル名ごとに切り替わるようにしている
- ※Googleアカウントのパスワードポリシーなどによっては上手く動作しない可能性あり。

## 開発

```bash
# - Setup
sudo apt install -y build-essential # pre-setting
cargo run -q # simple run

# - Release
# 古いglibでも動くように配布する
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
cp -a ./target/x86_64-unknown-linux-musl/release/swgprof ./
# arm用作成
sudo apt install -y gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-musl
RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release --target=aarch64-unknown-linux-musl
cp -a ./target/aarch64-unknown-linux-musl/release/swgprof ./
```
