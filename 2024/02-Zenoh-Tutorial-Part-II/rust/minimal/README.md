# Zenoh Minimal Runtime

This example shows hot to build and configure Zenoh to run with a minimal runtime. 

## How To Run
When running the examples pass the configuration file which limits the 
number of threads used by the transport to one.

````
transport: {
  link: {
    tx: {
      threads: 1
    }
  }
}
````

When running also set the environment variable `ASYNC_STD_THREAD_COUNT` to `1`. 
Thus run as follows to minimise the number of threads:

`$ ASYNC_STD_THREAD_COUNT=1 target/debug/m_sub config.json5`  

