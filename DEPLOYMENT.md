# Deployment Tips

Looking to run your own copy of this app? These tips may help you get started.

## Docker Images

Each component and minigame implemented in this repository has a Docker image published on
this GitHub repository. Those images are public and may be used to run your own copy of the
server. In fact, the [official instance][cameldridge] of this app is currently run using 
those images via `docker-compose` on the server. You can refer to the `docker-compose.yml`
in this repository as a baseline.

[cameldridge]: https://party.cameldridge.com

One point to be aware of, however, is that there is only a `latest` version, and no other
versions are kept. That `latest` version will correspond to the current commit of the
[release](https://github.com/foxfriends/minigames/tree/release) branch, which updates
at an inconsistent interval. At this time, there is no mechanism by which you can keep
that up to date, other than to check occasionally and pull new versions.

## Adjusting minigame builds

The official images of minigames are built without knowledge of being hosted at any
particular location. As these tend to be frontend web applications, where environment
variables are only accessed at compile time, you will need to extend the base image to
set those variables appropriately for your deployment. These images are produced with
all source code and development tooling still included, so you can simply set the 
relevant environment variables for that application and then `RUN` its build script.
