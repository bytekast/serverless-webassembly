# Serverless WebAssembly Template

This is a simple Serverless application written in Rust, compiled to WebAssembly and deployed to AWS Lambda using the Serverless Framework.

## 📦 Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install).

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Install the Rust => WebAssembly compiler: [wasm-pack](https://github.com/rustwasm/wasm-pack)

`cargo install wasm-pack`

Install [Node](https://www.npmjs.com/get-npm).

Install the [Serverless Framework](https://serverless.com/framework/).

`npm install -g serverless`

## 🏗️ Create

```
sls create --template-url https://github.com/bytekast/serverless-webassembly --path my-service
cd my-service
```

## 🦀 Build

`npm install`

## 🔫 Run Locally

`npm run local`

## 🛵 Deploy

`sls deploy`

```bash
Serverless: Packaging service...
Serverless: Excluding development dependencies...
Serverless: Creating Stack...
Serverless: Checking Stack create progress...
.....
Serverless: Stack create finished...
Serverless: Uploading CloudFormation file to S3...
Serverless: Uploading artifacts...
Serverless: Uploading service my-service.zip file to S3 (124.08 KB)...
Serverless: Validating template...
Serverless: Updating Stack...
Serverless: Checking Stack update progress...
..............................
Serverless: Stack update finished...
Service Information
service: my-service
stage: dev
region: us-east-1
stack: my-service-dev
resources: 10
api keys:
  None
endpoints:
  GET - https://xxxxxx.execute-api.us-east-1.amazonaws.com/dev/hello
functions:
  hello: my-service-dev-hello
layers:
  None
Serverless: Run the "serverless" command to setup monitoring, troubleshooting and testing.
```