#!/bin/bash
set -e

attrib=1
color=35

# Define function
Help ()
{
   # Display Help
   echo "Usage:"
   echo "   Syntax:"
   echo $'\t'"scriptTemplate [flag <action: string>]"
   echo "\n   Flag:"
   echo $'\t'"-c | --community    Community contract."
   echo $'\t'"-a | --admin        Admin contract."
   echo $'\t'"-n | --nft          NFT contract."
   echo "\n   Action:"
   echo $'\t'"b | build           Build specific contract."
   echo $'\t'"o | override        Deploy specific contract with old contract id."
   echo $'\t'"n | new             Deploy specific contract with new contract id."
   echo
   exit
}

Highlight_print_line ()
{
  printf %b "\033[$attrib;${color}m|| => $@\033[m\n"
}

build=0
deploy=0
new=0

action="Build"
action_index=0
while getopts :b-:o-:n-: flag
do
  case "${flag}" in
    -)
      case "${OPTARG}" in
        build) 
          ;;
        override)
          action="Override"
          action_index=1
          ;;
        new)
          action="New"
          action_index=2
          ;;
        *)
          Help 
          exit
          ;;
      esac
      ;;
    b) ;;
    o)
      action="Override"
      action_index=1
      ;;
    n)
      action="New"
      action_index=2
      ;;
    *) ;;
  esac
  shift $(($optind + 1))
done

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p res
cp target/wasm32-unknown-unknown/release/*.wasm res/contract.wasm

wait

if [ $action_index -eq 2 ]
then
  directory=lastest_neardev
  if test -d "$directory"; then
    echo "Removing last version"
    rm -rf ./lastest_neardev
  fi
  mv ./neardev ./lastest_neardev
fi

if [ $action_index -gt 0 ]
then
  near dev-deploy res/contract.wasm
fi

wait

ID=$(<neardev/dev-account)

if [ $action_index -eq 2 ]
then
  near call $ID new '{}' --account-id nttin.testnet
fi
printf %b "\033[$attrib;${color}m|| --------------------------------------------\033[m\n"
Highlight_print_line "$action, contract id: $ID"
printf %b "\033[$attrib;${color}m|| --------------------------------------------\033[m\n"

