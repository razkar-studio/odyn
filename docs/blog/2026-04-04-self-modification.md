---
slug: when-self-modification
title: When Your App Learns to Touch Itself
authors: razkar
tags: [cli, app, self-modification, projectception]
---

There is a moment, if you work on software long enough, where something clicks into place and feels genuinely strange. Not bad strange. More like the first time you realized a mirror is showing you a reversed version of your face and you spent a few seconds unable to look away.

That moment, for a lot of developers, is the first time they watch their application do something to itself.

Not to a user's data. Not to an external service. To its own running body.

---

## The git fetch Moment

The most common version of this goes something like: you are setting up a deployment script, and you add a line where the service calls `git fetch` before starting. The application reaches out to the origin, compares its own current state against the remote, and decides whether to pull and restart. You run it. You watch the output. The SHA updates. The process restarts with a newer version of its own code.

You sit there for a second just staring at the terminal.

What you have just built is a system that asks, in functional terms, whether it is still current. Whether it is still the right version of itself. And then it acts on the answer.

That is a very different thing from reading a config file or querying a database. Those operations treat the world as external. This one turns inward.

---

## Projectception: When the Tool Clones Itself

And then there is the version of this that stops people cold the first time they realize it works.

`git` is open source. Its source code lives in a repository, like any other project. Which means you can run:

```sh
git clone https://github.com/git/git
```

And `git` will go out, contact a remote server, download the source code for `git`, and store it on your machine. The tool is fetching the instructions for how to build itself. You used the thing to get the thing.

There is no trick here, no special case, nothing unusual about how it works technically. Git does not know or care that it is cloning its own source code. It just follows the same protocol it always does. But that is exactly what makes it feel so odd. The mundaneness of the mechanism does not cancel out the strangeness of what is happening. A hammer cannot be used to forge the steel that made it. A compiler, famously, can compile itself, and the first people who made that work were reportedly unsettled by it too.

This specific flavor of self-reference has a name in developer circles: *Projectception*. The project, within itself, contains instructions for rebuilding the project. Pull the thread long enough and you get something that loops back on itself in a way that is hard to look at directly.

My project, Odyn, the project this exact website is made for, is also no exception of Odynception. Odyn is open-source, on Codeberg. Which means even if Odyn is not an Odin library, you can still run:

```sh
odyn get razkar/odyn --platform codeberg
```

And it'll fetch Odyn's source code, put them to `odyn_deps/`, add an entry to `Odyn.lock`, and it'll work fine. You can even go into `odyn_deps/odyn/`, build Odyn from source inside, and do the same thing again. Over and over.

---

## Self-Modification Is Everywhere, Once You Notice It

Once you start paying attention, you see this pattern all over infrastructure. Watchtower watches your Docker containers and redeploys them when newer images appear in the registry. Kubernetes operators reconcile the state of a cluster, including the specs for their own pods. A GitHub Actions workflow that bumps a version number and commits it back to the same repository it is running from.

These are all examples of systems that take themselves as the subject of an operation, not just the executor of one.

The line between "a tool that does things" and "a thing that maintains itself" is thin, and crossing it feels different every time. There is something almost biological about it. A cell that can repair its own DNA. A script that can upgrade its own dependencies. You are not writing code anymore. You are writing behavior.

---

## Why It Feels Odd

Part of what makes this feeling hard to shake is that we are trained, as programmers, to think of a program as a static artifact. You write it, you compile it, you run it. The code is the author; the process is the execution. Those two things are supposed to stay separate.

Self-referential systems break that assumption. When a running process modifies its own source, updates its own config, or decides to restart itself with different arguments, the author and the execution are no longer cleanly separated. The process has become, in a small way, a participant in its own authorship.

Douglas Hofstadter spent a whole book on this idea. Strange loops, he called them. A system that, through a sequence of steps, ends up back at a level where it can reference itself. He was mostly writing about consciousness and Godel's incompleteness theorems, but the feeling transfers. When you watch a process that watches itself, your brain registers something similar to what it registers when you point a camera at a monitor showing the camera's feed. It recurses. It loops.

---

## Practical Weirdness

Beyond the philosophy, there are real engineering consequences to systems that act on themselves, and most of them are worth thinking about before you are staring at a production incident at 2am.

Self-modifying systems can get into states that are very hard to reason about. If a deployment script has a bug in the part that updates itself, you may end up with a broken updater that cannot update itself out of its own broken state. You have to reach in from outside, which is always a little humbling.

There is also the question of atomicity. When a process modifies its own config and then reads that config, you need to be careful about the ordering. The same concerns show up in database migrations that run as part of the application startup, another extremely common pattern, and one that bites people constantly.

And then there is the human factor. Automated self-updating systems are powerful, but they can also be alarming to work with, especially on a team where not everyone set them up. Someone wakes up to find the application is running a version from three hours ago because a scheduled job auto-reverted a failed deploy. That is correct behavior. It is also going to cause some confusion before the coffee kicks in.

---

## The Part That Stays With You

Despite all of that, there is something worth sitting with in the experience of watching a system manage itself. It represents a shift in how you think about software. A program that can only do what you explicitly command it to do in the moment is a tool. A program that observes its own state and responds to what it finds there is something closer to an agent.

Most production systems live somewhere on that spectrum. The further toward the "agent" end they move, the more interesting and the more unsettling they get to work with.

That first `git fetch` in a deployment script is, for a lot of developers, the first step in that direction. It is a small thing. Probably thirty seconds of work, tucked into a bash script somewhere. But you watch the output scroll by, and for just a moment, the app is not doing what you told it to do. It is doing what it determined it needed to do.

That is a different kind of software. And once you have written some of it, you start seeing the possibilities everywhere.
