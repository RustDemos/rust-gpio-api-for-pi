# GPIO API for RaspberryPi

## start
```
cargo run
```

eg: set gpio 26 (pin 37) off

```
curl --location --request POST 'localhost:7789/gpio' \
--header 'Cache-Control: no-cache' \
--header 'Accept: */*' \
--header 'Accept-Encoding: gzip, deflate' \
--header 'Connection: keep-alive' \
--data-raw '{
    "pin":26,
    "value":false
}'
```

eg: set gpio 26 (pin 37) on

```
curl --location --request POST 'localhost:7789/gpio' \
--header 'Cache-Control: no-cache' \
--header 'Accept: */*' \
--header 'Accept-Encoding: gzip, deflate' \
--header 'Connection: keep-alive' \
--data-raw '{
    "pin":26,
    "value":true
}'
```


![](./pinout.png)