---
title: "Deployment is a Dirty Word"
date: "2022-11-18"
slug: "deployment-is-a-dirty-word"
category: "devops"
tag: "windows server"
summary: "An article providing an overview of a deployment strategy I found at work for my coding projects"
draft: false
edited: false
---

I’m a Principal in the Engineering firm I work for. Each Principal has an "extra thing" they are responsible for in running the business, mine is IT. I act as a specialist in the software and systems we need to conduct our business. I have an IT company which supports me in the day-to-day grind, but I maintain the skills to step in and take care of things if needed. I’ve been around computers all my life and I love information technology, so having this outlet is something I’ve truly been thankful for.

As I expressed in my Rust articles, I recently was bitten by the bug to re-learn to code. I say re-learn because fundamentally, I know how. I’m just not up on modern techniques and systems in this internet era. I coded in Turbo Pascal back in the day and took many computer science classes in my engineering education. I remember coding, from scratch, a Turbo Pascal program to display a bezier curve in 3D, as one of my projects. The very next year, the class after me started using AutoCAD. To this day I’m choked about that. Anyway, I digress…

I always do my coding projects with the ultimate goal of deploying them. I just don’t want to create things and have them languish on my computer, never to see the light of day. In the context of my day job, I’ve taken it upon myself to code simple systems which benefit my staff and me. For example, I made a very simple intranet site, created with React and backed by a Microsoft SQL Server database. My office runs a Microsoft-centric environment, which means Internet Information Services on Windows Server 2019. I’ve learned how to supplement it with other services to get my little coding projects into production.

Deployment seems to be somewhat of a dirty word. There’s tons of support on coding and how to code, in any language of your choice, but little to nothing on how to get software into deployment within a Windows Server environment. Nothing I create touches the internet, my skills are not there. However, it’s relatively safe to make a website or something that is consumed from the desktops and browsers of my staff.

I have to be an army of one in my coding efforts, so I choose tools that have lots of support and are inherently safe. My languages of choice for server-side projects are Rust and C#. I love how straightforward it is to spin up a minimal API with C# and connect it to an MS SQL Server database. I’ve created a searchable database of our specialized solutions (alternative solutions, in my vocabulary) and hooked it up to a C# API which sits in front. Admin staff use Access to enter new information into the database. I chose Access mainly because staff are used to it, but also because my coding skills are still developing and I’ve yet to gain confidence in posting form submissions back to a database.

I’ve started another API/database project in Rust. I love Rust because I know that it’s inherently safe and I’m far less likely to be bitten by my noobie coding skills. The tooling is excellent and I love how the language makes me think. I’m using the Rocket web framework and SQLite as the database. I like Rocket a lot. I find it a gentle introduction to the concepts that need to be mastered in an API project.

Deploying a C# API is a no-brainer in an IIS/Microsoft environment. Rust apps, however, are more involved. Once you build your Windows executable, where do you put it? How do you ensure it stays running after the server reboots? In a Linux environment, there are loads of tools. In Windows, it’s tougher and I’ve had to Google, Google, and Google some more. I tried to use IIS, but there is precious little information on setting it up as a reverse proxy. What little there is seems out of date or just doesn’t work.

Here is the system I’ve settled on:

- [Non-Sucking Service Manager](https://nssm.cc/) (NSSM) for Windows— for running a Rust executable as a service
- [nginx for Windows](https://nginx.org/en/docs/windows.html) — as a reverse proxy

It’s super simple to convert any Rust executable into a Windows service with NSSM. Once it’s installed as a service, it becomes trivial to manage it with the built-in services administration tool. nginx works beautifully on Windows as a reverse proxy to take incoming IP requests to an address of your choice and kick them on to the IP address that the executable listens on.

Deployment is not so much of a dirty, scary word anymore.
