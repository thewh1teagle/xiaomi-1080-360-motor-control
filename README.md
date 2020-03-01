# xiaomi-1080-360-motor-control

## Developement

1. Is this compatible with my camera?

This controller works with the [MJSXJ02CM camera](https://www.mi.com/global/camera-360):

[![MJSXJ02CM camera](https://i.imgur.com/3fOE6ZR.png)](https://www.mi.com/global/camera-360)

2. Get a shell

Install these mods to get a shell from within your camera:  
https://github.com/telmomarques/xiaomi-360-1080p-hacks

3. Install the toolchain

```shell
 $ sudo apt-get install gcc-arm-linux-gnueabihf binutils-arm-linux-gnueabi
 $ curl https://sh.rustup.rs -sSf | sh
 $ rustup install stable
```

4. Clone the repository
```
 $ git clone https://github.com/thewh1teagle/xiaomi-1080-360-motor-control.git
```

5. Kitchen is ready

```shell
 $ make
```

```shell
make run -- motor move pan forward 10
```

```shell
export MIJIA_LIB_PATH=$(pwd)/mocks
./control-debug motor move pan forward 10
```

6. Build release binary

```shell
make release
```

## Usage

```shell
export MIJIA_LIB_PATH=/mnt/data/lib/libdevice_kit.so
./control motor move pan forward 10
./control motor stop
./control server --listen 0.0.0.0:8080
```
