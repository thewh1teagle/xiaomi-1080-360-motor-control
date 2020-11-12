# xiaomi-1080-360-motor-control

Provides a way controlling the motor directly on the [MJSXJ02CM camera](https://www.mi.com/global/camera-360).


## Getting started

1. Is it the same as mine?

[![MJSXJ02CM camera](https://i.imgur.com/3fOE6ZR.png)](https://www.mi.com/global/camera-360)


2. Get a shell

Install these mods to get a shell from within your camera:  
https://github.com/telmomarques/xiaomi-360-1080p-hacks





## Install the hacks
1. Download the latest release from [releases](https://github.com/thewh1teagle/xiaomi-1080-360-motor-control/releases)

2. Copy the contents of "hacks" folder to the hacks folder in your SD Card

3. Power off the camera and insert the SD Card
4. Power on the camera
5. Open the web interface on the camrea on your browser
http:8080//<your-camera-ip/

## How to build

1. Install docker
```shell
 sudo apt-get install -y docker.io
```


2. Clonning the repo
```git clone
 git clone https://github.com/thewh1teagle/xiaomi-1080-360-motor-control.git
```

3. Build the container 
```shell
sudo docker build -t motor .
```
4. Run docker container with current directory mounted
```shell
sudo docker run -it -v $(pwd):/src motor
```

5. Cross compile (in every different module e.g web_control)

```shell
make cross
```

You can also build it for your pc to test it without the camera.

```shell
make test
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd) ./motord
```

6. Clean everything!
```
sudo docker image rm motor --force
sudo apt autoremove docker.io
```

Enjoy!

