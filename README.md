# hololive-archiver-worker
A worker program which downloads the first stream from the queue

The worked will look for a stream in the queue and download it.
Once the stream is downloaded, it will be moved to the location specified in the job.
Then the job will be set to finished, and the worker will look for a new job.
