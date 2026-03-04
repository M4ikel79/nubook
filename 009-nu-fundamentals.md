# Nu Fundamentals

This chapter explains some of the fundamentals of the Nushell programming language. After going through it, you should have an idea how to write simple Nushell programs.

Nushell has a rich type system. You will find typical data types such as strings or integers and less typical data types, such as cell paths. Furthermore, one of the defining features of Nushell is the notion of *structured data* which means that you can organize types into collections: lists, records, or tables. Contrary to the traditional Unix approach where commands communicate via plain text, Nushell commands communicate via these data types. All of the above is explained in [Types of Data](/book/types_of_data.html).

[Loading Data](/book/loading_data.html) explains how to read common data formats, such as JSON, into *structured data*. This includes our own "NUON" data format.

Just like Unix shells, Nushell commands can be composed into [pipelines](/book/pipelines.html) to pass and modify a stream of data.

Some data types have interesting features that deserve their own sections: [strings](/book/working_with_strings.html), [lists](/book/working_with_lists.html), and [tables](/book/working_with_tables.html). Apart from explaining the features, these sections also show how to do some common operations, such as composing strings or updating values in a list.

Finally, [Command Reference](/commands/) lists all the built-in commands with brief descriptions. Note that you can also access this info from within Nushell using the `help` command.
