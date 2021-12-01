#!/bin/bash

export RUST_BACKTRACE=1
export LEDGER_URL=http://test.bcovrin.vonx.io
export CLOUD_AGENCY_URL=https://ariesvcx.agency.staging.absa.id
#export CLOUD_AGENCY_URL= https://8410-34-64-123-53.ngrok.io

target/debug/alice
