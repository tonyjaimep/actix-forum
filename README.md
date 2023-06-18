# Actix Forum

The purpose of this project is to teach myself Rust. It uses actix-web and
diesel.

This is a forum backend system that consists of forums, threads and posts.

## Running the backend

You will need Docker to run the server

Start the server and database by running
```sh
make start
```


It will take some time for the `actix-web` server to spin up, you can use
`docker compose logs actix-forum -f` to monitor the server.

```
make migrate
```
