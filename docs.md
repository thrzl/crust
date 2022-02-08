<!--
*** from https://github.com/othneildrew/best-readme-template
-->

<!-- project logo -->
<br />
<p align="center">
  <a href="crust.terabyteis.me">
    <img src="assets/book.png" alt="vertigo project logo" width="80">
  </a>

  <h3 align="center"><b>docs</b></h3>

  <p align="center">
    a quick getting started guide :)
    <br />
  </p>
</p>



<!-- table of contents -->



<!-- about the project -->

base url: https://api.crust.terabyteis.me \
parameter type: path params

### **get** /user/{name}

example:

(real request) a request to `https://api.crust.terabyteis.me/user/thrizzzle` will return:
```json
HTTP/1.1 200 OK
content-length: 62
content-type: text/plain; charset=utf-8
date: Tue, 08 Feb 2022 01:18:44 GMT

{
    "name": "thrizzzle",
    "uuid": "728a24c109534636a54c149f7ef627d3"
}
```

### **get** /hello

example:

a request to `https://api.crust.terabyteis.me/hello` will return:
```html
HTTP/1.1 200 OK
content-length: 270
content-type: text/html; charset=utf-8
date: Tue, 08 Feb 2022 01:17:43 GMT
server: railway
x-envoy-upstream-service-time: 4

<iframe width="100%" height="100%" src="https://www.youtube-nocookie.com/embed/Yw6u6YkTgQ4?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
```
##### i know, not much atm... more will be coming soon

###### **thrizzle. 2022**
