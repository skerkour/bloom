# Design


## Database

Each table must start with its domain name and an underscore (`_`). For example: `kernel_`, `analytics_`, `inbox_`...



## Code organization

### `webapp`

### `libs`


### Other folders

All other folders are parts of the Bloom server / CLI



# development

```
$ docker run --name bloom_db -p 5432:5432 -e POSTGRES_USER=bloom -e POSTGRES_PASSWORD=mysecretpassword -d postgres:12
```


## Debugging


### Inspecting the number of threads for the program

```
$ NUM=`ps M <pid> | wc -l` && echo $((NUM-1))
```
