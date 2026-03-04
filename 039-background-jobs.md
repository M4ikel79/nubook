# Background Jobs

Nushell has experimental support for thread-based background jobs.

## Spawning Jobs

Use `job spawn` to start a background job:
```nu
job spawn { sleep 10sec; 'done' | save result.txt }
# => 1  (job ID)
```

The job runs in a background thread, returns immediately.

## Listing and Killing Jobs

```nu
job list  # Show active jobs
job kill $id  # Kill a job
```

## Job Suspension (Unix)

Press Ctrl+Z to suspend a running external command:
```nu
vim
# => Job 1 is frozen (Ctrl+Z pressed)

job list
# Shows frozen job

job unfreeze  # Bring back to foreground
alias fg = job unfreeze  # Like Unix fg
job unfreeze 1  # Unfreeze specific job
```

## Communication Between Jobs

Send data to/from jobs:
```nu
let jobId = job spawn { job recv | save data.txt }
'hello' | job send $jobId
```

Main thread has ID 0:
```nu
job spawn {
    'from background' | job send 0
}
job recv
```

## Exit Behavior

Jobs are threads, not processes. They terminate when shell exits. Nushell warns if running background jobs when using `exit`.
