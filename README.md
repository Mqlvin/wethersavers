
At current, generic handling is present in both the frontend and backend API handlers for usage.
This uses a generic object:
```ts
{
    success: bool,
    error_message: Option<String>,
    data: Option<Any> // {...}
}
```
to wrap all web-requests between the front and back end.

Alternatively, [this commit](https://github.com/Mqlvin/rust-svelte-webapp/tree/a7dc798e97fa40a25ef797d46298261776f52536) can be cloned to use the pre-api handling template.
