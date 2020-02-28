# xiaomi-1080-360-motor-control

Provides a way controlling the motor directly on the [MJSXJ02CM camera](https://www.mi.com/global/camera-360).

## Developement

1. Is it the same as mine?

[![MJSXJ02CM camera](https://i.imgur.com/3fOE6ZR.png)](https://www.mi.com/global/camera-360)

2. Get a shell

Install these mods to get a shell from within your camera:  
https://github.com/telmomarques/xiaomi-360-1080p-hacks

3. Install the toolchain

```shell
 # sudo apt-get install gcc-arm-linux-gnueabihf binutils-arm-linux-gnueabi
 # curl https://sh.rustup.rs -sSf | sh
```

4. Clone the repository
```
 $ git clone https://github.com/thewh1teagle/xiaomi-1080-360-motor-control.git
```

6. Kitchen is ready

```shell
make
```

```
make run -- pan 1 10
# or
./control-debug ./mocks/libdevice_kit.so pan 1 10
```


## Usage

```
 $ make release
```

On ARMv7 device:

```
 $ ./control /path/to/libdevice_kit.so {pan, tilt} direction steps
```
