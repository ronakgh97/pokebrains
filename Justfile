#!/usr/bin/env just --justfile

run_test:
    cargo nextest run --test team_parse --test battle_event

run_tool_test:
    cargo nextest run  test_agent_tool_stream --no-capture

bin:
    cargo run --bin bin -- arg1
