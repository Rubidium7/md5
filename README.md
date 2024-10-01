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

This was the second project on my personal 10-day hackathon to learn about cryptography, and coding in rust, and to deepen my knowledge about network protocols and TLS/SSL.

I wanted to learn Rust and as I'm very much a learn-by-doing sort of fellow, I decided to just jump right away into writing a functional program. 
I did read some bit of [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch00-00-introduction.html) to get started, but after that I just started writing and learning on the fly.

### Some of my takeaway points from that process
- the rust compiler's attitude did take a bit to get used to, but in the end I do really appreciate the fact that after getting the program to compile you can really trust that it won't suddenly crash and burn (looking at you gcc)
- a lot of the rust methods felt really natural coming from c++ and python (yes, i really didn't expect either it would have so many similarities with python)
- the most difficult part about this project ended up being trying to define an error type and also simutaniously using predefined error types. I didn't expect it would ~~be so hard~~ lead to such a fruitful road of learning about closures, boxes, results etc.
- it did make me think about the less talked about potential vulnerabilities of languages like C, for example accepting numbers to overflow and having a NULL-type



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
