# RustOS
Diving into the bowels of low-level programming with Rust & Raspberry Pi hoping to learn how things work just a little bit better.

This project is based on a course originally created by Sergio Benitez at Stanford for his CS140e class.  This particular version has been updated and expanded by the staff at Georgia Tech for their own OS course which is modeled after Benitez's.  

[Benitez's Original CS140e Course](https://cs140e.sergio.bz/)  
[Georgia Tech's 3210 Version](https://tc.gts3.org/cs3210/2020/spring/index.html)
- Georgia Tech adds Labs 4 & 5.
### Labs
<details>
    <summary>Lab 1: Bootstrapping Pi</summary>

- [x] Phase 0: Preflight Check
- [x] Phase 1: Baking Pi
- [x] Phase 2: LED There Be Light
- [x] Phase 3: Shining C
- [x] Phase 4: Rusting Away
</details>

<details>
<summary>Lab 2: Shell & Bootloader</summary>

- [x] Phase 1: Oxidation
- [ ] Phase 2: Not a Seashell  
- [ ] Phase 3: Boot 'em up
</details>

<details>
<summary>Lab 3: FAT32 File System</summary>

- [ ] Phase 0: Getting Started
- [ ] Phase 1: Memory Lane 
- [ ] Phase 2: 32-bit Lipids
- [ ] Phase 3: Saddle Up
- [ ] Phase 4: Mo'sh
</details>

<details>
<summary>Lab 4: Preemptive Multitasking</summary>

- [ ] Phase 0: Getting Started
- [ ] Phase 1: ARM and a Leg
- [ ] Phase 2: It's a Process
- [ ] Phase 3: Memory Management Unit
- [ ] Phase 4: Programs In The Disk 
</details>

<details>
<summary>Lab 5: Multicore & Networking</summary>

- [ ] Phase 0: Getting Started
- [ ] Phase 1: Enabling Multicore
- [ ] Phase 2: TCP Networking
- [ ] Phase 3: Echo Server
</details>

&nbsp;
&nbsp;

# Course Info

This repository contains lab assignments for Georgia Tech CS3210 "Design of Operating Systems".
The latest course material is available [here](https://tc.gts3.org/cs3210/2020/spring/index.html).

## Acknowledgement

We built our labs based on the materials originally developed for
[CS140e: An Experimental Course on Operating Systems](https://cs140e.sergio.bz/)
by [Sergio Benitez](https://sergio.bz/).
We have ported it to use newer toolchains such as Rust 2018 edition,
`cargo-xbuild` (instead of `xargo`), and `no_std` Rust with a minimal shim library
(instead of custom built std).
We’ve also developed it further to include topics such as virtual memory management, multicore scheduling, mutex designing, and implementing a networking stack.
