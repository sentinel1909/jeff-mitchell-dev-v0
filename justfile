# just commands for the jeff-mitchell-dev repo

# use PowerShell
set shell := ["powershell.exe", "-c"]

# dev loop
dev:
  cargo watch -x clippy -x fmt -qcx 'shuttle run'

# run locally
local:
  cargo shuttle run

# restart container on Shuttle and deploy
deploy:
  cargo shuttle project restart
  cargo shuttle deploy
  