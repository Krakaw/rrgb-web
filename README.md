# rrgb-web
A web service in front of a ws281x RGB panel

Very early stages of a web server in front of a WS2812 RGB panel.

The panel type has been hard coded for now. 

Compiling on a Pi Zero works, cross-compiling not so much.

## Usage
```
# Server
cargo run
```

```
# Client
# Set the first LED to white
curl -X POST -d '{"values": {"0": [255,255,255,0]}}' localhost:8090
# Maintain the current LEDs and set the second to red
curl -X PATCH -d '{"values": {"1": [255,0,0,0]}}' localhost:8090
# Reset all
curl -X DELETE localhost:8090
# Use GET request to set single LED (without resetting others)
# Set the first LED to green
curl -X GET localhost:8090/0/0/255/0/0
```

