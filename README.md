# Introduction

Show HN: Bloom - If Basecamp, Intercom and Zapier had an Open Source child (baby)
(hubspot, intercom)

Breaking changes may happen anytime without prior notics until 1.0. Self host at your own risks


# Supported stacks

* Go
* JS/TS
* Flutter

# Install

```sh
$ cargo install --git https://github.com/bloom42/bloom.git bloom
```

# Webapp

## Sidebar

* Project overview
* Bots
  * Dashboard -> last history + chiffres
  * Bots
  * history
* Growth
  * Contacts
  * email campaigns
  * Analytics
  * Forms
* Community
  * Blog
  * Forums / discussions
* Support
  * Inboxes (livechat, email)
* Monitor
  * Status page
  * error tracking
* Settings
  * General


explore: groups, users, projects, bots


# Server

## Architecture

The architecture of the server is layered:

### Infrastructure (Delivery & 3rd party services)

HTTP, SSH, Stripe...


### API

GraphQL, GitHTTP...

In the `api` folder.

### Application services

The is where our business logic lives.

They are in the `domain/[xxx]/service` folders.


### Repositories

This is our abstraction over data storage and access.

They are in the `domain/[xxx]/repository` folders.


## Folders

`driver`: contains interfaces and implementations for external services like email and caching

`api`: contains the implementation of the differents apis (graphql...)

`app`: contains the logic to have the other components running as a program

`db`: contains interfaces and implementation to interact with a database

`domain`: contains all the data structures and interfaces to modelize the domain, organized byt its bounded contextes.

`errors`: contains the error types of the application

`http`: contains the implementation of the http delivery layer
