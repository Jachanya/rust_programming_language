### Under the hood: Executing Features and Tasks

Features is nothing more than a type. What this notes try to explain is the underlying structure of features and asynchronous tasks.

## Feature Trait

Trait is a feature that lets you describe generalization or properties that can be exhibited by other types. A Feature is an async compt that can produce a value.

- Features can be advanced by calling the poll method, this way making the feature move closer to completion as much as possible.
- The variants of enum Poll::Ready(T) is return if the feature is completed otherwise Poll::Pending is returned. This then arrange for the wake function to be called when the feature is ready to make progress.
- When the function wake is called, the feature calls poll again to make progress.
- Wake is what allows to know when a feature is ready to make progress.
- Say we want to read from a file stream and we perform a read operation, but before the file is ready to send the stream of data to the buffer the thread executing the read operation is blocked and not doing anything significant. Using feature, we can configure the wake function to alert us when there is file available to read in the buffer, but before file is ready we yield the cpu to the caller of the async function.