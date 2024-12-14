- <https://codingchallenges.fyi/challenges/challenge-webserver>

- https://doc.rust-lang.org/book/ch20-01-single-threaded.html

# Instructions

## Step 2

In this step your goal is to server your first HTML document. When the request is for a valid path, your web server should return the document at that path and the HTTP status code 200 as in step 1.

So first, lets create a simple HTML test page, something like this:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Simple Web Page</title>
  </head>
  <body>
    <h1>Test Web Page</h1>
    <p>My web server served this page!</p>
  </body>
</html>
```

Save this as index.html in your project, perhaps in a www directory. Then change your server to use the path we extracted from the HTTP request. Open the file at that path and return it’s contents after the HTTP header for success (HTTP/1.1 200 OK\r\n\r\n).

By convention a request for / usually serves up the file index.html, we’ll follow that, so a request to http://localhost/ or http://localhost/index.html would return the same document.

So when you test you should now see:

```bash
% curl -i http://localhost/
HTTP/1.1 200 OK

<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Simple Web Page</title>
  </head>
  <body>
    <h1>Test Web Page</h1>
    <p>My web server served this page!</p>
  </body>
</html>
```

or:

```bash
% curl -i http://localhost/index.html
HTTP/1.1 200 OK

<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Simple Web Page</title>
  </head>
  <body>
    <h1>Test Web Page</h1>
    <p>My web server served this page!</p>
  </body>
</html>
```

Assuming you used the example HTML from above.

When the request is for an invalid path, you server should return the status code 404 and a suitable error message, the usual is Not Found.

```bash
% curl -i http://localhost/invalid.html
HTTP/1.1 400 Not Found
```

Once you’ve got that working, pause for a moment and think about the security risks you might have introduced in your simple server.
