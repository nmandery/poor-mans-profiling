# poor mans profiling

Well, it is just a span, which prints its metadata to stdout when it is dropped. The metadata includes:

* timestamp of start
* timestamp of end
* an optional message
* a user-defined serializable value.

The emitted messages look like this:

```json
{"t_start":"2021-05-06T17:57:33.985703608Z","t_end":"2021-05-06T17:57:35.985855186Z","message":"Sleeping","data":{"n_secs":2}}
```

This project is nothing more than a quick hack which may help in some cases to gain insights. It is in no way a real profiling tool.

## License
MIT
