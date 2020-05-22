# xiaomi-1080-360-motor-control

Provides a way controlling the motor directly on the [MJSXJ02CM camera](https://www.mi.com/global/camera-360).



## Requirements
arm-linux-gcc 

[go compiler](https://golang.org)



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


5. clonning the repo
```git clone
 git clone https://github.com/thewh1teagle/xiaomi-1080-360-motor-control.git
```

6. install go requirements and cross compile web-server

```shell
go get -u github.com/gorilla/mux
cd web-server
env GOOS=linux GOARCH=arm go build
```

7. build motor binary

```shell
cd motor
make
```

8. put web-server and motor in the same directory. now you can use them.
enjoy!

