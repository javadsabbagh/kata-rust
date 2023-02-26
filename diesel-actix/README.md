

## Running Yugabytedb
Yugabytedb is a Postgres compliant RDBMS with distributed sql nature. In this tutorial I'm going to use 
yugabytedb instead of postgres. 

## Running on docker
```bash
docker run -d --name yugabyte \
          -p7000:7000 -p9000:9000 -p5433:5433 -p9042:9042 \
          -v ~/yb_data:/home/yugabyte/yb_data \
          yugabytedb/yugabyte:latest bin/yugabyted start \
          --base_dir=/home/yugabyte/yb_data --daemon=false
```

This runs yugabytedb with persistent docker volume for storing data.

## Running ysqlsh client

Using docker:
```bash
docker run -it yugabytedb/yugabyte-client ysqlsh -h <remote-server-ip> -p 5433
```

Using a shell script:

Use either curl 
```bash
$ curl -sSL https://downloads.yugabyte.com/get_clients.sh | bash
```
or wget

```bash
wget -q -O - https://downloads.yugabyte.com/get_clients.sh | sh
```

