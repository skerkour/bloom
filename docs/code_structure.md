# Code structure

## bloom

The `bloom` folder is the backend. It's a Rust multi crates project.

Crates are splitted according to domain logic.

`bloom` contains the CLI and final binary to run Bloom.

`kernel` contains everything related to user, groups and billing management.

`http_server` contains the plumbing to convert API requests to good data structure to call the differents domain service.

`worker` contains the plumbing to run jobs in the context of background workers.

`scheduler` contains the logic to schedule bacnkground jobs.


## bloom.js

The `bloom.js` project contains all the code to load other modules such as `chatbox.js` and send analytics events to backend.


## chatbox.js

The `chatbox.js` project contains all the code to display the customer support chatbox embedded on websites.
It is loaded by `bloom.js`.
