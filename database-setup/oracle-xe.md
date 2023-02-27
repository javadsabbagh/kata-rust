## list docker images

```bash
docker  images
docker  images | grep oracle
```

## save docker images 
```bash
docker image save gvenzl/oracle-xe:21-slim-faststart > oracle-21.tar
```

## load docker image
```bash
docker image load < oracle-21.tar
```
## run oracle on docker

```bash
docker run -d \
   -p 1521:1521 \
   -e APP_USER=chk \
   -e APP_USER_PASSWORD=chktest \
   -e ORACLE_PASSWORD=simple_password \  
   --name oracle-21 \
   -v /home/javad/oracle-21-xe:/u01/app/oracle/oradata \   
   gvenzl/oracle-xe:21-slim-faststart
```

## stop oracle  
```bash
docker stop oracle-21 
```
## start oracle  
```bash
docker start oracle-21
```

## view oracle logs 
```bash
docker logs  oracle-21
```
## view oracle processes 
```bash
docker top  oracle-21
```
## List Running docker containers 
```bash
docker ps 
docker ps -a
```