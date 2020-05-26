all: cross

cross:
	GOOS=linux GOARCH=arm go build -o web_control

test:
	go build -o web_control

clean:
	rm  web_control
