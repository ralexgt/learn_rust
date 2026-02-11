-> Commonly used styles, patterns and guidelines, generally agreed upon by the community

# Use borrowed values for arguments
-> Deref coercion will be used for the indirection when possible, making the code more flexible
-> For example, can use `&str` instead of `&String` or `&[T]` instead of `&Vec<T>`