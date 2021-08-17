docker run -d -p 8086:8086 \
      -v $PWD/data:/var/lib/influxdb2 \
      -v $PWD/config:/etc/influxdb2 \
      -e DOCKER_INFLUXDB_INIT_MODE=setup \
      -e DOCKER_INFLUXDB_INIT_USERNAME=ockam \
      -e DOCKER_INFLUXDB_INIT_PASSWORD=ockamockam\
      -e DOCKER_INFLUXDB_INIT_ORG=ockam \
      -e DOCKER_INFLUXDB_INIT_BUCKET=ockam-bucket \
      influxdb:2.0
