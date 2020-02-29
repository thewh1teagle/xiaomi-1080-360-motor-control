# xiaomi-1080-360-motor-control

Provides a way controlling the motor directly on the [MJSXJ02CM camera](https://www.mi.com/global/camera-360).

## Getting started

1. Is it the same as mine?

[![MJSXJ02CM camera](https://i.imgur.com/3fOE6ZR.png)](https://www.mi.com/global/camera-360)

2. Get a shell

Install these mods to get a shell from within your camera:  
https://github.com/telmomarques/xiaomi-360-1080p-hacks

3. Install the toolchain

```shell
 sudo apt-get install gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross binutils-arm-linux-gnueabi
```

4. Bring your own libs (optional)

If you want to use your own libs, you can get it from the camera using [`pull_libs`](./pull_libs). 


5. clonning from master branch (stable)
```git clone
 git clone -b master https://github.com/thewh1teagle/xiaomi-1080-360-motor-control.git
```



6. Kitchen is ready

```shell
make
```

