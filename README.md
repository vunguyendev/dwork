# dWork
 
 ## Workflow: 
 - Job owner create a job 
 - Developers or workers have to register into dwork with a small bond.
 - Developers submit a proposal to one or more job at 1 time. Developers's hour estimation must be less than 20% different from job owner
 - Job owner select 1 proposal to process
 - Developers do their work and submit
 - Owner check the result and make a payroll
 
 ## Planning:
 - Developers will have level base on their previous work and total money they received
 - Handle exception case like: workers not do their work, owner not do payroll
 - Handle bonus, tip, maintainance, fix bug, etc
 - Charge services fee

### new
```sh
near call $ID new '{}' --accountId $ID
```
### new_job by requester
```sh
//Register as a requester
near call $ID register '{"requester": true}' --accountId job_creator.testnet --amount 0.5
// Create new job: job_creator.testnet
near call $ID new_task '{"title": "Retweet LNC post", "description": "Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers", "hour_rate": 10000000000, "hour_estimation": 86400, "max_participants": 2}' --accountId job_creator.testnet
```
### register_as_a_worker
```sh
near call $ID register '{"requester": false}' --accountId job_worker.testnet --amount 0.5
```

### submit_proposal
```sh
near call $ID submit_proposal '{"task_id": "job_creator.testnet_81919914", "cover_letter": "I am hungry", "hour_estimation": 3600000000000}' --accountId job_worker.testnet
```

### view_proposals 
```sh 
near view $ID view_proposals '{"task_id": "job_creator.testnet_81920544"}'
```

### select_proposal
```sh
near call $id select_proposal '{"task_id": "job_creator.testnet_81919914", "index": 0}' --accountid job_creator.testnet --amount 1
```

### submit_work
```sh
near call $ID submit_work '{"task_id": "job_creator.testnet_80190186", "url": "https://github.com/vunguyendev/dupwork"}' --accountId job_worker.testnet 
```

### validate_work
```sh
near call $ID validate_work '{"task_id": "job_creator.testnet_80190186"}' --accountId job_creator.testnet
```


## Views 
```sh 
// get available_tasks
near view $ID available_tasks '{"from_index": 0, "limit": 10}'

Response example
[
  {
    task_id: 'job_creator.testnet_81919914',
    owner: 'job_creator.testnet',
    title: 'Retweet LNC post',
    description: 'Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers',
    max_participants: 2,
    hour_rate: '1000000000',
    hour_estimation: 86400000000,
    proposals: [],
    status: { type: 'ReadyForApply' }
  },
  {
    task_id: 'job_creator.testnet_81920544',
    owner: 'job_creator.testnet',
    title: 'Retweet LNC post',
    description: 'Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers',
    max_participants: 2,
    hour_rate: '1000000000',
    hour_estimation: 86400000000,
    proposals: [],
    status: { type: 'ReadyForApply' }
  }
]

//get user info
near view $ID user_info '{"account_id": "job_creator.testnet"}'

Response example
{
  account_id: 'job_creator.testnet',
  user_type: { type: 'Requester', total_stake: '0', current_requests: 0 },
  completed_jobs: []
}
```


