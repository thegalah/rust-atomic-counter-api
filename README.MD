# Rust Atomic Counter API

## Overview

The Rust Atomic Counter API is a highly performant and reliable web service designed for generating and managing atomic counters. Built in Rust and leveraging the Actix-web framework, this API is particularly suited for use cases requiring unique incremental build IDs, such as with Argo Workflows in CI/CD pipelines. Its atomic operations ensure thread safety and consistency across multiple instances, making it an ideal choice for distributed systems.

## Features

1. Atomic Incremental Counters: Provides atomic increment functionality to ensure unique, sequential numbering without race conditions.
1. Concurrency-safe: Built to handle concurrent requests efficiently, ensuring reliable counter updates even under high load.
1. Lightweight & Fast: Rust's efficiency combined with Actix-web's performance offers a minimal overhead solution.
1. Easy Integration: Designed to be easily integrated into Argo Workflows or any system requiring unique build IDs.
1. Data Persistence: Counter values are persisted to a file, enabling the service to resume from the last known count after a restart.

## Configuring the Service

### Environment Variables

**PORT:** Sets the port number on which the API server listens. If not specified, the server defaults to port 9000.

## API Endpoints

### GET /health

**Description:** Verifies API server availability.
Responses:

1. 200 OK: Server operational.
1. 500 Internal Server Error: Server issue.

### GET /liveness

**Description:** Checks server liveness for orchestrators.
Responses:

1. 200 OK: Server responsive.
1. 500 Internal Server Error: Server unresponsive.

### GET /counter

**Description:** Fetches the current atomic counter value.
Responses:

1. 200 OK: counter_value.
1. 500 Internal Server Error: Error fetching value.

## Data Persistence

The API ensures the continuity of counter values across service restarts by writing the current counter value to a file named counter.txt located in the same directory as the executable. This file is read at startup to initialize the counter with the last saved value, allowing the service to seamlessly resume its operation without resetting the counter to zero.

## Deploying in Kubernetes

Here's how you would deploy this in kubernetes:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-atomic-counter-api
spec:
  replicas: 1 # Recommended to be 1 if sharing state via a file to avoid write conflicts
  selector:
    matchLabels:
      app: rust-atomic-counter-api
  template:
    metadata:
      labels:
        app: rust-atomic-counter-api
    spec:
      containers:
        - name: rust-atomic-counter-api
          image: docker.io/thegalah/rust-atomic-counter-api
          ports:
            - containerPort: 9000
          volumeMounts:
            - name: counter-storage
              mountPath: /data
      volumes:
        - name: counter-storage
          persistentVolumeClaim:
            claimName: counter-pvc
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: counter-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Mi
```
