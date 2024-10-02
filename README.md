<a id="readme-top"></a>

<div align="center">
  <h2 align="center">md5</h3>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About the project</a>
    </li>
    <li>
      <a href="#getting-started">Getting started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#how-to-run-it">How to run it</a></li>
      </ul>
    </li>
    <!--<li><a href="#roadmap">Roadmap</a></li>-->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About the project

![product-screenshot](./screenshots/md5.PNG)

This was the second project on my personal 10-day [hackathon](https://github.com/Rubidium7/hackathon) to learn about cryptography, and coding in rust, and to deepen my knowledge about network protocols and TLS/SSL.

Basically this is just the md5 algorithm implemented in Rust. You give it an input string and it outputs a 128-bit hash.

And btw this is my first ever project in rust, so don't @ me if there's some unconventional handlings of things, it's all part of the learning process :D



<!-- GETTING STARTED -->
## Getting started

This is how you might run this program locally.
Do note that it was originally made for a unix system, so your mileage may vary depending on your machine. 

### Prerequisites

As this program is coded in rust, you need to have rust and cargo installed

### How to run it

The program takes input as either a string, a file or reads it from the stdin

```
git clone https://github.com/Rubidium7/md5.git
cd md5
cargo build
./target/debug/md5 -s <string>
  or
./target/debug/md5 <filename>
  or
./target/debug/md5 (<= will read stdin) 
```

<!-- ROADMAP -->
<!--## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature -->


<p align="right">(<a href="#readme-top">back to top</a>)</p>
