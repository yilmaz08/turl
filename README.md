# tURL
tURL is a [curl](https://github.com/curl/curl) inspired command-line tool to make plain TCP requests.
## Installation
Currently, there are no releases. However, you can still clone the repository and use it.
```
git clone https://github.com/yilmaz08/turl
```
## Usage
```
$ cargo run -- 127.0.0.1:80 --content "GET / HTTP/1.1\r\nHost: 127.0.0.1:80\r\n\r\n"
```
This will send a HTTP request to 127.0.0.1:80 with the specified headers and content.

Alternatively, you can use the recently introduced `HTTP Mode`:
```
$ cargo run -- 127.0.0.1:80 --content "body here" --http GET
```

Response (on a nginx welcome page):
```
HTTP/1.1 200 OK
Server: nginx/1.26.1
Date: Sun, 21 Jul 2024 12:12:06 GMT
Content-Type: text/html
Content-Length: 615
Last-Modified: Mon, 03 Jun 2024 20:30:53 GMT
Connection: keep-alive
ETag: "665e27fd-267"
Accept-Ranges: bytes

<!DOCTYPE html>
<html>
<head>
<title>Welcome to nginx!</title>
<style>
html { color-scheme: light dark; }
body { width: 35em; margin: 0 auto;
font-family: Tahoma, Verdana, Arial, sans-serif; }
</style>
</head>
<body>
<h1>Welcome to nginx!</h1>
<p>If you see this page, the nginx web server is successfully installed and
working. Further configuration is required.</p>

<p>For online documentation and support please refer to
<a href="http://nginx.org/">nginx.org</a>.<br/>
Commercial support is available at
<a href="http://nginx.com/">nginx.com</a>.</p>

<p><em>Thank you for using nginx.</em></p>
</body>
</html>
```
## Contributing
tURL is open-source, and we welcome contributions! If you'd like to make a change or fix a bug, please:

- Open an issue to discuss your proposed changes (unless it is a small change or fix).
- Fork the repository and make needed changes on the forked repository. (If you split changes into different commits, it would be better.)
- Open a pull request to merge your changes into the main tURL repository.

We appreciate any contributions, no matter how small!

