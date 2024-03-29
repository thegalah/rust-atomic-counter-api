# Rust Atomic Counter API

## Overview

The Rust Atomic Counter API is a highly performant and reliable web service designed for generating and managing atomic counters. Built in Rust and leveraging the Actix-web framework, this API is particularly suited for use cases requiring unique incremental build IDs, such as with Argo Workflows in CI/CD pipelines. Its atomic operations ensure thread safety and consistency across multiple instances, making it an ideal choice for distributed systems.

## Features

1. Atomic Incremental Counters: Provides atomic increment functionality to ensure unique, sequential numbering without race conditions.
1. Concurrency-safe: Built to handle concurrent requests efficiently, ensuring reliable counter updates even under high load.
1. Lightweight & Fast: Rust's efficiency combined with Actix-web's performance offers a minimal overhead solution.
1. Easy Integration: Designed to be easily integrated into Argo Workflows or any system requiring unique build IDs.

## API Endpoints

### GET /health

**Description:** Verifies API server availability.
Responses:

1. 200 OK: Server operational.
1. 500 Internal Server Error: Server issue.
   ###GET /liveness
   **Description:** Checks server liveness for orchestrators.
   Responses:

1. 200 OK: Server responsive.
1. 500 Internal Server Error: Server unresponsive.

### GET /counter

**Description:** Fetches the current atomic counter value.
Responses:

1. 200 OK: Returns {"counter": value}.
1. 500 Internal Server Error: Error fetching value.
