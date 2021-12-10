#!/bin/bash

export RUST_BACKTRACE=1
export LEDGER_URL=http://test.bcovrin.vonx.io
export CLOUD_AGENCY_URL=https://ariesvcx.agency.staging.absa.id

target/debug/rufiber
