# xiaomi-1080-360-motor-control

Provides a way controlling the motor directly on the [MJSXJ02CM camera](https://www.mi.com/global/camera-360).



## Requirements
arm-linux-gcc 




## Getting started

1. Is it the same as mine?

[![MJSXJ02CM camera](https://i.imgur.com/3fOE6ZR.png)](https://www.mi.com/global/camera-360)

2. Get a shell

Install these mods to get a shell from within your camera:  
https://github.com/telmomarques/xiaomi-360-1080p-hacks

3. Install the toolchain

```shell
 sudo apt-get install gcc-arm-linux-gnueabihf  
```


4. clonning the repo
```git clone
 git clone https://github.com/thewh1teagle/xiaomi-1080-360-motor-control.git
```

7. build motor binary

```shell
make TARGET=cross
```

you can also build it for your pc to test it without the camera.

```shell
make test
export "LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/"
./motord
```

8. upload the binary using sftp and run it.


for controlling the camera you will have event file
in the same directoty
by writing to the file you can control the camera

```shell
echo '<pan|tilt> <forward|reverse> <steps>' > event
```

You will also have status file. 
this file used to know if the motor is in max position
max = 1
normal = 0

enojoy!
