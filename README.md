# ims
Inventory management system project written in Rust

Full stack system in rust to manage company systems.

Goal of this project exists out of 2 parts. The API and the Application.
All data will be stored using databases and the API will be used to communicate between the application and the databases.



## API

The API will be written in Rust using the actix-web framework. The API will 
be used to communicate between the application and the databases. 
The API will be used to create, read, update and delete data from the databases.

### extra futures to implement
- [ ] ban ip's after x amount of failed login attempts
