// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        IOError(::std::io::Error);
        PostgresError(::postgres::Error);
        Utf8Error(::std::string::FromUtf8Error);
    }
}